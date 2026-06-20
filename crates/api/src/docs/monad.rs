use utoipa::OpenApi;
use crate::routes::monad::__path_monad_health;

#[derive(OpenApi)]
#[openapi(
    paths(monad_health),
    tags((name = "monad", description = "Monad staking routes"))
)]
pub struct MonadDoc;