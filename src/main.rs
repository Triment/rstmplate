use std::time;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::mpsc;
mod plugin_loader;
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    loop {
        let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<()>();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dotenvy::var("DATABASE_URL")?.as_str())
            .await?;
        let mut app = api::create_router().with_state(common::state::AppState {
            db_pool: pool.clone(),
            shutdown_send: shutdown_send.clone(),
        });
        for plugin in plugin_loader::load_plugins()? {
            // 这里可以将插件注册到你的应用中，例如添加路由或中间件
            if let Some(middle) = plugin.middleware() {
                //continue; // 如果插件没有中间件，则跳过
                app = app.nest(
                    &plugin.config().endpoint,
                    plugin
                        .routes(common::state::AppState {
                            db_pool: pool.clone(),
                            shutdown_send: shutdown_send.clone(),
                        })
                        .layer(axum::middleware::from_fn(middle)),
                );
            } else {
                app = app.nest(
                    &plugin.config().endpoint,
                    plugin.routes(common::state::AppState {
                        db_pool: pool.clone(),
                        shutdown_send: shutdown_send.clone(),
                    }),
                );
            }
        }
        app = app.route(
            "/dura",
            axum::routing::get(async || {
                tokio::time::sleep(time::Duration::from_secs(5)).await;
                "test"
            }),
        );
        let addr = "127.0.0.1:3000";
        println!("listening on http://{}", addr);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        let server = axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                shutdown_recv.recv().await;
            });
        let exit_signal = tokio::spawn(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("Got Ctrl+C, exiting...");
        });
        tokio::select! {
            _ = server => {
                println!("Server stopped, restarting...");
                continue; // 走下一轮 loop
            }
            _ = exit_signal => {
                println!("Exiting process...");
                shutdown_send.send(()).unwrap(); // 发送关闭信号
                break; // 跳出 loop，进程退出
            }
        };
    }
    Ok(())
}
