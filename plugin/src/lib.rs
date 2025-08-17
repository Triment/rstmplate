use axum::{ body::Body, extract::Request, http::StatusCode, middleware::Next, response::{Response}, Router};
use common::state::AppState;

#[doc = include_str!("../README.md")]
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub endpoint: String, // 插件的端点
}

pub trait Plugin: Send + Sync + 'static {
    fn config(&self) -> PluginConfig;
    //middleware将用于处理请求的中间件函数
    fn middleware(&self) -> Option<fn(Request<Body>, Next) -> futures::future::BoxFuture<'static, Result<Response<Body>, StatusCode>>>;
    fn routes(&self, context: AppState) -> Router; // 示例：返回路由内容
}

// 插件工厂函数签名（插件必须导出这个）
pub type PluginCreate = fn() -> Box<dyn Plugin>;