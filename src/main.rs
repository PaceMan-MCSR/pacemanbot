mod components;
mod handlers;
#[cfg(test)]
mod tests;
mod types;
mod utils;
use dotenv::dotenv;
use handlers::Handler;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::prelude::GatewayIntents;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new().configure(|c| c.prefix("!"));

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    client.start().await?;
    Ok(())
}
