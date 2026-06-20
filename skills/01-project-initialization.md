# 01 — Rust Project Initialization

## Goal

Initialize Cortex staking as a Rust workspace, not a single monolithic crate.

The project should start small but make room for future chains.

## Recommended commands

```bash
mkdir cortex-staking-api
cd cortex-staking-api

git init
cargo new crates/api --bin
cargo new crates/core --lib
cargo new crates/auth --lib
cargo new crates/db --lib
mkdir -p crates/chains
cargo new crates/chains/monad --lib
mkdir migrations
mkdir skills
```

Create a root `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/api",
    "crates/core",
    "crates/auth",
    "crates/db",
    "crates/chains/monad",
]
resolver = "2"

[workspace.package]
edition = "2024"
version = "0.1.0"
license = "UNLICENSED"

[workspace.dependencies]
actix-web = "4"
actix-cors = "0.7"
anyhow = "1"
argon2 = "0.5"
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
secrecy = "0.10"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "json"] }
thiserror = "2"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
uuid = { version = "1", features = ["serde", "v4"] }
utoipa = "5"
utoipa-swagger-ui = { version = "9", features = ["actix-web"] }
```

## Initial crate responsibilities

```text
crates/api
  Actix server, app state, route registration, middleware, OpenAPI.

crates/core
  Shared domain types, errors, request context, common response models.

crates/auth
  API key hashing, auth middleware, scopes, organization access checks.

crates/db
  Database pool, migrations, repository helpers.

crates/chains/monad
  Monad-specific routes, services, RPC clients, request/response models.
```

## Rule

Do not put business logic directly in Actix handlers. Handlers should validate input, enforce auth, call services, and return responses.
