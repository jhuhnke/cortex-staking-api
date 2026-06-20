# Cortex Staking API Skills

This folder defines the engineering rules for building the Cortex staking API from scratch.

The API starts with:

- Admin routes
- Monad routes
- API key protection
- Cortex/admin keys
- Non-Cortex/partner keys

The architecture must remain ready for a multi-chain future.

## Skill files

1. `01-project-initialization.md` — how to initialize the Rust workspace.
2. `02-api-architecture.md` — service/module layout and routing philosophy.
3. `03-auth-and-api-keys.md` — Cortex vs partner API key model.
4. `04-routes-and-scopes.md` — route naming and scope requirements.
5. `05-chain-adapter-design.md` — how to add Monad now and more chains later.
6. `06-database-design.md` — DB schema and repository rules.
7. `07-error-handling.md` — consistent error model.
8. `08-observability-and-logging.md` — logs, tracing, metrics, and audit trail.
9. `09-security-rules.md` — security baseline for staking APIs.
10. `10-testing-and-ci.md` — test and CI expectations.

## Core principle

Build around this hierarchy:

```text
Organization
  → API Key
    → Scope
      → Chain
        → Operation
```

Do not build around isolated network-specific services with copied authentication logic.
