mod cache;
mod command;
mod config;
mod dispatcher;
mod env;
mod handler;
mod interaction;
mod log;
mod ws;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::futures::lock::Mutex;
use serenity::prelude::GatewayIntents;
use std::error::Error;
use std::sync::Arc;

use env::Env;
use log::Log;

use crate::cache::Cache;
use crate::handler::Handler;
use crate::ws::WS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = match Env::new() {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Environment parse error: {}", e);
            return Err(e);
        }
    };
    let log = Log::new(&env);
    let ws = WS::new(
        env.ws_url.clone(),
        env.ws_host.clone(),
        env.api_auth_key.clone(),
    );
    let framework = StandardFramework::new();

    let cache = Arc::new(Mutex::new(Cache::new()));

    let mut intents = GatewayIntents::all();
    intents.remove(GatewayIntents::GUILD_MEMBERS);
    intents.remove(GatewayIntents::GUILD_PRESENCES);
    intents.remove(GatewayIntents::MESSAGE_CONTENT);

    let mut client = Client::builder(&env.bot_token, intents)
        .event_handler(Handler {
            log: Arc::new(log),
            cache,
            ws: Arc::new(ws),
        })
        .framework(framework)
        .await?;
    client.start().await?;
    Ok(())
}
