use o4u_api::AppState;
use o4u_infrastructure::Config;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only load this source's .env. Legacy configuration remains untouched.
    dotenvy::from_path(".env").ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("o4u_api=info,tower_http=info".into()),
        )
        .init();
    let config = Config::from_env()?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    let state = AppState::new(config.clone(), pool);
    if std::env::args().any(|arg| arg == "worker") {
        loop {
            tokio::select! {
                _=tokio::signal::ctrl_c()=>break,
                result=state.db.process_notification(&state.integrations)=>{
                    match result {Ok(true)=>{},Ok(false)=>tokio::time::sleep(std::time::Duration::from_secs(2)).await,Err(e)=>{tracing::error!(error=%e,"Worker error");tokio::time::sleep(std::time::Duration::from_secs(5)).await;}}
                }
            }
        }
        return Ok(());
    }
    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    tracing::info!(address=%config.bind,"O4U API listening");
    axum::serve(
        listener,
        o4u_api::router(state).into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.ok();
    })
    .await?;
    Ok(())
}
