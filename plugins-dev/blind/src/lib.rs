use axum::{
    body::Body, extract::{Path, Request, State}, http::{header, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Router
};
use plugin::{create_asset_handler, Plugin};
use serde::{Deserialize, Serialize};
struct HelloPlugin;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}
impl Plugin for HelloPlugin {
    fn config(&self) -> common::plugin_config::PluginConfig {
        common::plugin_config::PluginConfig {
            name: "HelloPlugin".to_string(),
            description: "A simple hello world plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "assets".to_string(),
            endpoint: "/hello".to_string(), // 插件的端点
            dependencies: None,
            middleware_scope: common::plugin_config::MiddlewareScope::PluginOnly, // 中间件仅应用于插件路由
            assets: Some(vec!["main.js".to_string()]),
        }
    }

    fn middleware(
        &self,
    ) -> Option<
        fn(
            Request<axum::body::Body>,
            Next,
        ) -> futures::future::BoxFuture<'static, Result<Response<Body>, StatusCode>>,
    > {
        None
    }
    fn routes(&self, context: common::state::AppState) -> Router {
        // let service = tower_http::services::ServeDir::new("plugins/assets");
        Router::new()
            .route("/", axum::routing::get(home))
            .route("/assets/{*path}", axum::routing::get(create_asset_handler!("assets/")))
            .with_state(context)
    }
}

async fn home(State(state): State<common::state::AppState>) -> impl IntoResponse {
    println!("{:?}", state.plugins);
    let _ = state.shutdown_send.send(());
    "重启信号已发送，正在重启..."
}

// 导出插件工厂函数 (必须是 extern "C")

#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(HelloPlugin)
}
