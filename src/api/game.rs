use axum::{extract::State, Json, response::IntoResponse};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::state::AppState;
use crate::models::game::{GameState, GameSession};
use crate::models::messages::{CreateGameRequest, CreateGameResponse};

pub async fn create_game(
    State(state): State<AppState>,
    Json(payload): Json<CreateGameRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    
    let creator_color = match payload.side.as_str() {
        "white" => "white".to_string(),
        "black" => "black".to_string(),
        _ => if rand::random::<bool>() { "white".to_string() } else { "black".to_string() },
    };

    let game = GameState::new(creator_color.clone());
    
    // Save to DB
    sqlx::query("INSERT INTO games (id, fen, pgn, creator_color) VALUES ($1, $2, $3, $4)")
        .bind(id)
        .bind(&game.fen)
        .bind(&game.pgn)
        .bind(&creator_color)
        .execute(&state.db)
        .await
        .unwrap();

    let (tx, _) = tokio::sync::broadcast::channel(32);
    let session = GameSession {
        game: Arc::new(Mutex::new(game)),
        tx,
    };

    state.games.insert(id, session);

    Json(CreateGameResponse { id }).into_response()
}
