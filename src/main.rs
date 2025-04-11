use dotenv::dotenv;
mod cache;
mod components;
mod dispatcher;
mod handler;
#[cfg(test)]
mod tests;
mod utils;
mod ws;
use cache::CacheManager;
use handler::Handler;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::futures::lock::Mutex;
use serenity::prelude::GatewayIntents;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use utils::send_webhook_message::send_webhook_message;
use ws::consts::WS_TIMEOUT_FOR_RETRY;
use ws::WSManager;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn spawn_send_webhook(msg: String) {
    tokio::spawn(async move {
        send_webhook_message(msg).await;
    });
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {{
        let formatted = format!($($arg)*);
        std::eprintln!("{}", formatted);
        $crate::spawn_send_webhook(formatted);
    }};
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new();

    let cache_manager = Arc::new(Mutex::new(CacheManager::new()));

    let ws_manager;

    loop {
        ws_manager = Arc::new(Mutex::new(match WSManager::new().await {
            Ok(mgr) => mgr,
            Err(err) => {
                eprintln!("WSManager init error: {}", err);
                println!("Trying again in {} seconds...", WS_TIMEOUT_FOR_RETRY);
                sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
                continue;
            }
        }));
        break;
    }

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::GUILD_MEMBERS);
    intents.remove(GatewayIntents::GUILD_PRESENCES);
    intents.remove(GatewayIntents::MESSAGE_CONTENT);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            cache_manager,
            ws_manager,
        })
        .framework(framework)
        .await?;
    client.start().await?;
    Ok(())
}
