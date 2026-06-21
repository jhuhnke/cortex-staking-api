use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::docs::{admin::AdminDoc, health::HealthDoc, monad::MonadDoc};
use crate::routes;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::configure)
        .service(SwaggerUi::new("/documentation/{_:.*}").urls(vec![
            (
                Url::with_primary("Health", "/api-docs/health/openapi.json", true),
                HealthDoc::openapi(),
            ),
            (
                Url::new("Admin", "/api-docs/admin/openapi.json"),
                AdminDoc::openapi(),
            ),
            (
                Url::new("Monad", "/api-docs/monad/openapi.json"),
                MonadDoc::openapi(),
            ),
        ]));
}
