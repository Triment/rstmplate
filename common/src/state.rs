use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub shutdown_send: tokio::sync::mpsc::UnboundedSender<()>,
    pub plugins: Arc<Mutex<Vec<crate::plugin_config::PluginConfig>>>,
}
