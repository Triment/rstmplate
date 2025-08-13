use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
#[serde_with::serde_as]
#[derive(Serialize)]
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
    pub fn new(id: uuid::Uuid, username: String, password_hash: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: None,
            username,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, new_username: Option<String>, new_password_hash: Option<String>) {
        if let Some(username) = new_username {
            self.username = username;
        }
        if let Some(password_hash) = new_password_hash {
            self.password_hash = password_hash;
        }
        self.updated_at = OffsetDateTime::now_utc();
    }
}
