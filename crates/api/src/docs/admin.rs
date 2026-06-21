use crate::routes::admin::__path_admin_health;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(admin_health),
    tags((name = "admin", description = "Cortex admin routes"))
)]
pub struct AdminDoc;
