use crate::config::Config;
use reqwest::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    pub http_client: Client,
}

impl AppState {
    pub fn new(config: Config, db: PgPool, http_client: Client) -> Self {
        Self {
            config,
            db,
            http_client,
        }
    }
}
