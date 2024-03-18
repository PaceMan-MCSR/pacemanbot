use crate::{
    controller::Controller, guild_types::CachedGuilds, handler_utils::*, response_types::Response,
    utils::get_response_stream_from_api, ArcMux,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    futures::{lock::Mutex, StreamExt},
    model::{application::interaction::Interaction, gateway::Ready, prelude::Guild},
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        handle_guild_create(&ctx, guild.id).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        handle_interaction_create(&ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let ctx = Arc::new(ctx);

        const TIMEOUT_FOR_RETRY: u64 = 5;

        let guild_cache: ArcMux<CachedGuilds> = Arc::new(Mutex::new(HashMap::new()));

        tokio::spawn(async move {
            loop {
                let mut response_stream = match get_response_stream_from_api().await {
                    Ok(stream) => stream,
                    Err(err) => {
                        eprintln!("{}", err);
                        println!("Trying again in {} seconds...", TIMEOUT_FOR_RETRY);
                        sleep(Duration::from_secs(TIMEOUT_FOR_RETRY)).await;
                        continue;
                    }
                };
                while let Some(msg) = response_stream.next().await {
                    if let Ok(Message::Text(text_response)) = msg {
                        let record: Response = match serde_json::from_str(text_response.as_str()) {
                            Ok(response) => response,
                            Err(err) => {
                                eprintln!(
                                    "Unable to convert text response: '{}' to json due to: {}",
                                    text_response, err
                                );
                                continue;
                            }
                        };
                        let controller = Controller::new(ctx.clone(), record, guild_cache.clone());
                        tokio::spawn(async move {
                            controller.start().await;
                        });
                    }
                }
                println!(
                    "Invalid response from response stream.\nTrying again in {} seconds...",
                    TIMEOUT_FOR_RETRY
                );
                sleep(Duration::from_secs(TIMEOUT_FOR_RETRY)).await;
            }
        });
    }
}
