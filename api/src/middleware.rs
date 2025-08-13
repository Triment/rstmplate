use axum::{Extension, extract::Request, http::StatusCode, middleware::Next, response::Response};
use common::jwt::verify_jwt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}

async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let token_opt = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(token) = token_opt {
        let data = verify_jwt::<Claims>(token, None)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        req.extensions_mut().insert(data.claims);
        return Ok(next.run(req).await);
    }
    Err(StatusCode::UNAUTHORIZED)
}
