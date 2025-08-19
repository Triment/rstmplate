use axum::{ body::Body, extract::Request, http::StatusCode, middleware::Next, response::{Response}, Router};
use common::state::AppState;

#[doc = include_str!("../README.md")]
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,// 插件名称
    pub description: String,// 插件描述
    pub version: String,// 插件版本
    pub author: String,// 插件作者
    pub endpoint: String, // 插件的端点

}
//插件的静态文件访问路径为 plugins/assets/<author>
pub trait Plugin: Send + Sync + 'static {
    fn config(&self) -> PluginConfig;
    //middleware将用于处理请求的中间件函数
    fn middleware(&self) -> Option<fn(Request<Body>, Next) -> futures::future::BoxFuture<'static, Result<Response<Body>, StatusCode>>>;
    fn routes(&self, context: AppState) -> Router; // 示例：返回路由内容
}

// 插件工厂函数签名（插件必须导出这个）
pub type PluginCreate = fn() -> Box<dyn Plugin>;