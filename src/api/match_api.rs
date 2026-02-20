use axum::{extract::State, Json, response::IntoResponse};
use uuid::Uuid;
use crate::models::state::AppState;
use crate::models::messages::MatchmakingRequest;
use crate::core::token::decode_token;

pub async fn join_matchmaking(
    State(state): State<AppState>,
    Json(payload): Json<MatchmakingRequest>,
) -> impl IntoResponse {
    let token_data = decode_token(&payload.token);

    match token_data {
        Ok(data) => {
            let user_id = data.id;
            let mut queue = state.matchmaking.lock().await;
            if !queue.contains(&user_id) {
                queue.push_back(user_id);
            }
            Json(serde_json::json!({ "status": "queued" })).into_response()
        }
        Err(_) => (axum::http::StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    }
}

pub async fn match_status(
    State(state): State<AppState>,
    Json(payload): Json<MatchmakingRequest>,
) -> impl IntoResponse {
    let token_data = decode_token(&payload.token);

    match token_data {
        Ok(data) => {
            let user_id = data.id;
            let row: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM games WHERE (white_player_id = $1 OR black_player_id = $1) AND created_at > NOW() - INTERVAL '1 minute' ORDER BY created_at DESC LIMIT 1")
                .bind(user_id)
                .fetch_optional(&state.db)
                .await
                .unwrap();

            if let Some((game_id,)) = row {
                Json(serde_json::json!({ "status": "matched", "game_id": game_id })).into_response()
            } else {
                Json(serde_json::json!({ "status": "waiting" })).into_response()
            }
        }
        Err(_) => (axum::http::StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    }
}
