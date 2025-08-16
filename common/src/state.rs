#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub shutdown_send: tokio::sync::mpsc::UnboundedSender<()>,
}