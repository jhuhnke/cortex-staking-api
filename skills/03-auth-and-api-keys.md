# 03 — Auth and API Keys

## Goal

Use API keys with explicit organization ownership and scopes.

There are two key categories:

```text
Cortex keys      → admin/internal operations
Partner keys     → non-Cortex customer and staking operations
```

## Required auth context

Every authenticated request should resolve to:

```rust
pub struct AuthContext {
    pub api_key_id: uuid::Uuid,
    pub organization_id: uuid::Uuid,
    pub organization_kind: OrganizationKind,
    pub scopes: Vec<String>,
}
```

## Organization kinds

```rust
pub enum OrganizationKind {
    Cortex,
    Partner,
}
```

## API key storage

Never store plaintext API keys.

Store:

```text
api_key_id
organization_id
key_prefix
key_hash
status
environment
created_at
revoked_at
last_used_at
```

Show the full secret only once at creation or rotation.

## Token format

Use a recognizable format:

```text
ctx_live_<prefix>.<secret>
ctx_test_<prefix>.<secret>
```

## Scope examples

```text
admin:*
admin:keys:read
admin:keys:write
admin:organizations:write
admin:audit:read

monad:validators:read
monad:validators:write
monad:staking:write
monad:rewards:read
monad:accounting:read
```

## Rules

- Cortex keys may receive admin scopes.
- Partner keys must never receive `admin:*`.
- A route must declare its required scope.
- Authentication and authorization must be centralized middleware/extractors.
- Do not duplicate API key checks inside individual handlers.
