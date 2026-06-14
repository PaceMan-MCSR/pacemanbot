use std::{env, error::Error};

use dotenv::dotenv;

pub struct Env {
    pub bot_token: String,
    pub ws_host: String,
    pub ws_url: String,
    pub api_auth_key: String,
    pub webhook_url: String,
    pub webhook_name: String,
    pub log_level: String,
}

impl Env {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let bot_token = match env::var("BOT_TOKEN") {
            Ok(token) => token,
            Err(e) => {
                return Err(format!("Expected BOT_TOKEN: {}", e).into());
            }
        };
        let ws_host = env::var("WS_HOST").unwrap_or("paceman.gg:8081".to_string());
        let ws_url = env::var("WS_URL").unwrap_or("wss://paceman.gg/ws".to_string());
        let api_auth_key = match env::var("API_AUTH_KEY") {
            Ok(key) => key,
            Err(e) => {
                return Err(format!("Expected API_AUTH_KEY: {}", e).into());
            }
        };
        let webhook_url = env::var("WEBHOOK_URL").unwrap_or("".to_string());
        let webhook_name = env::var("WEBHOOK_NAME").unwrap_or("pacemanbotlogs".to_string());
        let log_level = env::var("LOG_LEVEL").unwrap_or("info".to_string());

        Ok(Self {
            bot_token,
            ws_host,
            ws_url,
            api_auth_key,
            webhook_url,
            webhook_name,
            log_level,
        })
    }
}
