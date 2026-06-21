# Cortex Staking API

A Rust-based staking platform developed by Cortex Global.

The long-term goal is to provide a secure, scalable, and multi-chain staking API that supports validator operations, staking workflows, accounting, and infrastructure management across multiple blockchain networks.

The initial release focuses on:

* Cortex administrative operations
* Monad staking operations
* API key authentication and authorization
* OpenAPI / Swagger-first development

---

## Documentation

Project documentation:

* `README.md` - Project overview and architecture
* `CONTRIBUTING.md` - Development workflow and contribution requirements
* `migrations/schema.md` - Database schema documentation
* `skills/` - Architecture and design standards

All contributors should review `CONTRIBUTING.md` before submitting changes.

---

# Current Status

## Implemented

* Rust workspace architecture
* Modular crate structure
* Actix Web API server
* Shared application state (`AppState`)
* SQLx PostgreSQL integration
* Dockerized local PostgreSQL environment
* Route grouping and isolation

  * `/healthz`
  * `/readyz`
  * `/admin/*`
  * `/monad/*`
* API key authentication framework
* Organization-based authorization model

  * Cortex organizations
  * Partner organizations
  * User accounts
* Scope-based authorization model

  * `admin`
  * `read`
  * `write`
* Database schema and migrations

  * Organizations
  * Users
  * API Keys
  * API Key Scopes
* Database-backed API key validation
* Cortex admin route protection
* Partner route protection
* Authentication and authorization test coverage
* OpenAPI / Swagger integration
* Project design standards and architecture documentation
* Local development tooling and seed data

## In Progress

* Organization management APIs
* API key management APIs
* API key creation and rotation workflows
* API key revocation workflows
* Repository pattern implementation
* Monad service layer architecture
* Request rate limiting
* Audit logging foundation
* Improved integration test coverage

## Planned

### Authentication & Administration

* Organization onboarding workflows
* User self-service API key generation
* Role and permission management
* API usage analytics
* Audit logging and compliance reporting

### Monad

* Validator discovery
* Validator health monitoring
* Staking operations
* Unstaking operations
* Reward reporting
* Accounting endpoints

### Multi-Chain Expansion

* Ethereum support
* Bitcoin support
* Additional staking networks
* Chain-agnostic service interfaces

### Platform Services

* Validator accounting engine
* Scheduled background jobs
* Infrastructure monitoring
* Metrics and alerting
* Distributed caching
* Production deployment pipeline
* CI/CD automation
* Disaster recovery procedures

### Future Products

* Partner dashboard
* User dashboard
* Billing and subscription management
* Public developer portal
* Self-service API access
* Usage-based rate limiting and quotas

---

# Architecture

```text
cortex-staking-api/
│
├── crates/
│   ├── api/
│   │   ├── routes/
│   │   ├── docs/
│   │   ├── app.rs
│   │   ├── config.rs
│   │   ├── state.rs
│   │   └── main.rs
│   │
│   ├── core/
│   │   ├── errors/
│   │   ├── models/
│   │   └── traits/
│   │
│   ├── auth/
│   │   ├── middleware/
│   │   ├── extractors/
│   │   └── scopes/
│   │
│   ├── db/
│   │   ├── repositories/
│   │   └── migrations/
│   │
│   └── chains/
│       └── monad/
│           ├── handlers/
│           ├── services/
│           ├── repositories/
│           └── rpc/
│
├── migrations/
├── skills/
├── Cargo.toml
└── README.md
```

---

# Route Structure

## Public Health Routes

```text
GET /healthz
```

Used for health checks, uptime monitoring, and load balancer validation.

---

## Admin Routes

Administrative routes intended for Cortex-owned API keys.

```text
GET  /admin/health

POST /admin/organizations
POST /admin/api-keys
POST /admin/api-keys/{id}/rotate
POST /admin/api-keys/{id}/revoke

GET  /admin/audit-log
```

---

## Monad Routes

Partner-facing Monad staking functionality.

```text
GET  /monad/health

GET  /monad/validators
GET  /monad/validators/{id}

POST /monad/stake
POST /monad/unstake

GET  /monad/rewards
GET  /monad/accounting
```

---

# Authentication Model

Every request must be authenticated using an API key.

API keys belong to an organization.

Organizations are classified as:

```text
Cortex
Partner
```

Each API key is granted one or more scopes.

Examples:

```text
admin:*
admin:keys:write
admin:keys:read

monad:validators:read
monad:validators:write

monad:staking:write
monad:accounting:read
```

Requests are evaluated against scopes before reaching handlers.

---

# Security Principles

* API keys are stored as hashes
* Plaintext keys are never persisted
* Keys are shown only once at creation
* Authorization is scope-based
* No secrets are written to logs
* No private keys are stored by the API
* External requests must use timeouts
* Authentication and authorization occur before handler execution

---

# OpenAPI Documentation

Swagger documentation is generated using:

* utoipa
* utoipa-swagger-ui

Separate documentation sets are maintained for each route group.

```text
/swagger-ui/
```

Available specifications:

```text
Health
Admin
Monad
```

OpenAPI endpoints:

```text
/api-docs/health/openapi.json
/api-docs/admin/openapi.json
/api-docs/monad/openapi.json
```

Every route should be documented at the time it is created.

---

# Local Development

Run the API:

```bash
cargo run -p cortex-staking-api
```

Health check:

```bash
curl http://127.0.0.1:8080/healthz
```

Admin health:

```bash
curl http://127.0.0.1:8080/admin/health
```

Monad health:

```bash
curl http://127.0.0.1:8080/monad/health
```

Swagger UI:

```text
http://127.0.0.1:8080/swagger-ui/
```

---

# Design Principles

## Keep main.rs Small

`main.rs` is responsible only for:

* Loading configuration
* Initializing logging
* Starting the HTTP server

No routes, handlers, business logic, or database code should be placed in `main.rs`.

---

## Thin Handlers

Handlers should:

* Validate requests
* Call services
* Return responses

Handlers should not contain business logic.

---

## Service-Oriented Business Logic

Business rules belong in services.

Examples:

```text
Create Stake
Calculate Rewards
Validate Withdrawal
Generate Accounting Records
```

---

## Repository Pattern

Database access belongs in repositories.

Handlers and services should not execute raw SQL directly.

---

## Chain Isolation

Each blockchain implementation should remain isolated.

Chain-specific logic belongs inside:

```text
chains/<chain>/
```

The goal is to make future support for:

* Ethereum
* Solana
* Sui
* Bitcoin
* Other networks

possible without modifying core platform logic.

---

# Next Milestone

Build the authentication foundation:

1. Shared application state
2. Organizations table
3. API keys table
4. API key middleware
5. Scope authorization
6. Admin vs Partner route protection

Success criteria:

* Cortex keys can access admin routes
* Partner keys cannot access admin routes
* Partner keys can access Monad routes
* Missing or invalid keys return 401
* Insufficient scopes return 403
