use std::env;

use serenity::{http::Http, model::webhook::Webhook};

pub async fn send_webhook_message(message: String) {
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    let webhook_url = env::var("WEBHOOK_URL").expect("Expected a webhook url in the environment");
    let http = Http::new(token.as_str());
    let webhook = match Webhook::from_url(&http, webhook_url.as_str()).await {
        Ok(webhook) => webhook,
        Err(err) => {
            return eprintln!("Webhook error: {}", err);
        }
    };

    match webhook
        .execute(&http, true, |w| {
            w.content(message);
            w.username("pacemanbotlogs-1.15")
        })
        .await
    {
        Ok(_) => (),
        Err(err) => {
            return eprintln!("Webhook error: {}", err);
        }
    };
}
