mod components;
mod consts;
mod controller;
mod guild_types;
mod handler;
mod handler_utils;
mod response_types;
#[cfg(test)]
mod tests;
mod utils;
use dotenv::dotenv;
use handler::Handler;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::futures::lock::Mutex;
use serenity::prelude::GatewayIntents;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

pub type ArcMux<T> = Arc<Mutex<T>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new();

    let guild_cache = Arc::new(Mutex::new(HashMap::new()));

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::GUILD_MEMBERS);
    intents.remove(GatewayIntents::GUILD_PRESENCES);
    intents.remove(GatewayIntents::MESSAGE_CONTENT);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { guild_cache })
        .framework(framework)
        .await?;
    client.start().await?;
    Ok(())
}
