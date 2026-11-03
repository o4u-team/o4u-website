use std::env;
#[derive(Clone)]
pub struct Config {
    pub trusted_proxy_ips: Vec<std::net::IpAddr>,
    pub database_url: String,
    pub bind: String,
    pub frontend_url: String,
    pub session_secret: String,
    pub secure_cookie: bool,
    pub session_cookie_domain: Option<String>,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub crypto_key: String,
    pub registration_url: String,
    pub registration_key: String,
    pub turnstile_secret: String,
    pub turnstile_site_key: String,
    pub turnstile_bypass: bool,
    pub firebase_credentials: String,
}
fn get(key: &str) -> String {
    env::var(key).unwrap_or_default()
}
impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        use sqlx::ConnectOptions;
        let database_url = if get("DATABASE_URL").is_empty() {
            let options = sqlx::postgres::PgConnectOptions::new()
                .host(&get("DB_HOST"))
                .port(get("DB_PORT").parse().unwrap_or(5432))
                .database(&get("DB_DATABASE"))
                .username(&get("DB_USERNAME"))
                .password(&get("DB_PASSWORD"));
            options.to_url_lossy().to_string()
        } else {
            get("DATABASE_URL")
        };
        let secret = get("SESSION_SECRET");
        if secret.len() < 32 {
            return Err("SESSION_SECRET must have at least 32 characters".into());
        }
        let frontend_url = env::var("FRONTEND_URL").unwrap_or("http://localhost:5173".into());
        let secure_cookie = frontend_url.starts_with("https://");
        let origin = reqwest::Url::parse(&frontend_url)?;
        if !matches!(origin.scheme(), "http" | "https")
            || origin.host_str().is_none()
            || origin.path() != "/"
            || origin.query().is_some()
            || origin.fragment().is_some()
            || frontend_url.ends_with('/')
        {
            return Err("FRONTEND_URL must be an HTTP(S) origin without a trailing slash".into());
        }
        let session_cookie_domain = cookie_domain(
            &get("SESSION_COOKIE_DOMAIN"),
            &frontend_url,
            &get("GOOGLE_REDIRECT_URI"),
        )?;
        Ok(Self {
            trusted_proxy_ips: get("TRUSTED_PROXY_IPS")
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            database_url,
            bind: env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8088".into()),
            frontend_url,
            session_secret: secret,
            secure_cookie,
            session_cookie_domain,
            google_client_id: get("GOOGLE_CLIENT_ID"),
            google_client_secret: get("GOOGLE_CLIENT_SECRET"),
            google_redirect_uri: get("GOOGLE_REDIRECT_URI"),
            crypto_key: get("O4U_SECRET_KEY"),
            registration_url: env::var("ODOO_TENANT_REGISTRATION_URL").unwrap_or(format!(
                "{}/api/tenant/public/register",
                get("ODOO_TENANT_BASE_URL").trim_end_matches('/')
            )),
            registration_key: get("ODOO_TENANT_REGISTRATION_KEY"),
            turnstile_secret: get("TURNSTILE_SECRET_KEY"),
            turnstile_site_key: get("TURNSTILE_SITE_KEY"),
            turnstile_bypass: get("TURNSTILE_BYPASS") == "true",
            firebase_credentials: get("FIREBASE_CREDENTIALS"),
        })
    }
}

fn cookie_domain(
    value: &str,
    frontend_url: &str,
    callback_url: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let domain = value.trim().trim_start_matches('.').to_ascii_lowercase();
    if domain.is_empty() {
        return Ok(None);
    }
    if !domain.contains('.')
        || !domain.split('.').all(|label| {
            !label.is_empty()
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'-')
        })
        || domain.parse::<std::net::IpAddr>().is_ok()
    {
        return Err("SESSION_COOKIE_DOMAIN must be a DNS domain".into());
    }
    for url in [frontend_url, callback_url] {
        let url = reqwest::Url::parse(url)?;
        let host = url.host_str().unwrap_or_default();
        if host != domain && !host.ends_with(&format!(".{domain}")) {
            return Err(
                "SESSION_COOKIE_DOMAIN must include both frontend and Google callback hosts".into(),
            );
        }
    }
    Ok(Some(domain))
}

#[cfg(test)]
mod tests {
    use super::cookie_domain;

    #[test]
    fn shared_session_domain_must_cover_both_hosts() {
        let frontend = "https://o4u.example.com";
        let callback = "https://o4u-backend.example.com/auth/callback";
        assert_eq!(cookie_domain("", frontend, callback).unwrap(), None);
        assert_eq!(
            cookie_domain(".example.com", frontend, callback).unwrap(),
            Some("example.com".into())
        );
        for domain in [
            "o4u.example.com",
            "evil.com",
            "ample.com",
            "com",
            "example.com; Secure",
            "127.0.0.1",
        ] {
            assert!(cookie_domain(domain, frontend, callback).is_err());
        }
    }
}
