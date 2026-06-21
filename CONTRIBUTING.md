# Contributing

Thank you for contributing to Cortex Staking API.

## Branching Strategy

The `main` branch is protected.

All changes must be submitted through a Pull Request.

Direct commits to `main` are not permitted.

## Pull Request Requirements

Before merging:

* All tests must pass.
* `cargo check` must pass.
* `cargo test` must pass.
* Database migrations must be reviewed.
* OpenAPI documentation must be updated if routes change.
* Version numbers must be updated according to Semantic Versioning.

A review from **jhuhnke** is required before merging into `main`.

---

# Semantic Versioning

The project follows:

```text
MAJOR.MINOR.PATCH
```

Examples:

```text
0.1.0
0.2.0
1.0.0
1.1.1
```

## Major Version

Increment when:

* Existing API routes change behavior
* Request schemas change incompatibly
* Response schemas change incompatibly
* Authentication behavior changes incompatibly

Example:

```text
1.2.0 -> 2.0.0
```

## Minor Version

Increment when:

* New routes are added
* New blockchain support is added
* New features are added
* Changes are backwards compatible

Example:

```text
1.2.0 -> 1.3.0
```

## Patch Version

Increment when:

* Bugs are fixed
* Tests are added
* Documentation changes
* Refactors
* Dependency updates
* Performance improvements

Example:

```text
1.2.0 -> 1.2.1
```

---

# Local Development

## Prerequisites

* Rust
* Docker Desktop
* SQLx CLI

---

## Windows

Start Postgres:

```powershell
docker compose -f docker-compose.local.yml up -d
```

Run migrations:

```powershell
sqlx migrate run
```

Run development seed:

```powershell
Get-Content dev\seed_dev.sql | docker exec -i cortex-staking-postgres psql -U cortex -d cortex_staking_api
```

Run the API:

```powershell
cargo run -p cortex-staking-api
```

Run tests:

```powershell
cargo test
```

---

## macOS / Linux

Start Postgres:

```bash
docker compose -f docker-compose.local.yml up -d
```

Run migrations:

```bash
sqlx migrate run
```

Run development seed:

```bash
docker exec -i cortex-staking-postgres psql -U cortex -d cortex_staking_api < dev/seed_dev.sql
```

Run the API:

```bash
cargo run -p cortex-staking-api
```

Run tests:

```bash
cargo test
```

---

# Development Workflow

1. Create feature branch
2. Implement changes
3. Add or update tests
4. Update OpenAPI documentation
5. Update version number
6. Open Pull Request
7. Request review from jhuhnke
8. Merge into main after approval
