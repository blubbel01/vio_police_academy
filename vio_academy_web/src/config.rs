use std::env;
use dotenvy::dotenv;

pub struct Config {
    pub oauth_app_id: String,
    pub oauth_app_secret: String,
    pub oauth_auth_url: String,
    pub oauth_token_url: String,
    pub database_url: String,
    pub base_url: String,
}

impl Config {
    pub fn new(oauth_app_id: String, oauth_app_secret: String, oauth_auth_url: String, oauth_token_url: String, database_url: String, base_url: String) -> Self {
        Self { oauth_app_id, oauth_app_secret, oauth_auth_url, oauth_token_url, database_url, base_url }
    }

    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            oauth_app_id: env::var("OAUTH_APP_ID").unwrap(),
            oauth_app_secret: env::var("OAUTH_APP_SECRET").unwrap(),
            oauth_auth_url: env::var("OAUTH_AUTH_URL").unwrap(),
            oauth_token_url: env::var("OAUTH_TOKEN_URL").unwrap(),
            database_url: env::var("DATABASE_URL").unwrap(),
            base_url: env::var("BASE_URL").unwrap(),
        }
    }
}