

async fn add_handler() -> String {
    // Example handler implementation
    "test".to_string()
}

pub async fn create_router() -> axum::Router {
    let router = axum::Router::new()
        .route("/add", axum::routing::get(add_handler));
    router
}

#[cfg(test)]
mod tests {
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let router = create_router().await;
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
