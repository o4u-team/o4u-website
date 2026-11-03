use async_trait::async_trait;
use o4u_domain::{Error, ListQuery, Resource, Result, invalid, validate_registration};
use serde_json::Value;
use std::sync::Arc;

/// Read port. Queries never change state.
#[async_trait]
pub trait ReadRepository: Send + Sync {
    async fn list(&self, resource: Resource, query: ListQuery) -> Result<Value>;
    async fn get(&self, resource: Resource, id: i64) -> Result<Value>;
    async fn assignments(&self, system_id: i64) -> Result<Value>;
    async fn usage(&self, filters: Value) -> Result<Value>;
    async fn app(&self, identifier: &str, numeric: bool) -> Result<Value>;
    async fn system(&self, uuid: &str, app_id: i64) -> Result<Value>;
    async fn access(&self, app_id: i64, endpoint: &str, db: &str) -> Result<Option<String>>;
    async fn permissions(&self, modules: Vec<String>) -> Result<Value>;
    async fn user(&self, id: i64) -> Result<Value>;
    async fn api_key(&self, key: &str) -> Result<()>;
}
/// Write port. Each operation owns its transaction.
#[async_trait]
pub trait WriteRepository: Send + Sync {
    async fn save(&self, resource: Resource, id: Option<i64>, data: Value) -> Result<Value>;
    async fn delete(&self, resource: Resource, id: i64) -> Result<()>;
    async fn sync_apps(&self, system_id: i64, data: Value) -> Result<()>;
    async fn ensure_system(&self, app: Value, endpoint: &str, db: &str) -> Result<Value>;
    async fn log_usage(&self, app: Value, system: Value, data: Value, ip: &str) -> Result<Value>;
    async fn update_device(&self, data: Value, key: &str) -> Result<Value>;
    async fn google_user(&self, email: &str, google_id: &str, avatar: &str) -> Result<Value>;
}
#[async_trait]
pub trait RegistrationGateway: Send + Sync {
    async fn register(&self, data: Value, ip: &str) -> Result<(u16, Value)>;
}
#[derive(Clone)]
pub struct Queries(pub Arc<dyn ReadRepository>);
#[derive(Clone)]
pub struct Commands(pub Arc<dyn WriteRepository>);
impl Commands {
    pub async fn save(&self, resource: Resource, id: Option<i64>, data: Value) -> Result<Value> {
        resource.validate(&data)?;
        self.0.save(resource, id, data).await
    }
    pub async fn sync_apps(&self, id: i64, data: Value) -> Result<()> {
        let ids = data["app_ids"]
            .as_array()
            .ok_or_else(|| invalid("app_ids", "Must be an array"))?;
        let mut unique = std::collections::BTreeSet::new();
        for id in ids {
            let id = id
                .as_i64()
                .filter(|id| *id > 0)
                .ok_or_else(|| invalid("app_ids", "IDs must be positive integers"))?;
            if !unique.insert(id) {
                return Err(invalid("app_ids", "IDs must be distinct"));
            }
        }
        if data.get("app_settings").is_some_and(|v| !v.is_object()) {
            return Err(invalid("app_settings", "Must be an object"));
        }
        self.0.sync_apps(id, data).await
    }
    pub async fn ensure(&self, app: Value, data: Value) -> Result<Value> {
        o4u_domain::required(&data, "endpoint", 1, 255)?;
        o4u_domain::required(&data, "db_name", 1, 255)?;
        let endpoint = o4u_domain::text(&data, "endpoint").trim_end_matches('/');
        if !o4u_domain::http_url(endpoint) {
            return Err(invalid("endpoint", "Must be an HTTP(S) URL"));
        }
        self.0
            .ensure_system(app, endpoint, o4u_domain::text(&data, "db_name"))
            .await
    }
    pub async fn log_usage(
        &self,
        app: Value,
        system: Value,
        data: Value,
        ip: &str,
    ) -> Result<Value> {
        o4u_domain::required(&data, "username", 1, 255)?;
        o4u_domain::required(&data, "device_id", 1, 255)?;
        if data
            .get("device_info")
            .is_some_and(|v| !v.is_null() && !v.is_object())
        {
            return Err(invalid("device_info", "Must be an object"));
        }
        self.0.log_usage(app, system, data, ip).await
    }
}
pub async fn register(
    gateway: &dyn RegistrationGateway,
    mut data: Value,
    ip: &str,
) -> Result<(u16, Value)> {
    validate_registration(&data)?;
    data["email"] = Value::String(o4u_domain::text(&data, "email").to_lowercase());
    gateway.register(data, ip).await
}
pub fn id(value: &Value) -> Result<i64> {
    value["id"].as_i64().ok_or(Error::Internal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Mutex;
    struct Registration {
        calls: Mutex<Vec<Value>>,
    }
    #[async_trait]
    impl RegistrationGateway for Registration {
        async fn register(&self, data: Value, _ip: &str) -> Result<(u16, Value)> {
            self.calls.lock().unwrap().push(data);
            Ok((201, json!({"success":true})))
        }
    }
    #[tokio::test]
    async fn invalid_registration_never_calls_upstream() {
        let gateway = Registration {
            calls: Mutex::default(),
        };
        assert!(
            register(&gateway, json!({"company_name":"A"}), "127.0.0.1")
                .await
                .is_err()
        );
        assert!(gateway.calls.lock().unwrap().is_empty());
    }
    #[tokio::test]
    async fn registration_normalizes_email_before_calling_gateway() {
        let gateway = Registration {
            calls: Mutex::default(),
        };
        let response=register(&gateway,json!({"company_name":"ACME","contact_name":"Nhan","email":" TEST@Example.test ","phone":"0123456789","modules":["inventory"]}),"127.0.0.1").await.unwrap();
        assert_eq!(response.0, 201);
        assert_eq!(
            gateway.calls.lock().unwrap()[0]["email"],
            "test@example.test"
        );
    }
}
