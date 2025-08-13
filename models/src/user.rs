use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
#[serde_with::serde_as]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub username: String,
    pub password_hash: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl User {
    pub async fn create(
        pool: &sqlx::PgPool,
        username: String,
        password_hash: String
    ) -> Result<Self, common::error::CommonError> {
        let user = sqlx::query_as!(
            Self,
            r#"INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash, created_at, updated_at"#,
            username,
            password_hash
        ).fetch_one(pool).await?;
        Ok(user)
    }
    pub async fn get_by_username(
        pool: &sqlx::PgPool,
        username: &str
    ) -> Result<Option<Self>, common::error::CommonError> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE username = $1",
            username
        ).fetch_optional(pool).await?;
        Ok(user)
    }
    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<Self>, common::error::CommonError> {
        let users = sqlx::query_as!(Self, "SELECT * FROM users").fetch_all(pool).await?;
        Ok(users)
    }
}