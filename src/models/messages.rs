use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;
use super::game::GameState;

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    Move { uci: String },
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    State(GameState),
    Error(String),
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateGameRequest {
    pub side: String, // "white", "black", "random"
}

#[typeshare]
#[derive(Serialize)]
pub struct CreateGameResponse {
    pub id: Uuid,
}

#[typeshare]
#[derive(Deserialize)]
pub struct MatchmakingRequest {
    pub token: String,
}
