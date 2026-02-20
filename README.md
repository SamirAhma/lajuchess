# LajuChess - High Performance Chess

Fast, lightweight, and incredibly scalable chess platform built with Rust + SvelteKit.

## Features
- **Enterprise Architecture**: Domain-Driven Design (DDD) Rust backend ensuring strict separation of concerns (`api`, `core`, `models`, `db`).
- **Feature-based Frontend**: SvelteKit frontend strictly organized by feature domains.
- **Fast**: Rust backend, minimal JavaScript, lightning-fast execution.
- **Real-time**: High-performance WebSocket-based game updates synced via `DashMap` concurrency.
- **Secure Authentication**: Argon2 password hashing and highly secure, stateless JWT sessions.
- **Instant Matchmaking**: Background worker assigning queued players to real-time game sessions instantly.
- **Type Safety**: End-to-end type safety between Rust and TypeScript via `typeshare`.
- **Robust Integration Testing**: Mathematically proven stability with a 100% passing Playwright E2E test suite.

## Quick Start

### Docker (Recommended)

LajuChess uses a multi-container Docker setup for the application and PostgreSQL database.

```bash
# Start PostgreSQL Database and LajuChess Server
docker compose up -d --build
```

Visit: `http://localhost:3000`

### Local Development

```bash
# 1. Start Postgres Database via Docker
docker compose up -d postgres

# 2. Run the Rust Backend
DATABASE_URL=postgres://chess_user:chess_password@localhost:5432/lajuchess cargo run

# 3. Run the SvelteKit Frontend (in a new terminal)
cd frontend
npm install
npm run dev
```

## Architecture Stack

- **Backend**: Rust, Axum (HTTP + WebSockets), Tokio (Async), SQLx (Postgres), DashMap.
- **Frontend**: SvelteKit 5, TypeScript, Vite (HMR).
- **Testing**: Playwright (E2E Integration Testing).
- **Database**: PostgreSQL
- **Security**: Argon2, jsonwebtoken

## Testing

To ensure stability, always run the integration tests before a release:

```bash
cd frontend
npm run test:integration
```

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string (default: `postgres://chess_user:chess_password@postgres:5432/lajuchess`)
- `JWT_SECRET`: Secret key for JWT signing.
- `RUST_LOG`: Logging level (default: `info`)

## License

MIT
