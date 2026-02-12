use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set (check .env)");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set (check .env)");

        Self {
            database_url,
            jwt_secret,
        }
    }
}
