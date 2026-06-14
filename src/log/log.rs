use crate::{env::Env, log::LogLevel};
use serenity::{http::Http, model::webhook::Webhook};

pub struct Log {
    pub log_level: LogLevel,
    pub webhook_url: String,
    pub webhook_name: String,
    pub bot_token: String,
}

impl Log {
    pub fn new(env: &Env) -> Self {
        return Self {
            log_level: LogLevel::from(env.log_level.as_str()),
            bot_token: env.bot_token.clone(),
            webhook_url: env.webhook_url.clone(),
            webhook_name: env.webhook_name.clone(),
        };
    }

    async fn send_webhook_message(
        message: String,
        bot_token: String,
        webhook_url: String,
        webhook_name: String,
    ) {
        let http = Http::new(bot_token.as_str());
        let webhook = match Webhook::from_url(&http, webhook_url.as_str()).await {
            Ok(webhook) => webhook,
            Err(err) => {
                return eprintln!("Log webhook error: {}", err);
            }
        };

        match webhook
            .execute(&http, true, |w| {
                w.content(message);
                w.username(webhook_name.as_str())
            })
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return eprintln!("Log webhook error: {}", err);
            }
        };
    }

    pub fn log(&self, level: LogLevel, msg: String) {
        let message = format!("{} {}", level.to_log_prefix(), msg);
        if level <= self.log_level {
            if level == LogLevel::Error {
                let webhook_url = self.webhook_url.clone();
                let webhook_name = self.webhook_name.clone();
                let bot_token = self.bot_token.clone();
                tokio::spawn(async move {
                    Log::send_webhook_message(msg, bot_token, webhook_url, webhook_name).await;
                });
                eprintln!("{}", message);
            } else {
                println!("{}", message);
            }
        }
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::Error, msg.to_string());
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg.to_string());
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::Info, msg.to_string());
    }

    #[allow(dead_code)]
    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg.to_string());
    }
}
