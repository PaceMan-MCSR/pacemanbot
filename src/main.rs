mod components;
mod core;
mod guild_types;
mod handler;
mod handler_utils;
mod response_types;
#[cfg(test)]
mod tests;
mod utils;
use dotenv::dotenv;
use guild_types::CachedGuilds;
use handler::Handler;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::futures::lock::Mutex;
use serenity::prelude::GatewayIntents;
use std::sync::Arc;
use std::{collections::HashMap, env};

pub type ArcMux<T> = Arc<Mutex<T>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new().configure(|c| c.prefix("!"));

    let guild_cache: ArcMux<CachedGuilds> = Arc::new(Mutex::new(HashMap::new()));

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler { guild_cache })
        .framework(framework)
        .await
        .expect("Error creating client");
    client.start().await?;
    Ok(())
}
