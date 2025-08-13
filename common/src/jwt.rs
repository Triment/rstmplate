use jsonwebtoken::{Validation, Algorithm, decode};

use crate::error::CommonError;

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
    validation: Option<Validation>,
) -> Result<jsonwebtoken::TokenData<Data>, CommonError>
where
    Data: for<'de> serde::Deserialize<'de>,
{
    let secret = dotenvy::var("JWT_SECRET").unwrap_or("default_key".to_string());
    let validation = validation.unwrap_or_else(Validation::default);
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret.as_ref());

    Ok(decode(token, &decoding_key, &validation)?)
}
