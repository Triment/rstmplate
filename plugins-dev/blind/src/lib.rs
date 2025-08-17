use axum::{
    body::Body, extract::{Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}, Router
};
use common::jwt::verify_jwt;
use jsonwebtoken::Validation;
use plugin::Plugin;
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
    fn config(&self) -> plugin::PluginConfig {
        plugin::PluginConfig {
            name: "HelloPlugin".to_string(),
            description: "A simple hello world plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "Your Name".to_string(),
            endpoint: "/hello".to_string(), // 插件的端点
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
        Router::new()
            .route("/", axum::routing::get(home))
            .with_state(context)
    }
}

async fn home(State(state): State<common::state::AppState>) -> impl IntoResponse {
    let _ = state.shutdown_send.send(());
    "重启信号已发送，正在重启..."
}


// 导出插件工厂函数 (必须是 extern "C")
#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(HelloPlugin)
}
