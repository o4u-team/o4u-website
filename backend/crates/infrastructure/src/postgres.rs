use async_trait::async_trait;
use o4u_application::{ReadRepository, WriteRepository, id};
use o4u_domain::{
    Error, ListQuery, Resource, Result, SETTINGS, effective_settings, invalid, text,
    validate_settings,
};
use serde_json::{Value, json};
use sqlx::{AssertSqlSafe, PgPool};
use uuid::Uuid;
#[derive(Clone)]
pub struct Postgres(pub PgPool);
pub fn db_error(error: sqlx::Error) -> Error {
    if let sqlx::Error::Database(ref e) = error {
        match e.code().as_deref() {
            Some("23503") => {
                return invalid(
                    "reference",
                    "Referenced record does not exist or is still in use",
                );
            }
            Some("23505") => return invalid("record", "A record with these values already exists"),
            Some("23502" | "23514" | "22P02" | "22001") => {
                return invalid("record", "Invalid field value");
            }
            _ => {}
        }
    }
    tracing::error!(error=%error,"Database operation failed");
    Error::Internal
}
async fn scalar(pool: &PgPool, sql: &'static str) -> Result<Value> {
    sqlx::query_scalar(sql)
        .fetch_one(pool)
        .await
        .map_err(db_error)
}
#[async_trait]
impl ReadRepository for Postgres {
    async fn list(&self, r: Resource, q: ListQuery) -> Result<Value> {
        let page = q.page.unwrap_or(1).clamp(1, 1_000_000);
        let search = format!("%{}%", q.search.unwrap_or_default());
        let status = q.status.unwrap_or_default();
        let client = q.client_id;
        let sort = q
            .sort_field
            .filter(|s| {
                [
                    "id",
                    "name",
                    "uuid",
                    "status",
                    "created_at",
                    "updated_at",
                    "endpoint",
                    "db_name",
                ]
                .contains(&s.as_str())
                    && (r == Resource::ClientSystems
                        || !["endpoint", "db_name"].contains(&s.as_str()))
            })
            .unwrap_or("created_at".into());
        let direction = if q.sort_direction.as_deref() == Some("asc") {
            "ASC"
        } else {
            "DESC"
        };
        let extra = if r == Resource::ClientSystems {
            " OR t.endpoint ILIKE $1 OR t.db_name ILIKE $1"
        } else {
            ""
        };
        let client_clause = if r == Resource::ClientSystems {
            " AND ($3::bigint IS NULL OR t.client_id=$3)"
        } else {
            " AND ($3::bigint IS NULL OR TRUE)"
        };
        let filter = format!(
            "(t.name ILIKE $1 OR t.uuid::text ILIKE $1{extra}) AND ($2='' OR t.status::text=$2){client_clause}"
        );
        let table = r.table();
        let total: i64 = sqlx::query_scalar(AssertSqlSafe(
            format!("SELECT count(*) FROM {table} t WHERE {filter}").as_str(),
        ))
        .bind(&search)
        .bind(&status)
        .bind(client)
        .fetch_one(&self.0)
        .await
        .map_err(db_error)?;
        let row = if r == Resource::ClientSystems {
            "to_jsonb(t) || jsonb_build_object('client',(SELECT to_jsonb(c) FROM clients c WHERE c.id=t.client_id))"
        } else {
            "to_jsonb(t)"
        };
        let data:Vec<Value>=sqlx::query_scalar(AssertSqlSafe(format!("SELECT {row} FROM {table} t WHERE {filter} ORDER BY t.{sort} {direction},t.id {direction} LIMIT 10 OFFSET $4").as_str())).bind(search).bind(status).bind(client).bind((page-1)*10).fetch_all(&self.0).await.map_err(db_error)?;
        Ok(
            json!({"data":data,"total":total,"current_page":page,"per_page":10,"last_page":((total+9)/10).max(1)}),
        )
    }
    async fn get(&self, r: Resource, id: i64) -> Result<Value> {
        sqlx::query_scalar(AssertSqlSafe(
            format!("SELECT to_jsonb(t) FROM {} t WHERE id=$1", r.table()).as_str(),
        ))
        .bind(id)
        .fetch_optional(&self.0)
        .await
        .map_err(db_error)?
        .ok_or(Error::NotFound("Record not found".into()))
    }
    async fn assignments(&self, system_id: i64) -> Result<Value> {
        let system = self.get(Resource::ClientSystems, system_id).await?;
        let apps:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(a) || jsonb_build_object('assigned',p.id IS NOT NULL,'overrides',to_jsonb(p)) FROM apps a LEFT JOIN app_client_system p ON p.app_id=a.id AND p.client_system_id=$1 WHERE a.status='active' OR p.id IS NOT NULL ORDER BY a.name").bind(system_id).fetch_all(&self.0).await.map_err(db_error)?;
        Ok(json!({"clientSystem":system,"apps":apps}))
    }
    async fn usage(&self, filters: Value) -> Result<Value> {
        let n = |key: &str| {
            filters[key]
                .as_str()
                .and_then(|s| s.parse::<i64>().ok())
                .or_else(|| filters[key].as_i64())
        };
        let page = n("devices_page").unwrap_or(1).clamp(1, 1_000_000);
        let log_page = n("logs_page").unwrap_or(1).clamp(1, 1_000_000);
        let filter = "($1::bigint IS NULL OR d.app_id=$1) AND ($2::bigint IS NULL OR d.client_system_id=$2) AND (d.username ILIKE $3 OR u.device_id ILIKE $3)";
        let search = format!("%{}%", text(&filters, "search_devices"));
        let total:i64=sqlx::query_scalar(AssertSqlSafe(format!("SELECT count(*) FROM app_user_devices d JOIN user_devices u ON u.id=d.user_device_id WHERE {filter}").as_str())).bind(n("app_id")).bind(n("client_system_id")).bind(&search).fetch_one(&self.0).await.map_err(db_error)?;
        let devices:Vec<Value>=sqlx::query_scalar(AssertSqlSafe(format!("SELECT to_jsonb(d)||jsonb_build_object('app',to_jsonb(a),'client_system',to_jsonb(s),'user_device',to_jsonb(u)) FROM app_user_devices d JOIN apps a ON a.id=d.app_id JOIN client_systems s ON s.id=d.client_system_id JOIN user_devices u ON u.id=d.user_device_id WHERE {filter} ORDER BY d.last_connected_at DESC NULLS LAST,d.id DESC LIMIT 10 OFFSET $4").as_str())).bind(n("app_id")).bind(n("client_system_id")).bind(search).bind((page-1)*10).fetch_all(&self.0).await.map_err(db_error)?;
        let log_total:i64=sqlx::query_scalar("SELECT count(*) FROM app_connection_logs l JOIN app_user_devices d ON d.id=l.app_user_device_id WHERE ($1::bigint IS NULL OR d.app_id=$1) AND ($2::bigint IS NULL OR d.client_system_id=$2)").bind(n("log_app_id")).bind(n("log_client_system_id")).fetch_one(&self.0).await.map_err(db_error)?;
        let logs:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(l)||jsonb_build_object('app_user_device',to_jsonb(d)||jsonb_build_object('app',to_jsonb(a),'client_system',to_jsonb(s),'user_device',to_jsonb(u))) FROM app_connection_logs l JOIN app_user_devices d ON d.id=l.app_user_device_id JOIN apps a ON a.id=d.app_id JOIN client_systems s ON s.id=d.client_system_id JOIN user_devices u ON u.id=d.user_device_id WHERE ($1::bigint IS NULL OR d.app_id=$1) AND ($2::bigint IS NULL OR d.client_system_id=$2) ORDER BY l.created_at DESC,l.id DESC LIMIT 15 OFFSET $3").bind(n("log_app_id")).bind(n("log_client_system_id")).bind((log_page-1)*15).fetch_all(&self.0).await.map_err(db_error)?;
        Ok(
            json!({"appUserDevices":{"data":devices,"total":total,"current_page":page,"last_page":((total+9)/10).max(1)},"connectionLogs":{"data":logs,"total":log_total,"current_page":log_page,"last_page":((log_total+14)/15).max(1)},"apps":scalar(&self.0,"SELECT coalesce(jsonb_agg(to_jsonb(a) ORDER BY name),'[]') FROM apps a WHERE status='active'").await?,"clientSystems":scalar(&self.0,"SELECT coalesce(jsonb_agg(to_jsonb(s) ORDER BY name),'[]') FROM client_systems s WHERE status='active'").await?}),
        )
    }
    async fn app(&self, identifier: &str, numeric: bool) -> Result<Value> {
        let sql = if numeric && identifier.parse::<i64>().is_ok() {
            "SELECT to_jsonb(a) FROM apps a WHERE id::text=$1 AND status='active'"
        } else {
            "SELECT to_jsonb(a) FROM apps a WHERE uuid::text=$1 AND status='active'"
        };
        sqlx::query_scalar(sql)
            .bind(identifier)
            .fetch_optional(&self.0)
            .await
            .map_err(db_error)?
            .ok_or(Error::NotFound("Invalid or inactive app.".into()))
    }
    async fn system(&self, uuid: &str, app_id: i64) -> Result<Value> {
        let mut system:Value=sqlx::query_scalar("SELECT to_jsonb(s) || jsonb_build_object('client',(SELECT to_jsonb(c) FROM clients c WHERE c.id=s.client_id)) FROM client_systems s WHERE uuid::text=$1 AND status='active'").bind(uuid).fetch_optional(&self.0).await.map_err(db_error)?.ok_or(Error::NotFound("Invalid or inactive client system.".into()))?;
        let pivot: Value = sqlx::query_scalar(
            "SELECT to_jsonb(p) FROM app_client_system p WHERE app_id=$1 AND client_system_id=$2",
        )
        .bind(app_id)
        .bind(id(&system)?)
        .fetch_optional(&self.0)
        .await
        .map_err(db_error)?
        .ok_or(Error::Forbidden(
            "App is not allowed for this client system.".into(),
        ))?;
        system["overrides"] = pivot;
        Ok(system)
    }
    async fn access(&self, app_id: i64, endpoint: &str, db: &str) -> Result<Option<String>> {
        sqlx::query_scalar("SELECT s.uuid::text FROM client_systems s JOIN app_client_system p ON p.client_system_id=s.id WHERE p.app_id=$1 AND s.endpoint=$2 AND s.db_name=$3 AND s.status='active' ORDER BY s.id LIMIT 1").bind(app_id).bind(endpoint).bind(db).fetch_optional(&self.0).await.map_err(db_error)
    }
    async fn permissions(&self, modules: Vec<String>) -> Result<Value> {
        let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(m)||jsonb_build_object('features',coalesce((SELECT jsonb_object_agg(f.feature,true) FROM module_app_features f WHERE f.app_id=m.id),'{}')) FROM module_apps m").fetch_all(&self.0).await.map_err(db_error)?;
        let mut out = json!({});
        for row in rows {
            out[text(&row, "name")] = if modules.iter().any(|s| s == text(&row, "name")) {
                row["features"].clone()
            } else {
                json!(false)
            };
        }
        Ok(out)
    }
    async fn user(&self, id: i64) -> Result<Value> {
        sqlx::query_scalar(
            "SELECT to_jsonb(u)-'password'-'remember_token' FROM users u WHERE id=$1",
        )
        .bind(id)
        .fetch_optional(&self.0)
        .await
        .map_err(db_error)?
        .ok_or(Error::Unauthorized("User not found".into()))
    }
    async fn api_key(&self, key: &str) -> Result<()> {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM o4u_clients WHERE api_key=$1)")
                .bind(key)
                .fetch_one(&self.0)
                .await
                .map_err(db_error)?;
        if exists && !key.is_empty() {
            Ok(())
        } else {
            Err(Error::Forbidden("UnAuthorization".into()))
        }
    }
}
#[async_trait]
impl WriteRepository for Postgres {
    async fn save(&self, r: Resource, id: Option<i64>, mut data: Value) -> Result<Value> {
        let fields = r.fields();
        for key in &fields {
            if data.get(*key).is_none() {
                data[*key] = if ["allow_public", "allow_get_info"].contains(key) {
                    json!(false)
                } else {
                    Value::Null
                };
            }
            if data[*key].as_str().is_some_and(|s| s.trim().is_empty())
                && !["name", "status"].contains(key)
            {
                data[*key] = Value::Null;
            }
        }
        let table = r.table();
        let names = fields.join(",");
        let sql = if id.is_some() {
            format!(
                "UPDATE {table} t SET ({names},updated_at)=(SELECT {names},now() FROM jsonb_populate_record(NULL::{table},$1)) WHERE t.id=$2 RETURNING to_jsonb(t)"
            )
        } else {
            data["uuid"] = json!(Uuid::new_v4().to_string());
            format!(
                "INSERT INTO {table} AS t ({names},uuid,created_at,updated_at) SELECT {names},uuid,now(),now() FROM jsonb_populate_record(NULL::{table},$1) WHERE $2::bigint IS NULL RETURNING to_jsonb(t)"
            )
        };
        sqlx::query_scalar(AssertSqlSafe(sql.as_str()))
            .bind(data)
            .bind(id)
            .fetch_optional(&self.0)
            .await
            .map_err(db_error)?
            .ok_or(Error::NotFound("Record not found".into()))
    }
    async fn delete(&self, r: Resource, id: i64) -> Result<()> {
        let result = sqlx::query(AssertSqlSafe(
            format!("DELETE FROM {} WHERE id=$1", r.table()).as_str(),
        ))
        .bind(id)
        .execute(&self.0)
        .await
        .map_err(db_error)?;
        if result.rows_affected() == 0 {
            return Err(Error::NotFound("Record not found".into()));
        }
        Ok(())
    }
    async fn sync_apps(&self, system_id: i64, data: Value) -> Result<()> {
        let mut tx = self.0.begin().await.map_err(db_error)?;
        let exists: Option<i64> =
            sqlx::query_scalar("SELECT id FROM client_systems WHERE id=$1 FOR UPDATE")
                .bind(system_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(db_error)?;
        if exists.is_none() {
            return Err(Error::NotFound("Client system not found".into()));
        }
        let ids: Vec<i64> = data["app_ids"]
            .as_array()
            .ok_or_else(|| invalid("app_ids", "Required"))?
            .iter()
            .filter_map(Value::as_i64)
            .collect();
        for app_id in &ids {
            let app: Value = sqlx::query_scalar("SELECT to_jsonb(a) FROM apps a WHERE id=$1")
                .bind(app_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(db_error)?
                .ok_or_else(|| invalid("app_ids", "App not found"))?;
            let old:Value=sqlx::query_scalar("SELECT to_jsonb(p) FROM app_client_system p WHERE app_id=$1 AND client_system_id=$2").bind(app_id).bind(system_id).fetch_optional(&mut *tx).await.map_err(db_error)?.unwrap_or(json!({}));
            let mut settings = old;
            if let Some(url) = data["webapp_urls"].get(app_id.to_string()) {
                settings["webapp_url"] = url.clone();
            }
            if let Some(new) = data["app_settings"][app_id.to_string()].as_object() {
                for (key, v) in new {
                    if !SETTINGS.contains(&key.as_str()) {
                        return Err(invalid("app_settings", "Unknown setting"));
                    }
                    settings[key] = v.clone();
                }
            }
            validate_settings(&settings)?;
            validate_settings(&effective_settings(&app, &settings))?;
            settings["app_id"] = json!(app_id);
            settings["client_system_id"] = json!(system_id);
            let fields = SETTINGS.join(",");
            let update = SETTINGS
                .iter()
                .map(|s| format!("{s}=EXCLUDED.{s}"))
                .collect::<Vec<_>>()
                .join(",");
            sqlx::query(AssertSqlSafe(format!("INSERT INTO app_client_system (app_id,client_system_id,{fields},created_at,updated_at) SELECT app_id,client_system_id,{fields},now(),now() FROM jsonb_populate_record(NULL::app_client_system,$1) ON CONFLICT (app_id,client_system_id) DO UPDATE SET {update},updated_at=now()").as_str())).bind(settings).execute(&mut *tx).await.map_err(db_error)?;
        }
        sqlx::query(
            "DELETE FROM app_client_system WHERE client_system_id=$1 AND NOT(app_id=ANY($2))",
        )
        .bind(system_id)
        .bind(ids)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;
        tx.commit().await.map_err(db_error)
    }
    async fn ensure_system(&self, app: Value, endpoint: &str, db: &str) -> Result<Value> {
        let mut tx = self.0.begin().await.map_err(db_error)?;
        // Existing schema has no endpoint/db unique key. Serialize ensure without DDL.
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))")
            .bind(format!("{}|{}", endpoint.to_lowercase(), db))
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
        let found:Option<Value>=sqlx::query_scalar("SELECT to_jsonb(s) FROM client_systems s WHERE lower(rtrim(trim(endpoint),'/'))=lower($1) AND db_name=$2 ORDER BY id LIMIT 1 FOR UPDATE").bind(endpoint).bind(db).fetch_optional(&mut *tx).await.map_err(db_error)?;
        let created = found.is_none();
        let system = if let Some(s) = found {
            sqlx::query("UPDATE client_systems SET status='active',updated_at=now() WHERE id=$1")
                .bind(id(&s)?)
                .execute(&mut *tx)
                .await
                .map_err(db_error)?;
            s
        } else {
            let name = reqwest::Url::parse(endpoint)
                .ok()
                .and_then(|u| u.host_str().map(str::to_owned))
                .unwrap_or(db.into());
            let client_id:i64=sqlx::query_scalar("INSERT INTO clients (name,uuid,status,created_at,updated_at) VALUES ($1,$2::uuid,'active',now(),now()) RETURNING id").bind(&name).bind(Uuid::new_v4()).fetch_one(&mut *tx).await.map_err(db_error)?;
            sqlx::query_scalar("INSERT INTO client_systems AS s (name,uuid,client_id,endpoint,db_name,status,allow_get_info,created_at,updated_at) VALUES ($1,$2::uuid,$3,$4,$5,'active',false,now(),now()) RETURNING to_jsonb(s)").bind(format!("{name} / {db}")).bind(Uuid::new_v4()).bind(client_id).bind(endpoint).bind(db).fetch_one(&mut *tx).await.map_err(db_error)?
        };
        sqlx::query("INSERT INTO app_client_system (app_id,client_system_id,created_at,updated_at) VALUES ($1,$2,now(),now()) ON CONFLICT DO NOTHING").bind(id(&app)?).bind(id(&system)?).execute(&mut *tx).await.map_err(db_error)?;
        let pivot: Value = sqlx::query_scalar(
            "SELECT to_jsonb(p) FROM app_client_system p WHERE app_id=$1 AND client_system_id=$2",
        )
        .bind(id(&app)?)
        .bind(id(&system)?)
        .fetch_one(&mut *tx)
        .await
        .map_err(db_error)?;
        let mut out = effective_settings(&app, &pivot);
        out["client_system_id"] = system["uuid"].clone();
        out["created"] = json!(created);
        tx.commit().await.map_err(db_error)?;
        Ok(out)
    }
    async fn log_usage(&self, app: Value, system: Value, data: Value, ip: &str) -> Result<Value> {
        let mut tx = self.0.begin().await.map_err(db_error)?;
        let device_id:i64=sqlx::query_scalar("INSERT INTO user_devices(device_id,device_info,last_connected_at,created_at,updated_at) VALUES($1,$2,now(),now(),now()) ON CONFLICT(device_id) DO UPDATE SET device_info=EXCLUDED.device_info,last_connected_at=now(),updated_at=now() RETURNING id").bind(text(&data,"device_id")).bind(data.get("device_info").filter(|v|!v.is_null())).fetch_one(&mut *tx).await.map_err(db_error)?;
        let version = data["device_info"]
            .get("app_version")
            .filter(|v| !v.is_null())
            .or_else(|| data.get("app_version"))
            .and_then(Value::as_str);
        let link:i64=sqlx::query_scalar("INSERT INTO app_user_devices(app_id,client_system_id,user_device_id,username,app_version,last_connected_at,created_at,updated_at) VALUES($1,$2,$3,$4,$5,now(),now(),now()) ON CONFLICT(app_id,client_system_id,user_device_id) DO UPDATE SET username=EXCLUDED.username,app_version=EXCLUDED.app_version,last_connected_at=now(),updated_at=now() RETURNING id").bind(id(&app)?).bind(id(&system)?).bind(device_id).bind(text(&data,"username")).bind(version).fetch_one(&mut *tx).await.map_err(db_error)?;
        sqlx::query("INSERT INTO app_connection_logs(app_user_device_id,app_version,ip_address,created_at,updated_at) VALUES($1,$2,$3,now(),now())").bind(link).bind(version).bind(ip).execute(&mut *tx).await.map_err(db_error)?;
        tx.commit().await.map_err(db_error)?;
        let mut system = system;
        system.as_object_mut().map(|m| m.remove("overrides"));
        Ok(json!({"success":true,"message":"Logged successfully.","client_system":system}))
    }
    async fn update_device(&self, data: Value, key: &str) -> Result<Value> {
        for f in ["device_id", "device_info", "username", "domain", "db"] {
            o4u_domain::required(&data, f, 1, 65535)?;
        }
        let mut tx = self.0.begin().await.map_err(db_error)?;
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('o4u-user-device',0))")
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
        let client:Option<i64>=sqlx::query_scalar("SELECT id FROM o4u_clients WHERE api_key=$1 OR is_public=true ORDER BY (api_key=$1) DESC,id LIMIT 1").bind(key).fetch_optional(&mut *tx).await.map_err(db_error)?;
        let Some(client) = client else {
            return Ok(json!({"success":false,"error":"Invalid Client"}));
        };
        let user:Option<i64>=sqlx::query_scalar("SELECT id FROM client_users WHERE username=$1 AND domain=$2 AND db=$3 ORDER BY id LIMIT 1 FOR UPDATE").bind(text(&data,"username")).bind(text(&data,"domain")).bind(text(&data,"db")).fetch_optional(&mut *tx).await.map_err(db_error)?;
        let user = if let Some(user) = user {
            sqlx::query("UPDATE client_users SET client_id=$1,updated_at=now() WHERE id=$2")
                .bind(client)
                .bind(user)
                .execute(&mut *tx)
                .await
                .map_err(db_error)?;
            user
        } else {
            sqlx::query_scalar("INSERT INTO client_users(client_id,username,domain,db,created_at,updated_at) VALUES($1,$2,$3,$4,now(),now()) RETURNING id").bind(client).bind(text(&data,"username")).bind(text(&data,"domain")).bind(text(&data,"db")).fetch_one(&mut *tx).await.map_err(db_error)?
        };
        let device: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM client_user_devices WHERE user_id=$1 AND device_id=$2 LIMIT 1",
        )
        .bind(user)
        .bind(text(&data, "device_id"))
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;
        if let Some(device) = device {
            sqlx::query("UPDATE client_user_devices SET device_info=$1,status=true,updated_at=now() WHERE id=$2").bind(text(&data,"device_info")).bind(device).execute(&mut *tx).await.map_err(db_error)?;
        } else {
            sqlx::query("INSERT INTO client_user_devices(user_id,device_id,device_info,status,created_at,updated_at) VALUES($1,$2,$3,true,now(),now())").bind(user).bind(text(&data,"device_id")).bind(text(&data,"device_info")).execute(&mut *tx).await.map_err(db_error)?;
        }
        tx.commit().await.map_err(db_error)?;
        Ok(json!({"success":true,"message":"Save User info done"}))
    }
    async fn google_user(&self, email: &str, google_id: &str, avatar: &str) -> Result<Value> {
        sqlx::query_scalar("UPDATE users u SET google_id=$2,avatar=$3,updated_at=now() WHERE email=$1 RETURNING to_jsonb(u)-'password'-'remember_token'").bind(email).bind(google_id).bind(avatar).fetch_optional(&self.0).await.map_err(db_error)?.ok_or(Error::Unauthorized("User not found".into()))
    }
}
