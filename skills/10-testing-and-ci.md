# 10 — Testing and CI

## Goal

Catch auth, routing, database, and chain-specific bugs before deployment.

## Required tests

```text
Auth middleware tests
Scope enforcement tests
Cortex admin key access tests
Partner key denial tests
Organization isolation tests
Monad route handler tests
Repository tests
Error response tests
```

## Auth test examples

- Missing API key returns 401.
- Invalid API key returns 401.
- Revoked API key returns 401.
- Partner key cannot call `/admin/*`.
- Cortex key with missing scope cannot call restricted admin route.
- Partner key can only read its own organization data.

## CI steps

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo sqlx prepare --workspace --check
```

## Local development

Use Docker Compose for local Postgres.

```bash
docker compose up -d postgres
sqlx migrate run
cargo run -p api
```

## Rule

Do not merge new routes without tests proving the correct key type and scope behavior.
