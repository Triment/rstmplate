use axum::{ body::Body, extract::Request, http::StatusCode, middleware::Next, response::{Response}, Router};
use common::{plugin_config::PluginConfig};

/// 创建资源处理宏，接受folder参数并返回asset_handler函数
#[macro_export]
macro_rules! create_asset_handler {
    ($folder:expr) => {
        {
            #[derive(rust_embed::RustEmbed)]
            #[folder = $folder] 
            struct Assets;

            async move |Path(path): Path<String>| -> Response {
                match Assets::get(path.as_str()) {
                  Some(content) => {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                  }
                  None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
                }
            }
        }
    };
}





//插件的静态文件访问路径为 plugins/assets/<author>
pub trait Plugin: Send + Sync + 'static {
    fn config(&self) -> PluginConfig;
    //middleware将用于处理请求的中间件函数
    fn middleware(&self) -> Option<fn(Request<Body>, Next) -> futures::future::BoxFuture<'static, Result<Response<Body>, StatusCode>>>;
    fn routes(&self, context: common::state::AppState) -> Router; // 示例：返回路由内容
}

// 插件工厂函数签名（插件必须导出这个）
pub type PluginCreate = fn() -> Box<dyn Plugin>;