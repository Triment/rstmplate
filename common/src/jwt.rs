/// This module provides functionality for handling JSON Web Tokens (JWTs).
use std::str::FromStr;

use jsonwebtoken::{Validation, Algorithm, decode};

use crate::error::CommonError;

/// Parses the JWT algorithm from environment variables or defaults to HS256.
/// Returns the parsed `Algorithm`.
pub async fn parse_algorithm() -> Algorithm {
    dotenvy::var("JWT_ALGORITHM")
        .map(|alg| Algorithm::from_str(&alg).unwrap_or(Algorithm::HS256))
        .unwrap_or(Algorithm::HS256)
}

// pub async fn get_validation() -> Validation {
//     let mut validation = Validation::default();
//     validation.leeway = 60; // Allow a 60 second leeway for expiration
//     validation.validate_exp = true;
//     validation.validate_nbf = true;
//     validation.validate_aud = true;
//     validation.set_issuer(&["my_issuer"]);
//     validation.set_audience(&["my_audience"]);
//     validation.sub = Some("".to_string());
//     //"exp", "nbf", "aud", "iss", "sub"
//     validation.set_required_spec_claims(
//         &["exp", "nbf", "aud", "iss", "sub"],
//     );

//     validation
// }

pub async fn create_token<T>(
    claims: &T,
    header: Option<jsonwebtoken::Header>,
) -> Result<String, CommonError>
where
    T: serde::Serialize,
{
    let secret = dotenvy::var("JWT_SECRET").unwrap_or("default_key".to_string());
    let header = header.unwrap_or_else(jsonwebtoken::Header::default);
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_ref());

    Ok(jsonwebtoken::encode(&header, claims, &encoding_key)?)
}

pub fn verify_jwt<Data>(
    token: &str,
    validation: Validation,
) -> Result<jsonwebtoken::TokenData<Data>, CommonError>
where
    Data: for<'de> serde::Deserialize<'de>,
{
    
    let secret = dotenvy::var("JWT_SECRET").unwrap_or("default_key".to_string());
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret.as_ref());

    Ok(decode(token, &decoding_key, &validation)?)
}
