use crate::routes::health::__path_healthz;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(healthz),
    tags((name = "health", description = "Service health routes"))
)]
pub struct HealthDoc;
