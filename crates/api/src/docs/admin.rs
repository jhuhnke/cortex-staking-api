use utoipa::OpenApi;
use crate::routes::admin::__path_admin_health;

#[derive(OpenApi)]
#[openapi(
    paths(admin_health),
    tags((name = "admin", description = "Cortex admin routes"))
)]
pub struct AdminDoc;