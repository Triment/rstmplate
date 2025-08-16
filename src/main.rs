use sqlx::postgres::PgPoolOptions;
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenvy::var("DATABASE_URL")?.as_str())
        .await?;
    let app = api::create_router().with_state(common::state::AppState { db_pool: pool.clone() });

    let addr = "127.0.0.1:3000";
    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
