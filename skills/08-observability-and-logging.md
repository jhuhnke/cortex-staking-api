# 08 — Observability and Logging

## Goal

Make the API debuggable without leaking sensitive staking data.

## Required request fields

Every request log should include:

```text
request_id
method
path
status
latency_ms
organization_id
api_key_id
chain
route_name
```

## Never log

```text
API keys
private keys
seed phrases
raw signed transactions
authorization headers
full upstream response bodies
full request bodies containing sensitive data
```

## Audit log events

Write audit log rows for:

```text
API key created
API key rotated
API key revoked
organization created
admin route accessed
staking operation created
validator created or updated
```

## Recommended tracing setup

Use `tracing` and structured JSON logs.

```rust
tracing_subscriber::fmt()
    .json()
    .with_env_filter("info")
    .init();
```

## Rule

Operational logs are for debugging. Audit logs are for accountability. Do not treat them as the same thing.
