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

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new();

    let cache_manager = Arc::new(Mutex::new(CacheManager::new()));

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::GUILD_MEMBERS);
    intents.remove(GatewayIntents::GUILD_PRESENCES);
    intents.remove(GatewayIntents::MESSAGE_CONTENT);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { cache_manager })
        .framework(framework)
        .await?;
    client.start().await?;
    Ok(())
}
