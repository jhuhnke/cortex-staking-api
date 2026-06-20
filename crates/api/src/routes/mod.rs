pub mod admin;
pub mod health;
pub mod monad;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(health::configure)
        .service(web::scope("/admin").configure(admin::configure))
        .service(web::scope("/monad").configure(monad::configure));
}
