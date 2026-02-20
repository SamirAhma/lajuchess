use std::sync::Arc;
use uuid::Uuid;
use crate::models::state::AppState;
use crate::models::game::{GameState, GameSession};

pub fn start_matchmaking_loop(matcher_state: AppState) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let mut queue = matcher_state.matchmaking.lock().await;
            if queue.len() >= 2 {
                let p1 = queue.pop_front().unwrap();
                let p2 = queue.pop_front().unwrap();
                
                let game_id = Uuid::new_v4();
                let creator_color = if rand::random::<bool>() { "white" } else { "black" };
                let game = GameState::new(creator_color.to_string());
                
                sqlx::query("INSERT INTO games (id, fen, pgn, creator_color, white_player_id, black_player_id) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(game_id)
                    .bind(&game.fen)
                    .bind(&game.pgn)
                    .bind(&creator_color)
                    .bind(if creator_color == "white" { p1 } else { p2 })
                    .bind(if creator_color == "white" { p2 } else { p1 })
                    .execute(&matcher_state.db)
                    .await
                    .unwrap();

                let (tx, _) = tokio::sync::broadcast::channel(32);
                let session = GameSession {
                    game: Arc::new(tokio::sync::Mutex::new(game)),
                    tx,
                };
                matcher_state.games.insert(game_id, session);
                
                tracing::info!("Match found! Game {} created for {:?} and {:?}", game_id, p1, p2);
            }
        }
    });
}
