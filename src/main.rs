use axum::{extract::Path, Router};
use sqlx::postgres::PgPoolOptions;
use std::{sync::{Arc}, time};
use tokio::sync::mpsc;
mod plugin_loader;
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::layer::SubscriberExt::with(tracing_subscriber::registry(),tracing_subscriber::fmt::layer().with_target(false).with_level(true)));
    loop {
        let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<()>();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dotenvy::var("DATABASE_URL")?.as_str())
            .await?;
        let mut state = common::state::AppState {
            db_pool: pool.clone(),
            shutdown_send: shutdown_send.clone(),
            plugins: Arc::new(vec![].into()),
        };
        let mut app = Router::new().with_state(state.clone());
        //let mut app = api::create_router(state.clone()).with_state(state);
        for plugin in plugin_loader::load_plugins()? {
            // 这里可以将插件注册到你的应用中，例如添加路由或中间件
            let plugin_config = plugin.config();
            let plugin_routes = plugin.routes(state.clone());
            let endpoint = format!("/plugins{}", plugin_config.endpoint);
            if let Ok(mut plugins_vec) = state.plugins.lock() {
                plugins_vec.push(plugin_config.clone());
            }
            if let Some(middle) = plugin.middleware() {
                match plugin_config.middleware_scope {
                    common::plugin_config::MiddlewareScope::PluginOnly => {
                        // 中间件仅应用于插件路由
                        app = app.nest(
                            &endpoint,
                            plugin_routes.layer(axum::middleware::from_fn(middle)),
                        );
                    },
                    common::plugin_config::MiddlewareScope::Global => {
                        // 中间件应用于全局
                        app = app
                            .layer(axum::middleware::from_fn(middle))
                            .nest(&endpoint, plugin_routes);
                    }
                }
            } else {
                // 没有中间件，直接添加路由
                app = app.nest(&endpoint, plugin_routes);
            }
            // app = app
            //     .nest_service(
            //         "/assets",
            //         tower_http::services::ServeDir::new(
            //             path::PathBuf::from("plugins").join(&plugin.config().author),
            //         ),
            //     );
        }
        app = app.route(
            "/dura",
            axum::routing::get(async || {
                tokio::time::sleep(time::Duration::from_secs(5)).await;
                "test"
            }),
        ).layer(tower_http::trace::TraceLayer::new_for_http().make_span_with(|req: &axum::http::Request<_>| {
             let path = if let Some(path) = req.extensions().get::<axum::extract::OriginalUri>() {
                 // This will include `/api`
                 path.0.path().to_owned()
             } else {
                 // The `OriginalUri` extension will always be present if using
                 // `Router` unless another extractor or middleware has removed it
                 req.uri().path().to_owned()
             };
             tracing::info_span!("http-request", %path)
         }));
        let addr = "127.0.0.1:3000";
        println!("listening on http://{}", addr);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        let server = axum::serve(listener, app).with_graceful_shutdown(async move {
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
