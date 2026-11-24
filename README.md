# Backend Template

A Rust backend scaffolding built with **Axum**, **SQLx**, and **PostgreSQL**.  
Intended as a starting point for REST APIs with a clean layered architecture:

```
routes → services → repository → models
```

## Features

- **Framework**: Axum 0.8 (routing, JSON extraction, state)
- **Database**: PostgreSQL via SQLx 0.9 with runtime migrations
- **Async runtime**: Tokio
- **Serialization**: Serde + serde_json
- **Configuration**: environment variables via `dotenvy`
- **Logging**: `tracing` + `tracing-subscriber` (level controlled by `RUST_LOG`)
- **Error handling**: custom `AppError` enum mapped to HTTP status codes
- **Migrations**: versioned SQL migrations under `migrations/`
- **Container ready**: `docker-compose.yml` for PostgreSQL
- **Testing**: unit tests + integration tests, GitHub Actions CI

## Project structure

```
src/
├── main.rs          # Application entry point
├── config.rs        # Environment configuration
├── state.rs         # Shared application state
├── error.rs         # AppError enum and IntoResponse impl
├── routes/
│   ├── mod.rs       # Router composition
│   ├── users.rs     # User routes
│   └── posts.rs     # Post routes
├── services/
│   ├── mod.rs
│   ├── user_service.rs
│   └── post_service.rs
├── repository/
│   ├── mod.rs
│   ├── user_repository.rs
│   └── post_repository.rs
├── models/
│   ├── mod.rs
│   ├── user.rs
│   └── post.rs
└── dto/
    ├── mod.rs
    ├── user.rs
    └── post.rs

migrations/
├── 0001_users.sql
└── 0002_posts.sql
```

## Getting started

### Prerequisites

- Rust toolchain (stable)
- PostgreSQL 16 (or use Docker Compose)

### 1. Configure environment

Copy the example file and adjust values:

```bash
cp .env.example .env
```

`.env.example`:

### 2. Start the database

```bash
docker compose up -d
```

This starts a PostgreSQL 16 container on port `5432`.

### 3. Run the server
```bash
cargo run
```
The server listens on `0.0.0.0:3000` (or the `PORT`/`ADDRESS` from `.env`).  
SQLx migrations run automatically on startup.

## License
MIT