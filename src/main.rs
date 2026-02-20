use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;

pub mod api;
pub mod core;
pub mod db;
pub mod models;

use models::state::AppState;
use api::auth::{auth_login, auth_signup};
use api::game::create_game;
use api::match_api::{join_matchmaking, match_status};
use api::ws::ws_handler;
use crate::core::matchmaking::start_matchmaking_loop;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();


    let db_pool = db::setup_db().await;
    let state = AppState {
        db: db_pool,
        games: Arc::new(DashMap::new()),
        matchmaking: Arc::new(Mutex::new(std::collections::VecDeque::new())),
    };

    let app = Router::new()
        .route("/api/auth/signup", post(auth_signup))
        .route("/api/auth/login", post(auth_login))
        .route("/api/match/join", post(join_matchmaking))
        .route("/api/match/status", post(match_status))
        .route("/api/games", post(create_game))
        .route("/ws/:id", get(ws_handler))
        .fallback_service(ServeDir::new("public").not_found_service(ServeFile::new("public/index.html")))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    start_matchmaking_loop(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
