mod components;
mod core;
mod handler;
mod handler_utils;
#[cfg(test)]
mod tests;
mod types;
mod utils;
use dotenv::dotenv;
use handler::Handler;
use serenity::framework::standard::StandardFramework;
use serenity::futures::lock::Mutex;
use serenity::prelude::GatewayIntents;
use serenity::{client::Client, model::id::GuildId};
use std::sync::Arc;
use std::{collections::HashMap, env};
use types::{ArcMux, GuildData};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new().configure(|c| c.prefix("!"));

    let guild_cache: ArcMux<HashMap<GuildId, GuildData>> = Arc::new(Mutex::new(HashMap::new()));

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler { guild_cache })
        .framework(framework)
        .await
        .expect("Error creating client");
    client.start().await?;
    Ok(())
}
