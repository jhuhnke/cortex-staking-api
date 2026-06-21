use crate::routes::monad::__path_monad_health;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(monad_health),
    tags((name = "monad", description = "Monad staking routes"))
)]
pub struct MonadDoc;
