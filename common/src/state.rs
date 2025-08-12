#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
}