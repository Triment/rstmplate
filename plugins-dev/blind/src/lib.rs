use axum::{
    body::Body, extract::{Path, Request, State}, http::{header, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Router
};
use plugin::Plugin;
use serde::{Deserialize, Serialize};

use rust_embed::RustEmbed;
use mime_guess;

#[derive(RustEmbed)]
#[folder = "assets/"] 
struct Assets;

async fn asset_handler(Path(path): Path<String>) -> Response {
    match Assets::get(path.as_str()) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}

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
            author: "assets".to_string(),
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
        // let service = tower_http::services::ServeDir::new("plugins/assets");

        Router::new()
            .route("/", axum::routing::get(home))
            .route("/assets/{*path}", axum::routing::get(asset_handler))
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
