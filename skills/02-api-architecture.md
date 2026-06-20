# 02 — API Architecture

## Goal

Build one Cortex staking API platform with chain modules.

Avoid separate network services unless there is a strong operational reason.

## Recommended top-level route groups

```text
/healthz
/version

/admin/*
/monad/*
```

Future chains should follow the same pattern:

```text
/ethereum/*
/solana/*
/sui/*
/cosmos/*
```

## Recommended internal structure

```text
crates/api/src
  main.rs
  state.rs
  routes.rs
  middleware.rs

crates/chains/monad/src
  routes.rs
  handlers.rs
  service.rs
  repository.rs
  rpc.rs
  models.rs
```

## Handler pattern

Handlers should be thin:

```text
1. Extract auth context
2. Validate request
3. Call service
4. Map domain result to HTTP response
```

## Service pattern

Services contain business logic:

```text
- staking operation rules
- organization ownership checks
- chain-specific validation
- transaction/accounting side effects
```

## Repository pattern

Repositories contain SQL and database access only.

Do not mix SQL into route handlers.

## Rule

A future chain should be added by creating a new chain module, not by copying an entire API service.
