use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use o4u_api::{AppState, router};
use o4u_application::{ReadRepository, WriteRepository, id};
use o4u_domain::{ListQuery, Resource, effective_settings};
use o4u_infrastructure::{
    Config,
    integrations::{sign, verify},
};
use serde_json::{Value, json};
use tower::ServiceExt;
fn config(url: String) -> Config {
    Config {
        trusted_proxy_ips: vec![],
        database_url: url,
        bind: "127.0.0.1:8088".into(),
        frontend_url: "http://localhost:5173".into(),
        session_secret: "only-for-tests-a-long-session-secret-0123456789".into(),
        secure_cookie: false,
        google_client_id: "test-client".into(),
        google_client_secret: "".into(),
        google_redirect_uri: "http://localhost:5173/auth/callback".into(),
        crypto_key: "test-key".into(),
        registration_url: "".into(),
        registration_key: "".into(),
        turnstile_secret: "".into(),
        turnstile_site_key: "".into(),
        turnstile_bypass: false,
        firebase_credentials: "".into(),
    }
}
async fn body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), 2_000_000)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
/// Run only against the disposable database initialized by scripts/test-postgres.sh.
#[tokio::test]
#[ignore = "requires isolated PostgreSQL: scripts/test-postgres.sh"]
async fn old_schema_crud_mobile_api_sessions_and_queue() {
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL required");
    assert!(
        url.ends_with("/o4u_migration_test"),
        "Never run on the existing DB"
    );
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let state = AppState::new(config(url), pool.clone());
    let app = router(state.clone());
    let db = &state.db;
    let client = state
        .commands
        .save(
            Resource::Clients,
            None,
            json!({"name":"ACME","status":"active"}),
        )
        .await
        .unwrap();
    let application=state.commands.save(Resource::Apps,None,json!({"name":"Barcode","status":"active","android_min_version":"1.0.0","android_current_version":"2.0.0","webapp_url":"https://default.test","allow_public":false})).await.unwrap();
    let app_id = id(&application).unwrap();
    let uuid = application["uuid"].as_str().unwrap();
    let system = state
        .commands
        .ensure(
            application.clone(),
            json!({"endpoint":" https://odoo.test/ ","db_name":"main"}),
        )
        .await
        .unwrap();
    assert_eq!(system["created"], true);
    let again = state
        .commands
        .ensure(
            application.clone(),
            json!({"endpoint":"https://ODOO.test","db_name":"main"}),
        )
        .await
        .unwrap();
    assert_eq!(again["created"], false);
    assert_eq!(again["client_system_id"], system["client_system_id"]);
    let system_uuid = system["client_system_id"].as_str().unwrap();
    let resolved = db.system(system_uuid, app_id).await.unwrap();
    let system_id = id(&resolved).unwrap();
    assert_eq!(
        db.access(app_id, "https://odoo.test", "main")
            .await
            .unwrap()
            .as_deref(),
        Some(system_uuid)
    );
    assert!(db.system(system_uuid, 99999).await.is_err());
    let sync = json!({"app_ids":[app_id],"app_settings":{app_id.to_string():{"android_min_version":"1.5.0","webapp_url":"https://override.test"}}});
    state.commands.sync_apps(system_id, sync).await.unwrap();
    let resolved = db.system(system_uuid, app_id).await.unwrap();
    assert_eq!(
        effective_settings(&application, &resolved["overrides"])["webapp_url"],
        "https://override.test"
    );
    let invalid = json!({"app_ids":[app_id],"app_settings":{app_id.to_string():{"android_min_version":"9.0.0"}}});
    assert!(state.commands.sync_apps(system_id, invalid).await.is_err());
    let preserved = db.system(system_uuid, app_id).await.unwrap();
    assert_eq!(preserved["overrides"]["android_min_version"], "1.5.0");
    // Old clients that omit settings must preserve overrides.
    state
        .commands
        .sync_apps(system_id, json!({"app_ids":[app_id]}))
        .await
        .unwrap();
    assert_eq!(
        db.system(system_uuid, app_id).await.unwrap()["overrides"]["webapp_url"],
        "https://override.test"
    );
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/app/version?platform=android&app_version=1.2.0")
                .header("X-App-Id", uuid)
                .header("X-Client-System-Id", system_uuid)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let data = body(response).await;
    assert_eq!(data["data"]["force_update"], true);
    assert_eq!(data["data"]["min_version"], "1.5.0");
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/app-info")
                .header("X-App-Id", app_id.to_string())
                .header("X-Client-System-Id", system_uuid)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    sqlx::query("UPDATE client_systems SET allow_get_info=true WHERE id=$1")
        .bind(system_id)
        .execute(&pool)
        .await
        .unwrap();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/app-info")
                .header("X-App-Id", uuid)
                .header("X-Client-System-Id", system_uuid)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        body(response).await["data"]["webapp_url"],
        "https://override.test"
    );
    for version in ["1.0.0", "1.5.0"] {
        state.commands.log_usage(application.clone(),resolved.clone(),json!({"username":"Nhan","device_id":"test-device","device_info":{"app_version":version}}),"127.0.0.1").await.unwrap();
    }
    let usage = db
        .usage(json!({"app_id":app_id.to_string(),"log_app_id":app_id.to_string()}))
        .await
        .unwrap();
    assert_eq!(usage["appUserDevices"]["total"], 1);
    assert_eq!(usage["connectionLogs"]["total"], 2);
    let search = db
        .list(
            Resource::Clients,
            ListQuery {
                search: Some("' OR 1=1; --".into()),
                sort_field: Some("name; DROP TABLE clients".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(search["total"], 0);
    let module: i64 = sqlx::query_scalar(
        "INSERT INTO module_apps(name,display_name) VALUES('inventory','Inventory') RETURNING id",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO module_app_features(app_id,feature,display_name) VALUES($1,'scan','Scan')",
    )
    .bind(module)
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        db.permissions(vec!["inventory".into()]).await.unwrap()["inventory"]["scan"],
        true
    );
    sqlx::query(
        "INSERT INTO o4u_clients(name,api_key,is_public) VALUES('Test','test-api-key',true)",
    )
    .execute(&pool)
    .await
    .unwrap();
    db.api_key("test-api-key").await.unwrap();
    assert!(db.api_key("wrong").await.is_err());
    let data = json!({"device_id":"legacy-device","device_info":"{}","username":"legacy-user","domain":"https://odoo.test","db":"main"});
    db.update_device(data.clone(), "test-api-key")
        .await
        .unwrap();
    db.update_device(data, "test-api-key").await.unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM client_user_devices")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    let uid:i64=sqlx::query_scalar("INSERT INTO users(name,email,password) VALUES('Nhan','test@example.test',NULL) RETURNING id").fetch_one(&pool).await.unwrap();
    let user = db
        .google_user("test@example.test", "google-id", "https://avatar.test")
        .await
        .unwrap();
    assert_eq!(user["id"], uid);
    assert!(user.get("password").is_none());
    assert!(
        db.google_user("outsider@example.test", "x", "")
            .await
            .is_err()
    );
    let token = sign(&state.config, &uid.to_string(), "session", 28800).unwrap();
    let claims = verify(&state.config, &token, "session").unwrap();
    sqlx::query("INSERT INTO sessions(id,user_id,payload,last_activity) VALUES($1,$2,'o4u-rust',extract(epoch from now())::integer)").bind(format!("o4u:{}",claims.nonce)).bind(uid).execute(&pool).await.unwrap();
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/admin/clients")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/admin/clients")
                .header("Cookie", format!("o4u_session={token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/logout")
                .header("Cookie", format!("o4u_session={token}"))
                .header("Origin", "https://evil.test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/logout")
                .header("Cookie", format!("o4u_session={token}"))
                .header("Origin", "http://localhost:5173")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/admin/me")
                .header("Cookie", format!("o4u_session={token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    db.enqueue_fcm(json!({"title":"test","description":"test","fcm_tokens":["no-live-delivery"]}))
        .await
        .unwrap();
    let queued: i64 = sqlx::query_scalar("SELECT count(*) FROM jobs WHERE queue='o4u-rust-fcm'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(queued, 1);
    state
        .integrations
        .limit("test-only-throttle", 2, 3600)
        .await
        .unwrap();
    state
        .integrations
        .limit("test-only-throttle", 2, 3600)
        .await
        .unwrap();
    assert!(
        state
            .integrations
            .limit("test-only-throttle", 2, 3600)
            .await
            .is_err()
    );
    // Missing test Firebase credentials fail locally, without making network calls.
    for _ in 0..3 {
        sqlx::query("UPDATE jobs SET available_at=0 WHERE queue='o4u-rust-fcm'")
            .execute(&pool)
            .await
            .unwrap();
        assert!(db.process_notification(&state.integrations).await.unwrap());
    }
    let failed: i64 =
        sqlx::query_scalar("SELECT count(*) FROM failed_jobs WHERE queue='o4u-rust-fcm'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(failed, 1);
    state
        .commands
        .sync_apps(system_id, json!({"app_ids":[]}))
        .await
        .unwrap();
    assert!(db.system(system_uuid, app_id).await.is_err());
    db.delete(Resource::Clients, id(&client).unwrap())
        .await
        .unwrap();
}
