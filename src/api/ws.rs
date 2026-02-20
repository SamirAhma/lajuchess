use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Path, State},
    response::IntoResponse,
};
use uuid::Uuid;
use crate::models::state::AppState;
use crate::models::game::{GameState, GameSession};
use crate::models::messages::{ClientMessage, ServerMessage};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, id, state))
}

async fn handle_socket(mut socket: WebSocket, id: Uuid, state: AppState) {
    let session = if let Some(s) = state.games.get(&id) {
        s.clone()
    } else {
        let row: Option<(String, String, String, String)> = sqlx::query_as("SELECT fen, pgn, creator_color, white_player_id FROM games WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .unwrap();

        if let Some((fen, pgn, creator_color, _)) = row {
            let game = GameState { fen, pgn, creator_color };
            let (tx, _) = tokio::sync::broadcast::channel(32);
            let session = GameSession {
                game: std::sync::Arc::new(tokio::sync::Mutex::new(game)),
                tx,
            };
            state.games.insert(id, session.clone());
            session
        } else {
            let _ = socket.send(WsMessage::Text(serde_json::to_string(&ServerMessage::Error("Game not found".into())).unwrap())).await;
            return;
        }
    };

    let tx = session.tx.clone();
    let mut rx = tx.subscribe();
    let game_lock = session.game.clone();

    {
        let game = game_lock.lock().await;
        if let Err(_) = socket.send(WsMessage::Text(serde_json::to_string(&ServerMessage::State(game.clone())).unwrap())).await {
            return;
        }
    }

    loop {
        tokio::select! {
            Ok(new_state) = rx.recv() => {
                let msg = ServerMessage::State(new_state);
                if let Err(_) = socket.send(WsMessage::Text(serde_json::to_string(&msg).unwrap())).await {
                    break;
                }
            }
            Some(msg) = socket.recv() => {
                let msg = if let Ok(msg) = msg { msg } else { break; };
                
                if let WsMessage::Text(text) = msg {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        match client_msg {
                            ClientMessage::Move { uci } => {
                                let mut game = game_lock.lock().await;
                                if let Err(e) = game.make_move(&uci) {
                                    let resp = ServerMessage::Error(e);
                                    let _ = socket.send(WsMessage::Text(serde_json::to_string(&resp).unwrap())).await;
                                } else {
                                    sqlx::query("UPDATE games SET fen = $1, pgn = $2, updated_at = NOW() WHERE id = $3")
                                        .bind(&game.fen)
                                        .bind(&game.pgn)
                                        .bind(id)
                                        .execute(&state.db)
                                        .await
                                        .unwrap();
                                    let _ = tx.send(game.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
