use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Validation failed")]
    Validation(BTreeMap<String, Vec<String>>),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unavailable(String),
    #[error("Too many attempts. Please try again later.")]
    RateLimited,
    #[error("Internal server error")]
    Internal,
}
pub fn invalid(field: &str, message: &str) -> Error {
    Error::Validation(BTreeMap::from([(
        field.to_owned(),
        vec![message.to_owned()],
    )]))
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub [u64; 3]);
impl Version {
    pub fn parse(s: &str) -> Result<Self> {
        let parts: Vec<_> = s.split('.').collect();
        if parts.len() != 3
            || parts
                .iter()
                .any(|p| p.is_empty() || !p.bytes().all(|b| b.is_ascii_digit()))
        {
            return Err(invalid(
                "version",
                "Version must have format major.minor.patch",
            ));
        }
        let mut v = [0; 3];
        for (i, p) in parts.iter().enumerate() {
            v[i] = p
                .parse()
                .map_err(|_| invalid("version", "Version number is too large"))?;
        }
        Ok(Self(v))
    }
}
pub const SETTINGS: [&str; 7] = [
    "webapp_url",
    "android_min_version",
    "android_current_version",
    "ios_min_version",
    "ios_current_version",
    "android_store_url",
    "apple_store_url",
];
pub fn effective_settings(defaults: &Value, overrides: &Value) -> Value {
    let mut out = json!({});
    for key in SETTINGS {
        out[key] = overrides
            .get(key)
            .filter(|v| v.as_str().is_some_and(|s| !s.trim().is_empty()))
            .unwrap_or(&defaults[key])
            .clone();
    }
    out
}
pub fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("").trim()
}
pub fn required(value: &Value, key: &str, min: usize, max: usize) -> Result<()> {
    let len = text(value, key).chars().count();
    if len < min || len > max {
        return Err(invalid(
            key,
            &format!("Must contain {min} to {max} characters"),
        ));
    }
    Ok(())
}
pub fn http_url(s: &str) -> bool {
    url::Url::parse(s)
        .is_ok_and(|url| matches!(url.scheme(), "http" | "https") && url.host_str().is_some())
        && !s.chars().any(char::is_whitespace)
}
pub fn validate_settings(v: &Value) -> Result<()> {
    for key in SETTINGS {
        if let Some(raw) = v.get(key) {
            if !raw.is_null() && !raw.is_string() {
                return Err(invalid(key, "Must be a string or null"));
            }
            let s = text(v, key);
            if s.is_empty() {
                continue;
            }
            if key.contains("version") {
                Version::parse(s)
                    .map_err(|_| invalid(key, "Version must have format major.minor.patch"))?;
            } else if !http_url(s) || s.len() > 500 {
                return Err(invalid(
                    key,
                    "Must be a valid HTTP(S) URL, maximum 500 characters",
                ));
            }
        }
    }
    for platform in ["android", "ios"] {
        let min = format!("{platform}_min_version");
        let current = format!("{platform}_current_version");
        if !text(v, &min).is_empty()
            && !text(v, &current).is_empty()
            && Version::parse(text(v, &min))? > Version::parse(text(v, &current))?
        {
            return Err(invalid(
                &min,
                "Minimum version must be less than or equal to current version",
            ));
        }
    }
    Ok(())
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Resource {
    Clients,
    Apps,
    ClientSystems,
}
impl Resource {
    pub fn table(self) -> &'static str {
        match self {
            Self::Clients => "clients",
            Self::Apps => "apps",
            Self::ClientSystems => "client_systems",
        }
    }
    pub fn fields(self) -> Vec<&'static str> {
        match self {
            Self::Clients => vec!["name", "status"],
            Self::Apps => {
                let mut f = vec!["name", "status", "allow_public"];
                f.extend(SETTINGS);
                f
            }
            Self::ClientSystems => vec![
                "name",
                "status",
                "client_id",
                "endpoint",
                "webapp_endpoint",
                "db_name",
                "allow_get_info",
            ],
        }
    }
    pub fn validate(self, v: &Value) -> Result<()> {
        required(v, "name", 1, 255)?;
        let statuses: &[&str] = match self {
            Self::Clients => &["active", "inactive"],
            Self::Apps => &["active", "maintenance", "inactive"],
            Self::ClientSystems => &["active", "inactive", "expired"],
        };
        if !statuses.contains(&text(v, "status")) {
            return Err(invalid("status", "Invalid status"));
        }
        match self {
            Self::Apps => validate_settings(v)?,
            Self::ClientSystems => {
                required(v, "endpoint", 1, 255)?;
                required(v, "db_name", 1, 255)?;
                if !http_url(text(v, "endpoint")) {
                    return Err(invalid("endpoint", "Must be an HTTP(S) URL"));
                }
                let web = text(v, "webapp_endpoint");
                if !web.is_empty() && (!http_url(web) || web.len() > 255) {
                    return Err(invalid("webapp_endpoint", "Must be an HTTP(S) URL"));
                }
                if !v["client_id"].as_i64().is_some_and(|id| id > 0) {
                    return Err(invalid("client_id", "Client is required"));
                }
            }
            _ => {}
        }
        for key in ["allow_public", "allow_get_info"] {
            if v.get(key).is_some_and(|v| !v.is_boolean()) {
                return Err(invalid(key, "Must be a boolean"));
            }
        }
        Ok(())
    }
}
pub fn validate_registration(v: &Value) -> Result<()> {
    required(v, "company_name", 2, 128)?;
    required(v, "contact_name", 1, 128)?;
    required(v, "email", 3, 128)?;
    required(v, "phone", 8, 32)?;
    let email = text(v, "email");
    if !email.contains('@') || email.chars().any(char::is_whitespace) {
        return Err(invalid("email", "Invalid email"));
    }
    let modules = v["modules"]
        .as_array()
        .ok_or_else(|| invalid("modules", "Select one or two modules"))?;
    if modules.is_empty()
        || modules.len() > 2
        || modules.iter().any(|v| {
            !v.as_str()
                .is_some_and(|s| ["sale", "purchase", "inventory", "employee"].contains(&s))
        })
    {
        return Err(invalid("modules", "Select one or two valid modules"));
    }
    if text(v, "notes").len() > 2000 {
        return Err(invalid("notes", "Maximum 2000 characters"));
    }
    Ok(())
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub client_id: Option<i64>,
    pub page: Option<i64>,
    pub sort_field: Option<String>,
    pub sort_direction: Option<String>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn versions_compare_numerically() {
        assert!(Version::parse("1.10.0").unwrap() > Version::parse("1.9.9").unwrap());
        assert!(Version::parse("1.0").is_err());
        assert!(Version::parse("1.0.0-beta").is_err());
    }
    #[test]
    fn blank_override_inherits_default() {
        let settings = effective_settings(
            &json!({"android_min_version":"1.0.0","webapp_url":"https://a.test"}),
            &json!({"android_min_version":"  ","webapp_url":"https://b.test"}),
        );
        assert_eq!(settings["android_min_version"], "1.0.0");
        assert_eq!(settings["webapp_url"], "https://b.test");
    }
    #[test]
    fn reject_invalid_or_reversed_versions() {
        assert!(
            validate_settings(
                &json!({"android_min_version":"2.0.0","android_current_version":"1.0.0"})
            )
            .is_err()
        );
        assert!(validate_settings(&json!({"ios_min_version":"x"})).is_err());
        assert!(validate_settings(&json!({"webapp_url":"javascript:alert(1)"})).is_err());
    }
    #[test]
    fn registration_modules_are_bounded() {
        assert!(validate_registration(&json!({"company_name":"ACME","contact_name":"Nhan","email":"a@b.test","phone":"0123456789","modules":["sale"]})).is_ok());
        assert!(validate_registration(&json!({"modules":["root"]})).is_err());
    }
}

/// Limit the public ERP response to the same fields returned by the legacy controller.
pub fn registration_response(status: u16, result: &Value, email: &str) -> (u16, Value) {
    if status == 201 && result["success"] == true {
        return (
            201,
            json!({"success":true,"message":result["message"].as_str().unwrap_or("Đăng ký thành công."),"code":null,
            "data":{"tenant_url":result["data"]["tenant_url"],"email":result["data"]["email"].as_str().unwrap_or(email),"email_sent":result["data"]["email_sent"].as_bool().unwrap_or(false),"domain":result["data"]["domain"]}}),
        );
    }
    let code = result["code"].as_str().unwrap_or(match status {
        401 => "upstream_unavailable",
        429 => "daily_cap_exceeded",
        _ => "provision_failed",
    });
    let (field, message) = match code {
        "duplicate_email" => (Some("email"), "Email đã được đăng ký."),
        "duplicate_phone" => (Some("phone"), "Số điện thoại đã được đăng ký."),
        "duplicate_company" => (Some("company_name"), "Tên công ty đã được đăng ký."),
        "captcha_failed" => (
            Some("cf-turnstile-response"),
            "Xác minh captcha thất bại. Vui lòng thử lại.",
        ),
        "invalid_modules" => (Some("modules"), "Vui lòng chọn từ 1 đến 2 module hợp lệ."),
        "daily_cap_exceeded" => (
            None,
            "Đã vượt giới hạn đăng ký trong ngày. Vui lòng thử lại sau.",
        ),
        "too_many_attempts" => (
            None,
            "Bạn gửi quá nhiều yêu cầu. Vui lòng thử lại sau vài phút.",
        ),
        "upstream_unavailable" => (None, "Hệ thống đang bận. Vui lòng thử lại sau."),
        "validation_error" => (None, "Thông tin đăng ký không hợp lệ."),
        _ => (
            None,
            "Không thể tạo tài khoản dùng thử. Vui lòng thử lại sau.",
        ),
    };
    let mut out = json!({"success":false,"code":code,"message":message});
    if let Some(field) = field {
        out["errors"] = json!({field:[message]});
    }
    (
        if field.is_some() {
            422
        } else if status >= 400 {
            status
        } else {
            502
        },
        out,
    )
}
#[cfg(test)]
mod registration_tests {
    use super::*;
    #[test]
    fn upstream_credentials_are_never_exposed() {
        let (_, out) = registration_response(
            201,
            &json!({"success":true,"data":{"email":"a@b.test","admin_password":"secret","tenant_url":"https://tenant.test"}}),
            "fallback@example.test",
        );
        assert!(out["data"].get("admin_password").is_none());
        assert_eq!(out["data"]["email_sent"], false);
    }
    #[test]
    fn duplicate_errors_map_to_fields() {
        let (status, out) = registration_response(409, &json!({"code":"duplicate_email"}), "x");
        assert_eq!(status, 422);
        assert_eq!(out["errors"]["email"][0], "Email đã được đăng ký.");
    }
}
