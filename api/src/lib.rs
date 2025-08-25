use axum::{extract::State};
use common::state::AppState;

use crate::token::sign_in;
pub mod middleware;
mod token;

async fn get_user(
    State(state): State<AppState>
) -> Result<axum::Json<Vec<models::user::User>>, common::error::CommonError> {
    // Example user retrieval logic
    let user = models::user::User::get_all(&state.db_pool).await?;
    Ok(axum::Json(user))
}

pub fn create_router() -> axum::Router<AppState> {
    let router = axum::Router::new()
        .route("/v1/user", axum::routing::post(sign_in))
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
        let app_state = AppState {db_pool:pool.clone() };
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
