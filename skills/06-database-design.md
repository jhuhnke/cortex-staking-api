# 06 — Database Design

## Goal

Make organization ownership and auditability first-class.

## Required tables

```text
organizations
api_keys
api_key_scopes
audit_log
customers
monad_validators
staking_operations
chain_transactions
```

## Organization ownership

Every partner-visible row should include:

```text
organization_id
```

This includes:

```text
customers
validators
staking operations
transactions
accounting records
```

## API key table

```sql
CREATE TABLE api_keys (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL REFERENCES organizations(id),
    key_prefix TEXT NOT NULL UNIQUE,
    key_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    environment TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ
);
```

## Repository rules

- Handlers must not contain raw SQL.
- Services call repositories.
- Repositories return domain models or typed database errors.
- Use transactions for multi-step staking/accounting writes.

## SQLx rule

During early development, `sqlx::query` plus explicit mapping is acceptable.

Before production, add:

```bash
cargo sqlx prepare --workspace
```

and enforce query checking in CI.
