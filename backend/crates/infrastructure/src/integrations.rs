use crate::Config;
use async_trait::async_trait;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use o4u_application::RegistrationGateway;
use o4u_domain::{Error, Result, invalid, text};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub purpose: String,
    pub nonce: String,
}
pub fn sign(config: &Config, sub: &str, purpose: &str, seconds: i64) -> Result<String> {
    encode(
        &Header::default(),
        &Claims {
            sub: sub.into(),
            purpose: purpose.into(),
            exp: (chrono::Utc::now().timestamp() + seconds) as usize,
            nonce: uuid::Uuid::new_v4().to_string(),
        },
        &EncodingKey::from_secret(config.session_secret.as_bytes()),
    )
    .map_err(|_| Error::Internal)
}
pub fn verify(config: &Config, token: &str, purpose: &str) -> Result<Claims> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.session_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| Error::Unauthorized("Invalid or expired session".into()))?
    .claims;
    if claims.purpose != purpose {
        return Err(Error::Unauthorized("Invalid token purpose".into()));
    }
    Ok(claims)
}
#[derive(Clone)]
pub struct Integrations {
    pub config: Arc<Config>,
    pub http: reqwest::Client,
    pool: sqlx::PgPool,
}
impl Integrations {
    pub fn new(config: Arc<Config>, pool: sqlx::PgPool) -> Self {
        Self {
            config,
            pool,
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("HTTP client"),
        }
    }
    pub async fn limit(&self, key: &str, max: i64, seconds: i32) -> Result<()> {
        use sha2::{Digest, Sha256};
        let digest = Sha256::digest(key.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        // Dedicated keys in the existing Laravel cache table, atomic across API replicas.
        let count:i64=sqlx::query_scalar("INSERT INTO cache AS c(key,value,expiration) VALUES($1,'1',extract(epoch from now())::integer+$2) ON CONFLICT(key) DO UPDATE SET value=CASE WHEN c.expiration<=extract(epoch from now()) THEN '1' ELSE (c.value::bigint+1)::text END,expiration=CASE WHEN c.expiration<=extract(epoch from now()) THEN extract(epoch from now())::integer+$2 ELSE c.expiration END RETURNING value::bigint").bind(format!("o4u-rust-rate:{digest}")).bind(seconds).fetch_one(&self.pool).await.map_err(crate::postgres::db_error)?;
        if count > max {
            return Err(Error::RateLimited);
        }
        Ok(())
    }
    pub async fn google(&self, code: &str) -> Result<Value> {
        let c = &self.config;
        let token: Value = self
            .http
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code),
                ("client_id", c.google_client_id.as_str()),
                ("client_secret", c.google_client_secret.as_str()),
                ("redirect_uri", c.google_redirect_uri.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(upstream)?
            .error_for_status()
            .map_err(upstream)?
            .json()
            .await
            .map_err(upstream)?;
        let user: Value = self
            .http
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(text(&token, "access_token"))
            .send()
            .await
            .map_err(upstream)?
            .error_for_status()
            .map_err(upstream)?
            .json()
            .await
            .map_err(upstream)?;
        if user["email_verified"] != true || text(&user, "email").is_empty() {
            return Err(Error::Unauthorized("Google email is not verified".into()));
        }
        Ok(user)
    }
    pub async fn firebase(&self, data: &Value) -> Result<()> {
        let c = &self.config;
        let bytes = tokio::fs::read(&c.firebase_credentials)
            .await
            .map_err(|_| Error::Unavailable("FIREBASE_CREDENTIALS cannot be read".into()))?;
        let account: Value = serde_json::from_slice(&bytes).map_err(|_| Error::Internal)?;
        let now = chrono::Utc::now().timestamp();
        let claims = json!({"iss":account["client_email"],"scope":"https://www.googleapis.com/auth/firebase.messaging","aud":"https://oauth2.googleapis.com/token","iat":now,"exp":now+3600});
        let assertion = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(text(&account, "private_key").as_bytes())
                .map_err(|_| Error::Internal)?,
        )
        .map_err(|_| Error::Internal)?;
        let token: Value = self
            .http
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &assertion),
            ])
            .send()
            .await
            .map_err(upstream)?
            .error_for_status()
            .map_err(upstream)?
            .json()
            .await
            .map_err(upstream)?;
        let tokens = data["fcm_tokens"]
            .as_array()
            .ok_or_else(|| invalid("fcm_tokens", "Required"))?;
        for chunk in tokens.chunks(20) {
            let mut tasks = tokio::task::JoinSet::new();
            for target in chunk {
                let mut message = json!({"token":target,"notification":{"title":data["title"],"body":data["description"]}});
                if data["data"].is_object() {
                    message["data"] = data["data"].clone();
                }
                if data["data"]["app_id"] == "approval" {
                    message["android"] = json!({"priority":"high"});
                }
                let http = self.http.clone();
                let token = text(&token, "access_token").to_owned();
                let endpoint = format!(
                    "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                    text(&account, "project_id")
                );
                tasks.spawn(async move {
                    let response = http
                        .post(endpoint)
                        .bearer_auth(token)
                        .json(&json!({"message":message}))
                        .send()
                        .await
                        .map_err(upstream)?;
                    if matches!(response.status().as_u16(), 400 | 404) {
                        tracing::warn!(
                            status = response.status().as_u16(),
                            "FCM target rejected; check token or payload"
                        );
                        return Ok(());
                    }
                    response.error_for_status().map_err(upstream)?;
                    Ok::<_, Error>(())
                });
            }
            // Finish all in-flight sends before retrying a partially failed multicast.
            let mut failure = None;
            while let Some(result) = tasks.join_next().await {
                match result {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => failure = Some(e),
                    Err(_) => failure = Some(Error::Internal),
                }
            }
            if let Some(error) = failure {
                return Err(error);
            }
        }
        Ok(())
    }
}
fn upstream(error: reqwest::Error) -> Error {
    tracing::warn!(error=%error,"Upstream request failed");
    Error::Unavailable("Upstream service is unavailable".into())
}
#[async_trait]
impl RegistrationGateway for Integrations {
    async fn register(&self, mut data: Value, ip: &str) -> Result<(u16, Value)> {
        self.limit(&format!("register-ip:{ip}"), 5, 60).await?;
        self.limit(&format!("register-email:{}", text(&data, "email")), 3, 3600)
            .await?;
        let c = &self.config;
        if !c.turnstile_bypass {
            let captcha: Value = self
                .http
                .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
                .timeout(Duration::from_secs(10))
                .form(&[
                    ("secret", c.turnstile_secret.as_str()),
                    ("response", text(&data, "cf-turnstile-response")),
                    ("remoteip", ip),
                ])
                .send()
                .await
                .map_err(upstream)?
                .json()
                .await
                .map_err(upstream)?;
            if captcha["success"] != true {
                return Ok((
                    422,
                    json!({"success":false,"code":"captcha_failed","message":"Vui lòng xác minh captcha.","errors":{"cf-turnstile-response":["Vui lòng xác minh captcha."]}}),
                ));
            }
        }
        if c.registration_key.is_empty() || !c.registration_url.starts_with("http") {
            return Err(Error::Unavailable(
                "ERP registration is not configured".into(),
            ));
        }
        data.as_object_mut()
            .map(|m| m.remove("cf-turnstile-response"));
        let response = self
            .http
            .post(&c.registration_url)
            .header("X-Registration-Key", &c.registration_key)
            .json(&data)
            .send()
            .await
            .map_err(upstream)?;
        let status = response.status().as_u16();
        let result: Value = response.json().await.map_err(upstream)?;
        if status == 201 && result["success"] == true {
            // Best effort notification; credentials never appear in logs.
            let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
            let chat = std::env::var("TELEGRAM_REGISTRATION_CHAT_ID")
                .or_else(|_| std::env::var("TELEGRAM_CHAT_ID"))
                .unwrap_or_default();
            if !token.is_empty() && !chat.is_empty() {
                let message = format!(
                    "ERP SaaS registration\nCompany: {}\nEmail: {}\nURL: {}\nEmail sent: {}",
                    text(&data, "company_name"),
                    text(&data, "email"),
                    text(&result["data"], "tenant_url"),
                    result["data"]["email_sent"]
                );
                let response = self
                    .http
                    .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
                    .timeout(Duration::from_secs(8))
                    .json(&json!({"chat_id":chat,"text":message,"disable_web_page_preview":true}))
                    .send()
                    .await;
                if response.is_err() {
                    tracing::warn!("ERP registration notification failed");
                }
            }
        }
        Ok(o4u_domain::registration_response(
            status,
            &result,
            text(&data, "email"),
        ))
    }
}
pub fn google_authorize_url() -> reqwest::Url {
    reqwest::Url::parse("https://accounts.google.com/o/oauth2/v2/auth").expect("Static Google URL")
}
