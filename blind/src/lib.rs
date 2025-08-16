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
        Some(|mut req: Request<Body>, next: Next| {
            Box::pin(async move {
                let token_opt = req
                    .headers()
                    .get("Authorization")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.strip_prefix("Bearer "));

                if let Some(token) = token_opt {
                    let mut validation = Validation::new(common::jwt::parse_algorithm().await);
                    validation.set_issuer(&["my_issuer"]);
                    validation.set_audience(&["my_audience"]);
                    validation.set_required_spec_claims(&["exp", "nbf", "aud", "iss", "sub"]);
                    let data = verify_jwt::<Claims>(token, validation)
                        .map_err(|_| StatusCode::UNAUTHORIZED)?;
                    req.extensions_mut().insert(data.claims);
                    return Ok(next.run(req).await);
                }
                return Ok(next.run(req).await);
                Err(StatusCode::UNAUTHORIZED)
            })
        })
    }

    fn routes(&self, context: common::state::AppState) -> Router {
        Router::new()
            .route("/", axum::routing::get(home))
            .with_state(context)
    }
}

async fn home(State(state): State<common::state::AppState>) -> impl IntoResponse {
    state.shutdown_send.send(());
    "Hello, bbs!"
}


// 导出插件工厂函数 (必须是 extern "C")
#[unsafe(no_mangle)]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(HelloPlugin)
}
