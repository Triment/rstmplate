use axum::extract::State;
use common::state::AppState;
use validator::Validate;
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use i18n::t;
static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

// CREATE USER
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
    #[validate(length(min = 3, max = 16), regex(path = "USERNAME_REGEX"))]
    username: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}

pub async fn sign_up(
    State(state): State<AppState>,
    axum::Json(user): axum::Json<UserAuth>,
) -> Result<axum::Json<models::user::User>, common::error::CommonError> {
    user.validate()?;
    // Example user creation logic
    let user = models::user::User::create(
        &state.db_pool,
        user.username.clone(),
        common::password::hash(user.password).await?,
    ).await?;
    Ok(axum::Json(user))
}

pub async fn sign_in(
    State(state): State<AppState>,
    axum::Json(user): axum::Json<UserAuth>,
) -> Result<axum::Json<String>, common::error::CommonError> {
    user.validate()?;
    // Example user authentication logic
    let user = models::user::User::get_by_username(
        &state.db_pool,
        &user.username.clone()
    ).await?;

    if user.is_none() || !common::password::verify(user.clone().unwrap().password_hash, user.clone().unwrap().password_hash).await? {
        return Err(common::error::CommonError::Unauthorized(t!("common.error.UsernameOrPasswordIncorrect").into()));
    }
    #[derive(serde::Serialize)]
    struct Cliams {
        aud: &'static str,
        sub: String,
        iss: &'static str,
        nbf: u64,
        exp: u64,
        roles: Option<Vec<String>>,
    }
    let claims = Cliams {
        aud: "my_audience",
        sub: user.unwrap().id.unwrap().to_string(),
        iss: "my_issuer",
        nbf: time::OffsetDateTime::now_utc().unix_timestamp() as u64,
        exp: (time::OffsetDateTime::now_utc() + time::Duration::days(1)).unix_timestamp() as u64,
        roles: Some(vec!["user".to_string()]), // Example role, adjust as needed
    };
    
    let token = common::jwt::create_token(&claims, None).await?;
    Ok(axum::Json(token))
}
