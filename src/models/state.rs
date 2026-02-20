use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;
use uuid::Uuid;
use super::game::GameSession;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub games: Arc<DashMap<Uuid, GameSession>>,
    pub matchmaking: Arc<Mutex<std::collections::VecDeque<Uuid>>>,
}
