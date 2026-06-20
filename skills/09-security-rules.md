# 09 — Security Rules

## Goal

Protect partner data, Cortex admin operations, and staking workflows.

## API key security

- Hash API keys before storage.
- Show full API key only once.
- Support key rotation.
- Support key revocation.
- Track `last_used_at`.
- Store key prefix separately for lookup and support.

## Authorization security

- Every route must declare a required scope.
- Admin routes require Cortex organization kind plus admin scope.
- Partner routes must enforce organization ownership.
- Never trust organization IDs supplied by the client unless authorized.

## Network security

- Use request timeouts for all HTTP/RPC calls.
- Use connection pooling.
- Add rate limiting per API key.
- Add request body size limits.
- Disable debug routes in production.

## Data security

- Redact secrets in logs.
- Do not store private keys unless the product explicitly requires custody.
- If custody is ever added, isolate it into a separate service and security model.

## Production rule

If a route can move funds, create staking transactions, or change validator state, it must have:

```text
explicit scope
organization ownership check
request audit log
idempotency key support
rate limit
```
