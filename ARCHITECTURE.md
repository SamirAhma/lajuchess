# LajuChess Architecture: Technical Deep Dive

This document provides a detailed breakdown of the LajuChess architecture, written specifically for senior developers familiar with modern frontend ecosystems (React/TypeScript) and distributed systems.

---

## High-Level Overview

LajuChess is a real-time, multi-user chess platform built for high concurrency and low latency. The stack effectively separates **heavy-duty game logic and state persistence** (Rust) from the **reactive UI** (SvelteKit).

### Tech Stack
- **Backend**: Rust (Axum, Tokio, SQLx, DashMap, Shakmaty)
- **Frontend**: SvelteKit 5 (Runes), TypeScript, Tailwind-ready CSS
- **Database**: PostgreSQL (Persistence) + In-memory DashMap (Active Sessions)
- **Communication**: REST (Auth/Matchmaking) + WebSockets (In-game)
- **DevOps**: Docker Multi-stage builds, Playwright E2E

---

## 1. Backend Architecture: Domain-Driven Design (DDD)

As a React developer used to "Services" or "Reducers," think of the Rust backend as a set of highly concurrent state machines.

### Domain Separation
To avoid a massive `main.rs`, we use a DDD-lite structure:
- **`api/`**: The "Controller" layer. Handles routing, WebSocket upgrades, and JSON deserialization.
- **`core/`**: The "Service" layer. Contains pure business logic like JWT token generation and the background **Matchmaking Loop**.
- **`models/`**: The "Data Layer" / DTOs. Defines both the database schema and the messages sent over the wire.
- **`db/`**: The "Repository" layer. Encapsulates SQLx queries and Postgres interactions.

### Concurrency Model
The backend must handle thousands of games. Instead of a standard global lock, we use:
- **`DashMap<Uuid, GameSession>`**: A concurrent hash map that allows thread-safe access to active games without locking the entire system.
- **`Arc<Mutex<T>>`**: Used for granular state locking (e.g., inside a single game session).
- **Background Workers**: The matchmaking engine runs in a dedicated `tokio::spawn` loop, pairing players asynchronously from a global queue.

---

## 2. Frontend Architecture: Feature-Based SvelteKit

If you're coming from React 18+, Svelte 5 will feel very familiar due to its **Runes** system (`$state`, `$derived`, `$effect`), which mirrors `useState`, `useMemo`, and `useEffect` but with less boilerplate.

### Feature Folders (`frontend/src/lib/`)
We organize code by feature rather than type to minimize "folder jumping":
- **`api/`**: Strictly typed API clients (wrappers for `fetch`).
- **`core/`**: Infrastructure logic, specifically the `ws.ts` manager which handles the socket lifecycle.
- **`stores/`**: Svelte stores (like `authStore`) which function similarly to a persisted **React Context** but are accessible outside the component tree.
- **`components/`**: Divided into `ui` (atomic elements) and `board` (the complex game engine UI).

### State Management
- **Local State**: Handled via Svelte Runes.
- **Global State**: The `auth` store manages the user session.
- **Real-time State**: The WebSocket stream feeds a writable store in `ws.ts`. When a message arrives, it pushes data into the store, triggering reactive UI updates on the chessboard.

---

## 3. Real-Time Synchronization

The "Heart" of the app is the WebSocket bridge. 

1. **The Handshake**: Client requests `GET /ws/:id`. Axum validates the JWT.
2. **The Stream**: We use **Tokio Broadcast Channels**.
   - When Player A makes a move, the backend processes it via `shakmaty`.
   - The updated state is broadcast to all subscribers of that specific `game_id`.
   - The frontend receives a `ServerMessage::State(GameState)` and updates the board SVG.

---

## 4. End-to-End Type Safety (The Secret Sauce)

One of the biggest pain points in React/Node development is keeping types in sync. We solve this with **Typeshare**.

- **Rust Input**: We annotate Rust structs with `#[typeshare]`.
- **Generation**: A CLI tool scans the Rust `models/` folder and generates a `types.ts` file in the frontend.
- **Result**: If you change a field name in the Rust backend, the frontend TypeScript build will immediately fail. It provides "Full Stack" type safety.

---

## 5. Infrastructure & Deployment

- **Database**: PostgreSQL handles persistent user records and game history.
- **Containerization**: The `Dockerfile` is a triple-stage build:
  1. Build frontend assets (Node).
  2. Build backend binary (Rust).
  3. Final minimal image (Debian-slim) containing only the binary and the `public/` assets.
- **E2E Testing**: Playwright runs against the actual Docker composition to simulate real-user pairing and gameplay.

---

## Why this is better than a standard MERN stack?
1. **Performance**: Rust uses ~15MB of RAM where Node might use 100MB+.
2. **Reliability**: The Rust compiler prevents most runtime crashes (null pointers, race conditions) before they happen.
3. **Developer Experience**: Svelte 5's reactivity is more intuitive than React's hooks, resulting in ~30% less code for the same UI complexity.
