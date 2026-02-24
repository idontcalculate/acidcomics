# AcidComics™ Backend (Rust + Axum + Async-GraphQL + Postgres + SQLx)

Backend API for AcidComics™ — a Rust service with GraphQL, JWT auth, Postgres, SQLx (offline cache), and Docker Compose.

## Stack

- Rust (Axum)
- async-graphql (+ async-graphql-axum)
- Postgres 16
- SQLx (offline mode supported via `.sqlx/`)
- Docker / Docker Compose
- GitHub Actions CI (fmt + clippy + tests + docker build)

---

## Prerequisites

- Rust toolchain (stable)
- Docker (and Docker Compose)
- `sqlx-cli` (optional but recommended)

Install sqlx-cli:
```bash
cargo install sqlx-cli --no-default-features --features postgres