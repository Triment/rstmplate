use axum::{Router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::util::SubscriberInitExt::init(
        tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_level(true),
        ),
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenvy::var("DATABASE_URL")?.as_str())
        .await?;
    let state = common::state::AppState {
        db_pool: pool.clone()
    };
    let mut app = Router::new().with_state(state);
    app = app.layer(
        tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |req: &axum::http::Request<_>| {
                let path = if let Some(path) = req.extensions().get::<axum::extract::OriginalUri>()
                {
                    path.0.path().to_owned()
                } else {
                    req.uri().path().to_owned()
                };
                tracing::info_span!("http-request", %path)
            },
        ),
    );
    let addr = "127.0.0.1:3000";
    println!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let _ = axum::serve(listener, app).with_graceful_shutdown(async move {
        let _ = tokio::signal::ctrl_c().await;
    });
    Ok(())
}
