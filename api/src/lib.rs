

use std::sync::LazyLock;

use axum::{extract::State, Extension};
use common::state::AppState;
use models::user;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;
use uuid::Uuid;
mod middleware;

async fn add_handler() -> String {
    // Example handler implementation
    "test".to_string()
}

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

async fn create_user(
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

async fn get_user(
    State(state): State<AppState>
) -> Result<axum::Json<Vec<models::user::User>>, common::error::CommonError> {
    // Example user retrieval logic
    let user = models::user::User::get_all(&state.db_pool).await?;
    Ok(axum::Json(user))
}

pub fn create_router() -> axum::Router<AppState> {
    let router = axum::Router::new()
        .route("/add", axum::routing::get(add_handler))
        .route("/v1/user", axum::routing::post(create_user))
        .route("/v1/user", axum::routing::get(get_user));
    router
}

#[cfg(test)]
mod tests {
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use sqlx::postgres::PgPoolOptions;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
            .await
            .unwrap();
        let app_state = AppState { db_pool: pool.clone() };
        let router = create_router().with_state(app_state);
        // Here you would typically test the router's functionality
        // For example, you could use axum's test utilities to send requests
        // and assert the responses.
        let resp = router.oneshot(
            Request::get("/add")
            .header("content-type", "application/text")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"test");
    }
}
