#[derive(Debug, Clone)]
pub struct Config {
    pub bind_address: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bind_address: std::env::var("BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        }
    }
}
