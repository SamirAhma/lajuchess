use sqlx::{postgres::PgPool, Pool, Postgres};
use std::env;

pub async fn setup_db() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS games (
            id UUID PRIMARY KEY,
            fen TEXT NOT NULL,
            pgn TEXT NOT NULL,
            creator_color TEXT NOT NULL DEFAULT 'white',
            white_player_id UUID REFERENCES users(id),
            black_player_id UUID REFERENCES users(id),
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}
