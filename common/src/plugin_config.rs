#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub name: String,// 插件名称
    pub description: String,// 插件描述
    pub version: String,// 插件版本
    pub author: String,// 插件作者
    pub endpoint: String,// 插件的端点
    pub dependencies: Option<Vec<PluginDependency>>,// 插件依赖
    pub middleware_scope: MiddlewareScope,// 中间件应用范围
    pub assets: Option<Vec<String>>,// 插件的静态文件
}
/// 中间件应用范围
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MiddlewareScope {
    /// 中间件仅应用于插件路由
    PluginOnly,
    /// 中间件应用于全局
    Global,
}
impl Default for MiddlewareScope {
    fn default() -> Self {
        MiddlewareScope::PluginOnly // 默认为仅插件路由
    }
}