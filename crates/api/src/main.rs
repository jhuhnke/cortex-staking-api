mod app;
mod config;
mod routes;
mod docs;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().init();

    let config = Config::from_env();
    let bind_address = config.bind_address.clone();

    tracing::info!("starting cortex staking api on {}", bind_address);

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .configure(app::configure_app)
    })
    .bind(bind_address)?
    .run()
    .await
}
