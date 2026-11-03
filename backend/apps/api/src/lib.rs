use axum::{
    Json, Router,
    extract::{ConnectInfo, Path, Query, Request, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use o4u_application::{Commands, Queries, id};
use o4u_domain::{Error, ListQuery, Resource, Version, effective_settings, invalid, text};
use o4u_infrastructure::{
    Config, Postgres, crypto,
    integrations::{Integrations, sign, verify},
    postgres::db_error,
};
use serde_json::{Value, json};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Postgres,
    pub queries: Queries,
    pub commands: Commands,
    pub integrations: Integrations,
}
impl AppState {
    pub fn new(config: Config, pool: sqlx::PgPool) -> Self {
        let config = Arc::new(config);
        let db = Postgres(pool);
        let repo = Arc::new(db.clone());
        Self {
            integrations: Integrations::new(config.clone(), db.0.clone()),
            config,
            db,
            queries: Queries(repo.clone()),
            commands: Commands(repo),
        }
    }
}
pub struct ApiError(pub Error);
impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        Self(e)
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Error::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let mut body = json!({"success":false,"message":self.0.to_string()});
        if let Error::Validation(errors) = self.0 {
            body["errors"] = json!(errors);
        }
        (status, Json(body)).into_response()
    }
}
type ApiResult<T> = Result<T, ApiError>;
pub fn router(state: AppState) -> Router {
    let admin = Router::new()
        .route("/me", get(me))
        .route("/logout", post(logout))
        .route("/app-usage", get(usage))
        .route("/client-systems/{id}/apps", get(assignments))
        .route("/client-systems/{id}/sync-apps", post(sync_apps))
        .route("/{resource}", get(list).post(create))
        .route("/{resource}/{id}", get(show).put(update).delete(delete))
        .route_layer(middleware::from_fn_with_state(state.clone(), admin_guard));
    let origin: HeaderValue = state
        .config
        .frontend_url
        .parse()
        .expect("Valid FRONTEND_URL origin");
    Router::new()
        .route("/health", get(|| async { Json(json!({"status":"ok"})) }))
        .route("/ready", get(ready))
        .route("/api/public/config", get(public_config))
        .route("/auth/google", get(google_redirect))
        .route("/auth/callback", get(google_callback))
        .route("/erp-saas/register", post(register))
        .route("/api/app/version", get(version))
        .route("/api/app-info", get(app_info))
        .route("/api/app/permissions", post(permissions))
        .route("/api/app/check-access", post(check_access))
        .route("/api/app/client-systems/ensure", post(ensure))
        .route("/api/data/encrypt", post(encrypt))
        .route("/api/data/decrypt", post(decrypt))
        .route("/api/user/device", post(device))
        .route("/api/v1/app-usage/log", post(log_usage))
        .route("/api/fcm/send", post(fcm_single))
        .route("/api/v1/fcm/send-multi", post(fcm_multi))
        .route("/api/odoo-barcode/log", post(legacy_log))
        .route("/api/odoo-approval/log", post(legacy_log))
        .nest("/api/admin", admin)
        .layer(
            CorsLayer::new()
                .allow_origin(origin)
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::ACCEPT]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
fn cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get("cookie")?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|s| s.trim().strip_prefix(&format!("{name}=")))
}
fn header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get(name)?
        .to_str()
        .ok()
        .map(str::trim)
        .filter(|s| !s.is_empty())
}
fn set_cookie(s: &AppState, name: &str, value: &str, age: i64) -> String {
    cookie_value(s.config.secure_cookie, None, name, value, age)
}
fn cookie_value(secure: bool, domain: Option<&str>, name: &str, value: &str, age: i64) -> String {
    format!(
        "{name}={value}; Path=/; HttpOnly; SameSite=Lax; Max-Age={age}{}{}",
        if secure { "; Secure" } else { "" },
        domain
            .map(|domain| format!("; Domain={domain}"))
            .unwrap_or_default()
    )
}
fn session_cookies(s: &AppState, value: &str, age: i64) -> ApiResult<HeaderMap> {
    session_cookie_headers(
        s.config.secure_cookie,
        s.config.session_cookie_domain.as_deref(),
        value,
        age,
    )
}
fn session_cookie_headers(
    secure: bool,
    domain: Option<&str>,
    value: &str,
    age: i64,
) -> ApiResult<HeaderMap> {
    let mut headers = HeaderMap::new();
    // Remove the previous host-only session so it cannot shadow the shared cookie.
    if domain.is_some() {
        headers.append(
            "set-cookie",
            cookie_value(secure, None, "o4u_session", "", 0)
                .parse()
                .map_err(|_| Error::Internal)?,
        );
    }
    headers.append(
        "set-cookie",
        cookie_value(secure, domain, "o4u_session", value, age)
            .parse()
            .map_err(|_| Error::Internal)?,
    );
    Ok(headers)
}
async fn current_user(s: &AppState, h: &HeaderMap) -> o4u_domain::Result<Value> {
    let token = cookie(h, "o4u_session").ok_or(Error::Unauthorized("Login required".into()))?;
    let claims = verify(&s.config, token, "session")?;
    let active=sqlx::query("UPDATE sessions SET last_activity=extract(epoch from now())::integer WHERE id=$1 AND last_activity>extract(epoch from now())::integer-28800").bind(format!("o4u:{}",claims.nonce)).execute(&s.db.0).await.map_err(db_error)?.rows_affected();
    if active == 0 {
        return Err(Error::Unauthorized("Session has expired".into()));
    }
    s.queries
        .0
        .user(
            claims
                .sub
                .parse()
                .map_err(|_| Error::Unauthorized("Invalid user".into()))?,
        )
        .await
}
async fn admin_guard(State(s): State<AppState>, mut req: Request, next: Next) -> Response {
    if !matches!(*req.method(), Method::GET | Method::HEAD | Method::OPTIONS)
        && header(req.headers(), "origin") != Some(s.config.frontend_url.as_str())
    {
        return ApiError(Error::Forbidden("Invalid request origin".into())).into_response();
    }
    match current_user(&s, req.headers()).await {
        Ok(user) => {
            req.extensions_mut().insert(user);
            next.run(req).await
        }
        Err(e) => ApiError(e).into_response(),
    }
}
async fn me(axum::Extension(user): axum::Extension<Value>) -> Json<Value> {
    Json(user)
}
async fn logout(State(s): State<AppState>, h: HeaderMap) -> ApiResult<Response> {
    if let Some(token) = cookie(&h, "o4u_session") {
        let claims = verify(&s.config, token, "session")?;
        sqlx::query("DELETE FROM sessions WHERE id=$1")
            .bind(format!("o4u:{}", claims.nonce))
            .execute(&s.db.0)
            .await
            .map_err(db_error)?;
    }
    Ok((session_cookies(&s, "", 0)?, Json(json!({"success":true}))).into_response())
}
async fn ready(State(s): State<AppState>) -> ApiResult<Json<Value>> {
    sqlx::query("SELECT 1")
        .execute(&s.db.0)
        .await
        .map_err(db_error)?;
    Ok(Json(json!({"status":"ok"})))
}
async fn public_config(State(s): State<AppState>) -> Json<Value> {
    Json(
        json!({"turnstileSiteKey":s.config.turnstile_site_key,"turnstileBypass":s.config.turnstile_bypass}),
    )
}
async fn list(
    State(s): State<AppState>,
    Path(resource): Path<Resource>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<Value>> {
    Ok(Json(s.queries.0.list(resource, query).await?))
}
async fn show(
    State(s): State<AppState>,
    Path((resource, id)): Path<(Resource, i64)>,
) -> ApiResult<Json<Value>> {
    Ok(Json(s.queries.0.get(resource, id).await?))
}
async fn create(
    State(s): State<AppState>,
    Path(resource): Path<Resource>,
    Json(data): Json<Value>,
) -> ApiResult<impl IntoResponse> {
    Ok((
        StatusCode::CREATED,
        Json(s.commands.save(resource, None, data).await?),
    ))
}
async fn update(
    State(s): State<AppState>,
    Path((resource, id)): Path<(Resource, i64)>,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    Ok(Json(s.commands.save(resource, Some(id), data).await?))
}
async fn delete(
    State(s): State<AppState>,
    Path((resource, id)): Path<(Resource, i64)>,
) -> ApiResult<StatusCode> {
    s.commands.0.delete(resource, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
async fn assignments(State(s): State<AppState>, Path(id): Path<i64>) -> ApiResult<Json<Value>> {
    Ok(Json(s.queries.0.assignments(id).await?))
}
async fn sync_apps(
    State(s): State<AppState>,
    Path(id): Path<i64>,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    s.commands.sync_apps(id, data).await?;
    Ok(Json(json!({"success":true})))
}
async fn usage(
    State(s): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> ApiResult<Json<Value>> {
    Ok(Json(s.queries.0.usage(json!(query)).await?))
}
async fn google_redirect(State(s): State<AppState>) -> ApiResult<Response> {
    if s.config.google_client_id.is_empty() {
        return Err(Error::Unavailable("Google OAuth is not configured".into()).into());
    }
    let state = sign(&s.config, "oauth", "oauth", 600)?;
    let mut url = o4u_infrastructure::integrations::google_authorize_url();
    url.query_pairs_mut().extend_pairs([
        ("client_id", s.config.google_client_id.as_str()),
        ("redirect_uri", s.config.google_redirect_uri.as_str()),
        ("response_type", "code"),
        ("scope", "openid email profile"),
        ("state", state.as_str()),
    ]);
    Ok((
        [("set-cookie", set_cookie(&s, "o4u_oauth", &state, 600))],
        Redirect::temporary(url.as_str()),
    )
        .into_response())
}
async fn google_callback(
    State(s): State<AppState>,
    h: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let result=async {
        let state=q.get("state").ok_or(Error::Forbidden("Missing OAuth state".into()))?;
        verify(&s.config,state,"oauth")?;
        if cookie(&h,"o4u_oauth")!=Some(state.as_str()){return Err(Error::Forbidden("Invalid OAuth state".into()));}
        let profile=s.integrations.google(q.get("code").ok_or(Error::Unauthorized("Google login cancelled".into()))?).await?;
        let user=s.commands.0.google_user(text(&profile,"email"),text(&profile,"sub"),text(&profile,"picture")).await?;
        let token=sign(&s.config,&id(&user)?.to_string(),"session",28800)?;
        let claims=verify(&s.config,&token,"session")?;
        sqlx::query("INSERT INTO sessions(id,user_id,payload,last_activity) VALUES($1,$2,'o4u-rust',extract(epoch from now())::integer)").bind(format!("o4u:{}",claims.nonce)).bind(id(&user)?).execute(&s.db.0).await.map_err(db_error)?;
        Ok::<_,Error>(token)
    }.await;
    let mut response = match result {
        Ok(token) => (
            session_cookies(&s, &token, 28800)?,
            Redirect::to(&format!("{}/dashboard", s.config.frontend_url)),
        )
            .into_response(),
        Err(error) => {
            tracing::warn!(error=%error,"Google login failed");
            Redirect::to(&format!(
                "{}/login?error=google_login_failed",
                s.config.frontend_url
            ))
            .into_response()
        }
    };
    response.headers_mut().append(
        "set-cookie",
        set_cookie(&s, "o4u_oauth", "", 0)
            .parse()
            .map_err(|_| Error::Internal)?,
    );
    Ok(response)
}
async fn resolve_app(s: &AppState, h: &HeaderMap, numeric: bool) -> ApiResult<Value> {
    let identifier =
        header(h, "x-app-id").ok_or_else(|| ApiError(Error::Forbidden("Forbiden".into())))?;
    if !numeric && uuid::Uuid::parse_str(identifier).is_err() {
        return Err(Error::BadRequest("Invalid credentials".into()).into());
    }
    let identifier = uuid::Uuid::parse_str(identifier)
        .map(|u| u.to_string())
        .unwrap_or(identifier.into());
    match s.queries.0.app(&identifier, numeric).await {
        Ok(app) => Ok(app),
        Err(Error::NotFound(_)) if !numeric => {
            Err(Error::BadRequest("Invalid credentials".into()).into())
        }
        Err(e) => Err(e.into()),
    }
}
async fn resolve_system(s: &AppState, h: &HeaderMap, app: &Value) -> ApiResult<Value> {
    let uuid = header(h, "x-client-system-id").ok_or_else(|| {
        ApiError(invalid(
            "X-Client-System-Id",
            "Missing required headers: X-Client-System-Id, X-App-Id",
        ))
    })?;
    let uuid = uuid::Uuid::parse_str(uuid)
        .map_err(|_| Error::NotFound("Invalid or inactive client system.".into()))?
        .to_string();
    Ok(s.queries.0.system(&uuid, id(app)?).await?)
}
fn version_data(settings: &Value, platform: &str, client: &str) -> Value {
    let minimum = &settings[format!("{platform}_min_version")];
    let current = &settings[format!("{platform}_current_version")];
    let force = Version::parse(client)
        .ok()
        .zip(minimum.as_str().and_then(|s| Version::parse(s).ok()))
        .is_some_and(|(a, b)| a < b);
    json!({"platform":platform,"min_version":minimum,"current_version":current,"store_url":settings[if platform=="android"{"android_store_url"}else{"apple_store_url"}],"force_update":force})
}
async fn version(
    State(s): State<AppState>,
    h: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult<Json<Value>> {
    let app = resolve_app(&s, &h, false).await?;
    let settings = if header(&h, "x-client-system-id").is_some() {
        let system = resolve_system(&s, &h, &app).await?;
        effective_settings(&app, &system["overrides"])
    } else {
        app
    };
    let platform = header(&h, "x-platform")
        .or(q.get("platform").map(String::as_str))
        .unwrap_or("")
        .to_lowercase();
    let client = header(&h, "x-app-version")
        .or(q.get("app_version").map(String::as_str))
        .unwrap_or("");
    let data = if platform.is_empty() {
        json!({"webapp_url":settings["webapp_url"],"android":version_data(&settings,"android",client),"ios":version_data(&settings,"ios",client)})
    } else {
        if !["android", "ios"].contains(&platform.as_str()) {
            return Err(invalid("platform", "Invalid platform. Use android or ios.").into());
        }
        let mut data = version_data(&settings, &platform, client);
        data["webapp_url"] = settings["webapp_url"].clone();
        data
    };
    Ok(Json(json!({"success":true,"data":data})))
}
async fn app_info(State(s): State<AppState>, h: HeaderMap) -> ApiResult<Json<Value>> {
    let app = resolve_app(&s, &h, true).await?;
    let system = resolve_system(&s, &h, &app).await?;
    if system["allow_get_info"] != true {
        return Err(Error::Forbidden("Client system does not allow getting info.".into()).into());
    }
    let mut data = effective_settings(&app, &system["overrides"]);
    for k in ["endpoint", "webapp_endpoint", "db_name"] {
        data[k] = system[k].clone();
    }
    Ok(Json(json!({"success":true,"data":data})))
}
async fn permissions(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    resolve_app(&s, &h, false).await?;
    let modules = data["modules"]
        .as_array()
        .map(|m| {
            m.iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    Ok(Json(
        json!({"success":true,"data":s.queries.0.permissions(modules).await?}),
    ))
}
async fn check_access(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    let app = resolve_app(&s, &h, false).await?;
    o4u_domain::required(&data, "client_domain", 1, 255)?;
    o4u_domain::required(&data, "db_name", 1, 255)?;
    if app["allow_public"] == true {
        return Ok(Json(json!({"status":true,"client_system_uuid":null})));
    }
    let system = s
        .queries
        .0
        .access(
            id(&app)?,
            text(&data, "client_domain"),
            text(&data, "db_name"),
        )
        .await?;
    Ok(Json(match system {
        Some(uuid) => json!({"status":true,"client_system_uuid":uuid}),
        None => json!({"status":false}),
    }))
}
async fn ensure(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    let app = resolve_app(&s, &h, false).await?;
    Ok(Json(
        json!({"success":true,"data":s.commands.ensure(app,data).await?}),
    ))
}
async fn encrypt(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    s.queries
        .0
        .api_key(header(&h, "x-authorization").unwrap_or(""))
        .await?;
    o4u_domain::required(&data, "data_encrypt", 1, 1_000_000)?;
    Ok(Json(
        json!({"success":true,"data":crypto::encrypt(text(&data,"data_encrypt"),&s.config.crypto_key)?}),
    ))
}
async fn decrypt(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    resolve_app(&s, &h, false).await?;
    o4u_domain::required(&data, "data_decrypt", 1, 1_000_000)?;
    Ok(Json(
        match crypto::decrypt(text(&data, "data_decrypt"), &s.config.crypto_key) {
            Ok(v) => json!({"success":true,"data":v}),
            Err(e) => json!({"success":false,"message":e.to_string()}),
        },
    ))
}
async fn device(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    resolve_app(&s, &h, false).await?;
    Ok(Json(
        s.commands
            .0
            .update_device(data, header(&h, "x-authorization").unwrap_or(""))
            .await?,
    ))
}
fn client_ip(s: &AppState, h: &HeaderMap, addr: SocketAddr) -> String {
    if s.config.trusted_proxy_ips.contains(&addr.ip())
        && let Some(ip) = header(h, "x-real-ip").and_then(|v| v.parse::<std::net::IpAddr>().ok())
    {
        return ip.to_string();
    }
    addr.ip().to_string()
}
async fn log_usage(
    State(s): State<AppState>,
    h: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    let app = resolve_app(&s, &h, true).await?;
    let system = resolve_system(&s, &h, &app).await?;
    Ok(Json(
        s.commands
            .log_usage(app, system, data, &client_ip(&s, &h, addr))
            .await?,
    ))
}
async fn register(
    State(s): State<AppState>,
    h: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(data): Json<Value>,
) -> ApiResult<Response> {
    let (status, data) =
        o4u_application::register(&s.integrations, data, &client_ip(&s, &h, addr)).await?;
    Ok((
        StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_GATEWAY),
        Json(data),
    )
        .into_response())
}
async fn queue_fcm(s: &AppState, mut data: Value, multi: bool) -> ApiResult<Json<Value>> {
    for f in ["title", "description"] {
        o4u_domain::required(&data, f, 1, if f == "title" { 255 } else { 65535 })?;
    }
    if !multi {
        o4u_domain::required(&data, "fcm_token", 1, 4096)?;
        data["fcm_tokens"] = json!([data["fcm_token"]]);
    }
    let tokens = data["fcm_tokens"]
        .as_array()
        .ok_or_else(|| invalid("fcm_tokens", "Required"))?;
    let mut unique = std::collections::BTreeSet::new();
    for t in tokens {
        let t = t
            .as_str()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| invalid("fcm_tokens", "Tokens must be nonempty strings"))?;
        unique.insert(t.to_owned());
    }
    if unique.is_empty() || unique.len() > 500 {
        return Err(invalid("fcm_tokens", "Require 1 to 500 distinct tokens").into());
    }
    let count = unique.len();
    data["fcm_tokens"] = json!(unique);
    if let Some(values) = data.get("data").filter(|v| !v.is_null())
        && !values
            .as_object()
            .is_some_and(|m| m.values().all(Value::is_string))
    {
        return Err(invalid("data", "FCM data must contain string values").into());
    }
    s.db.enqueue_fcm(data).await?;
    Ok(Json(if multi {
        json!({"success":true,"message":format!("Notification queued for {count} device(s)."),"token_count":count})
    } else {
        json!({"success":true})
    }))
}
async fn fcm_single(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    s.queries
        .0
        .api_key(header(&h, "x-authorization").unwrap_or(""))
        .await?;
    queue_fcm(&s, data, false).await
}
async fn fcm_multi(
    State(s): State<AppState>,
    h: HeaderMap,
    Json(data): Json<Value>,
) -> ApiResult<Json<Value>> {
    let app = resolve_app(&s, &h, true).await?;
    resolve_system(&s, &h, &app).await?;
    queue_fcm(&s, data, true).await
}
fn redact_diagnostic(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let key = key.to_lowercase();
                if ["password", "secret", "token", "authorization", "api_key"]
                    .iter()
                    .any(|s| key.contains(s))
                {
                    *value = json!("[REDACTED]");
                } else {
                    redact_diagnostic(value);
                }
            }
        }
        Value::Array(values) => {
            for value in values {
                redact_diagnostic(value);
            }
        }
        _ => {}
    }
}
async fn legacy_log(
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    Json(mut data): Json<Value>,
) -> StatusCode {
    redact_diagnostic(&mut data);
    tracing::info!(path=%uri.path(),payload=%data,"Legacy mobile diagnostic received");
    StatusCode::OK
}

#[cfg(test)]
mod cookie_tests {
    use super::{cookie_value, session_cookie_headers};

    #[test]
    fn shared_session_replaces_host_cookie_and_keeps_security_attributes() {
        let headers = session_cookie_headers(true, Some("example.com"), "token", 28800)
            .unwrap_or_else(|_| panic!("Invalid cookie headers"));
        let cookies: Vec<_> = headers
            .get_all("set-cookie")
            .iter()
            .map(|h| h.to_str().unwrap())
            .collect();
        assert_eq!(cookies.len(), 2);
        assert!(cookies[0].contains("o4u_session=;"));
        assert!(cookies[0].contains("Max-Age=0"));
        assert!(!cookies[0].contains("Domain="));
        assert_eq!(
            cookies[1],
            "o4u_session=token; Path=/; HttpOnly; SameSite=Lax; Max-Age=28800; Secure; Domain=example.com"
        );
    }

    #[test]
    fn logout_clears_shared_and_legacy_sessions() {
        let headers = session_cookie_headers(true, Some("example.com"), "", 0)
            .unwrap_or_else(|_| panic!("Invalid cookie headers"));
        for cookie in headers.get_all("set-cookie") {
            let cookie = cookie.to_str().unwrap();
            assert!(cookie.starts_with("o4u_session=;"));
            assert!(cookie.contains("Max-Age=0"));
        }
        assert_eq!(headers.get_all("set-cookie").iter().count(), 2);
    }

    #[test]
    fn default_session_and_oauth_cookies_remain_host_only() {
        let headers = session_cookie_headers(false, None, "token", 28800)
            .unwrap_or_else(|_| panic!("Invalid cookie headers"));
        assert_eq!(headers.get_all("set-cookie").iter().count(), 1);
        let cookie = headers["set-cookie"].to_str().unwrap();
        assert!(!cookie.contains("Domain="));
        assert!(!cookie.contains("Secure"));
        assert_eq!(
            cookie_value(true, None, "o4u_oauth", "state", 600),
            "o4u_oauth=state; Path=/; HttpOnly; SameSite=Lax; Max-Age=600; Secure"
        );
    }
}
