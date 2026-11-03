use crate::{Postgres, integrations::Integrations, postgres::db_error};
use o4u_domain::Result;
use serde_json::Value;
impl Postgres {
    pub async fn enqueue_fcm(&self, data: Value) -> Result<()> {
        sqlx::query("INSERT INTO jobs(queue,payload,attempts,reserved_at,available_at,created_at) VALUES('o4u-rust-fcm',$1,0,NULL,extract(epoch from now())::integer,extract(epoch from now())::integer)").bind(data.to_string()).execute(&self.0).await.map_err(db_error)?;
        Ok(())
    }
    pub async fn process_notification(&self, integrations: &Integrations) -> Result<bool> {
        let job:Option<(i64,String,i16)>=sqlx::query_as("UPDATE jobs SET reserved_at=extract(epoch from now())::integer,attempts=attempts+1 WHERE id=(SELECT id FROM jobs WHERE queue='o4u-rust-fcm' AND available_at<=extract(epoch from now()) AND (reserved_at IS NULL OR reserved_at<extract(epoch from now())-1800) ORDER BY id LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id,payload,attempts::smallint").fetch_optional(&self.0).await.map_err(db_error)?;
        let Some((id, payload, attempts)) = job else {
            return Ok(false);
        };
        let data: Value =
            serde_json::from_str(&payload).map_err(|_| o4u_domain::Error::Internal)?;
        match integrations.firebase(&data).await {
            Ok(()) => {
                sqlx::query("DELETE FROM jobs WHERE id=$1")
                    .bind(id)
                    .execute(&self.0)
                    .await
                    .map_err(db_error)?;
            }
            Err(error) => {
                tracing::warn!(job_id=id,attempts,error=%error,"FCM delivery failed");
                if attempts >= 3 {
                    let mut tx = self.0.begin().await.map_err(db_error)?;
                    sqlx::query("INSERT INTO failed_jobs(uuid,connection,queue,payload,exception,failed_at) VALUES($1,'database','o4u-rust-fcm',$2,$3,now())").bind(uuid::Uuid::new_v4().to_string()).bind(payload).bind(error.to_string()).execute(&mut *tx).await.map_err(db_error)?;
                    sqlx::query("DELETE FROM jobs WHERE id=$1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                        .map_err(db_error)?;
                    tx.commit().await.map_err(db_error)?;
                } else {
                    sqlx::query("UPDATE jobs SET reserved_at=NULL,available_at=extract(epoch from now())::integer+60 WHERE id=$1").bind(id).execute(&self.0).await.map_err(db_error)?;
                }
            }
        }
        Ok(true)
    }
}
