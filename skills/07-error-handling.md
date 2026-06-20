# 07 — Error Handling

## Goal

Return consistent, safe, useful errors.

## Error categories

```text
bad_request
unauthorized
forbidden
not_found
conflict
rate_limited
upstream_timeout
upstream_error
internal_error
```

## Rules

- Do not expose secrets in errors.
- Do not expose raw upstream response bodies by default.
- Do not use `unwrap` or `expect` in request paths.
- Convert internal errors into stable public error codes.
- Include `request_id` in every error response.

## Example response

```json
{
  "error": {
    "code": "forbidden",
    "message": "Missing required scope: monad:staking:write",
    "request_id": "req_abc123"
  }
}
```

## Rust pattern

Use a shared API error type:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("not found")]
    NotFound,

    #[error("upstream timeout")]
    UpstreamTimeout,

    #[error("internal error")]
    Internal,
}
```

Implement Actix `ResponseError` once in the API/core layer.
