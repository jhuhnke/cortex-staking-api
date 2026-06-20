use utoipa::OpenApi;
use crate::routes::health::__path_healthz;

#[derive(OpenApi)]
#[openapi(
    paths(healthz),
    tags((name = "health", description = "Service health routes"))
)]
pub struct HealthDoc;