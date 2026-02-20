use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use crate::models::user::Claims;
use once_cell::sync::Lazy;
use std::env;

pub static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "secret".to_string())
        .into_bytes()
});

pub fn encode_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::default(), claims, &EncodingKey::from_secret(&JWT_SECRET))
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
