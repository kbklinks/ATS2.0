use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a number");
        Self { database_url, port }
    }
}
