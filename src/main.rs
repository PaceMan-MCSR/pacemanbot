mod components;
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
use std::env;
use std::sync::Arc;

pub type ArcMux<T> = Arc<Mutex<T>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new();

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    client.start().await?;
    Ok(())
}
