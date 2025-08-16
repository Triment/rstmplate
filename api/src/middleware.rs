use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use common::jwt::verify_jwt;
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}
/// iss (Issuer):
/// 签发者，表示JWT 的发行方。通常是一个URL 或者一个标识符，用于标识令牌的来源。
/// sub (Subject):
/// 主题，表示JWT 的使用者。通常是一个用户ID、用户名或其他标识用户身份的字符串。
/// aud (Audience):
/// 受众，表示JWT 的接收方。可以是一个或多个URL 或标识符，用于指定哪些接收方可以验证和使用该令牌。
/// exp (Expiration Time):
/// 过期时间，表示JWT 的失效时间。JWT 在此时间之后将不再有效。这通常是一个Unix 时间戳(以秒为单位)。
/// nbf (Not Before):
/// 生效时间，表示JWT 可以被验证和使用的起始时间。JWT 在此时间之前是无效的。这通常也是一个Unix 时间戳。
/// iat (Issued At):
/// 签发时间，表示JWT 的生成时间。这通常也是一个Unix 时间戳。
/// jti (JWT ID):
/// JWT 唯一标识符，用于区分不同的JWT，可以用来防止重放攻击。
/// Middleware to authenticate requests using JWT.
async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let token_opt = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(token) = token_opt {
        let mut validation = Validation::new(common::jwt::parse_algorithm().await);
        validation.set_issuer(&["my_issuer"]);
        validation.set_audience(&["my_audience"]);
        validation.set_required_spec_claims(
            &["exp", "nbf", "aud", "iss", "sub"],
        );
        let data = verify_jwt::<Claims>(token, validation)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        req.extensions_mut().insert(data.claims);
        return Ok(next.run(req).await);
    }
    Err(StatusCode::UNAUTHORIZED)
}
