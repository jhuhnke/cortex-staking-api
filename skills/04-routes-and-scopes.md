# 04 — Routes and Scopes

## Goal

Make route access obvious and enforceable.

## Admin routes

Admin routes are Cortex-only.

```text
POST /admin/organizations              admin:organizations:write
GET  /admin/organizations              admin:organizations:read
POST /admin/api-keys                   admin:keys:write
GET  /admin/api-keys                   admin:keys:read
POST /admin/api-keys/{id}/rotate       admin:keys:write
POST /admin/api-keys/{id}/revoke       admin:keys:write
GET  /admin/audit-log                  admin:audit:read
```

## Monad routes

Monad routes are chain-specific and may be used by partners when scoped.

```text
GET  /monad/validators                 monad:validators:read
GET  /monad/validators/{id}            monad:validators:read
POST /monad/validators                 monad:validators:write
POST /monad/stake                      monad:staking:write
POST /monad/unstake                    monad:staking:write
GET  /monad/rewards                    monad:rewards:read
GET  /monad/accounting                 monad:accounting:read
```

## Route design rules

- Use nouns for resources.
- Use verbs only for explicit actions like `rotate`, `revoke`, `stake`, or `unstake`.
- Keep admin and partner routes separate.
- Do not expose admin behavior through chain routes.
- Always include organization ownership checks for partner-visible resources.

## Response shape

Prefer consistent JSON responses:

```json
{
  "data": {},
  "request_id": "..."
}
```

For errors:

```json
{
  "error": {
    "code": "forbidden",
    "message": "Missing required scope",
    "request_id": "..."
  }
}
```
