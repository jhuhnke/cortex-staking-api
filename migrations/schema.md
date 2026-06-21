# Auth Schema

This schema supports Cortex-owned admin keys, partner organization keys, and future website-generated user keys.

## Design Summary

There are three major identity concepts:

```text
organizations
users
api_keys
```

Organizations are used for Cortex and partner companies.

Users are used for future website-generated self-serve API keys.

API keys can belong to either an organization or a user, but never both.

## Tables

---

# organizations

Represents Cortex or a partner organization.

## Columns

| Column       |          Type | Required | Description                          |
| ------------ | ------------: | -------: | ------------------------------------ |
| `id`         |        `UUID` |      Yes | Primary key.                         |
| `name`       |        `TEXT` |      Yes | Human-readable organization name.    |
| `kind`       |        `TEXT` |      Yes | Either `cortex` or `partner`.        |
| `status`     |        `TEXT` |      Yes | `active`, `suspended`, or `deleted`. |
| `created_at` | `TIMESTAMPTZ` |      Yes | Creation timestamp.                  |
| `updated_at` | `TIMESTAMPTZ` |      Yes | Last update timestamp.               |

## Rules

* Cortex-owned organizations use `kind = 'cortex'`.
* External partner organizations use `kind = 'partner'`.
* Only Cortex organizations should be allowed to create admin-capable keys.
* Organization names are unique case-insensitively.

---

# users

Represents a future self-serve website user.

A user may authenticate using email, wallet address, social sign-in, or a combination of those.

## Columns

| Column                    |          Type | Required | Description                                             |
| ------------------------- | ------------: | -------: | ------------------------------------------------------- |
| `id`                      |        `UUID` |      Yes | Primary key.                                            |
| `email`                   |        `TEXT` |       No | User email address.                                     |
| `wallet_address`          |        `TEXT` |       No | User wallet address.                                    |
| `social_provider`         |        `TEXT` |       No | Social login provider, such as `google` or `github`.    |
| `social_provider_user_id` |        `TEXT` |       No | Provider-specific user ID.                              |
| `status`                  |        `TEXT` |      Yes | `active`, `suspended`, or `deleted`.                    |
| `key_limit`               |     `INTEGER` |      Yes | Maximum number of active API keys this user may create. |
| `rate_limit_tier`         |        `TEXT` |      Yes | `free`, `paid`, or `internal`.                          |
| `created_at`              | `TIMESTAMPTZ` |      Yes | Creation timestamp.                                     |
| `updated_at`              | `TIMESTAMPTZ` |      Yes | Last update timestamp.                                  |

## Identity Rule

A user must have at least one of:

```text
email
wallet_address
social_provider + social_provider_user_id
```

This is enforced by:

```text
users_identity_required_chk
```

## User Key Limit Rule

Before creating a new user-owned API key, the API should check:

```sql
SELECT COUNT(*)
FROM api_keys
WHERE user_id = $1
  AND status = 'active';
```

The result must be less than:

```text
users.key_limit
```

Default user key limit:

```text
2
```

---

# api_keys

Represents an API key issued to either an organization or a user.

## Columns

| Column                  |          Type |    Required | Description                                  |
| ----------------------- | ------------: | ----------: | -------------------------------------------- |
| `id`                    |        `UUID` |         Yes | Primary key.                                 |
| `owner_type`            |        `TEXT` |         Yes | Either `organization` or `user`.             |
| `organization_id`       |        `UUID` | Conditional | Required when `owner_type = 'organization'`. |
| `user_id`               |        `UUID` | Conditional | Required when `owner_type = 'user'`.         |
| `name`                  |        `TEXT` |         Yes | Human-readable key name.                     |
| `key_prefix`            |        `TEXT` |         Yes | Public key prefix used for lookup/debugging. |
| `key_hash`              |        `TEXT` |         Yes | Hashed API key secret.                       |
| `status`                |        `TEXT` |         Yes | `active` or `revoked`.                       |
| `rate_limit_per_minute` |     `INTEGER` |         Yes | Per-key rate limit.                          |
| `last_used_at`          | `TIMESTAMPTZ` |          No | Last successful authentication timestamp.    |
| `expires_at`            | `TIMESTAMPTZ` |          No | Optional expiration timestamp.               |
| `revoked_at`            | `TIMESTAMPTZ` |          No | Revocation timestamp.                        |
| `created_at`            | `TIMESTAMPTZ` |         Yes | Creation timestamp.                          |
| `updated_at`            | `TIMESTAMPTZ` |         Yes | Last update timestamp.                       |

## Ownership Rule

An API key must belong to exactly one owner.

Valid ownership models:

| owner_type     | organization_id |  user_id |
| -------------- | --------------: | -------: |
| `organization` |        Required |     Null |
| `user`         |            Null | Required |

This is enforced by:

```text
api_keys_single_owner_chk
```

## Key Security Rules

* Plaintext API keys are never stored.
* `key_hash` stores the hashed secret.
* `key_prefix` is safe to log and display.
* Full API keys are shown only once at creation.
* Revoked keys must not authenticate.
* Expired keys must not authenticate.

---

# api_key_scopes

Represents permissions granted to an API key.

## Columns

| Column       |          Type | Required | Description                   |
| ------------ | ------------: | -------: | ----------------------------- |
| `api_key_id` |        `UUID` |      Yes | Foreign key to `api_keys.id`. |
| `scope`      |        `TEXT` |      Yes | Permission string.            |
| `created_at` | `TIMESTAMPTZ` |      Yes | Creation timestamp.           |

Primary key:

```text
(api_key_id, scope)
```

This prevents duplicate scopes on the same key.

---

# Authorization Model

Every authenticated request should resolve to an auth context:

```text
AuthContext
- api_key_id
- owner_type
- organization_id
- organization_kind
- user_id
- scopes
- rate_limit_per_minute
```

## Cortex Admin Keys

Cortex admin keys are organization-owned keys where:

```text
api_keys.owner_type = organization
organizations.kind = cortex
api_key_scopes includes admin:*
```

These keys can access:

```text
/admin/*
```

## Partner Keys

Partner keys are organization-owned keys where:

```text
api_keys.owner_type = organization
organizations.kind = partner
```

Partner keys cannot access Cortex admin routes.

Partner keys may receive scopes such as:

```text
monad:validators:read
monad:validators:write
monad:staking:write
monad:accounting:read
```

## User Keys

User keys are user-owned keys where:

```text
api_keys.owner_type = user
api_keys.user_id IS NOT NULL
```

User keys should be read-only by default.

Recommended user scopes:

```text
monad:validators:read
monad:rewards:read
monad:accounting:read
```

User keys should have strict rate limits.

Default:

```text
60 requests per minute
```

---

# Relationship Diagram

```text
organizations
  └── api_keys
        └── api_key_scopes

users
  └── api_keys
        └── api_key_scopes
```

---

# Initial Seed Strategy

Cortex keys should be seeded manually or through a controlled internal seed script.

Initial Cortex organization:

```text
name = Cortex Global
kind = cortex
status = active
```

Initial Cortex key:

```text
owner_type = organization
organization_id = Cortex organization ID
scope = admin:*
```

Only Cortex admin keys should be allowed to create partner organizations and partner API keys.

---

# Recommended Initial Scopes

## Admin

```text
admin:*
admin:organizations:read
admin:organizations:write
admin:keys:read
admin:keys:write
```

## Monad Partner

```text
monad:validators:read
monad:validators:write
monad:staking:write
monad:accounting:read
```

## User Read-Only

```text
monad:validators:read
monad:rewards:read
monad:accounting:read
```
