# 05 — Chain Adapter Design

## Goal

Support Monad now without blocking a multi-chain future.

## Principle

The API core should understand generic staking concepts. Chain modules should understand chain-specific implementation details.

## Generic concepts

```text
Validator
StakeOperation
UnstakeOperation
Rewards
AccountingRecord
ChainTransaction
```

## Chain-specific concepts

Examples:

```text
Monad validator identifier
Monad RPC request format
Monad staking transaction format
Monad reward/accounting calculation
```

## Suggested trait

```rust
#[async_trait::async_trait]
pub trait ChainAdapter {
    type StakeRequest;
    type StakeResponse;
    type UnstakeRequest;
    type UnstakeResponse;

    async fn create_stake(
        &self,
        ctx: &AuthContext,
        req: Self::StakeRequest,
    ) -> Result<Self::StakeResponse, ApiError>;

    async fn create_unstake(
        &self,
        ctx: &AuthContext,
        req: Self::UnstakeRequest,
    ) -> Result<Self::UnstakeResponse, ApiError>;
}
```

## Monad module rule

Monad may expose Monad-specific request/response models, but it should not define global auth, global error handling, global database pool creation, or global server configuration.

## Future-chain rule

A future chain should be added by creating:

```text
crates/chains/<chain>/src/routes.rs
crates/chains/<chain>/src/service.rs
crates/chains/<chain>/src/repository.rs
crates/chains/<chain>/src/rpc.rs
crates/chains/<chain>/src/models.rs
```

Do not copy/paste the Monad module and then edit names by hand. Extract shared abstractions first.
