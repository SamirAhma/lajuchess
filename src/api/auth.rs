use axum::{extract::State, Json, response::IntoResponse};
use rand::rngs::OsRng;
use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use uuid::Uuid;
use crate::models::state::AppState;
use crate::models::user::{RegisterRequest, LoginRequest, AuthResponse, Claims};
use crate::core::token::encode_token;

pub async fn auth_signup(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(payload.password.as_bytes(), &salt).unwrap().to_string();

    let result = sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(&payload.username)
        .bind(&hash)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Json(serde_json::json!({ "status": "success" })).into_response(),
        Err(_) => (axum::http::StatusCode::BAD_REQUEST, "Username already exists").into_response(),
    }
}

pub async fn auth_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let row: Option<(Uuid, String)> = sqlx::query_as("SELECT id, password_hash FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if let Some((id, hash)) = row {
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(7))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: payload.username.clone(),
                id,
                exp: expiration,
            };

            let token = encode_token(&claims).unwrap();

            return Json(AuthResponse {
                token,
                username: payload.username,
            }).into_response();
        }
    }

    (axum::http::StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
}
