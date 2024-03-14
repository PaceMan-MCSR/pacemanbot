use crate::{
    guild_types::{CachedGuilds, GuildData},
    handler_utils::*,
    response_types::Response,
    utils::get_response_stream_from_api,
    ArcMux,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    futures::StreamExt,
    model::{application::interaction::Interaction, gateway::Ready, prelude::Guild},
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

use crate::core::parse_record;
pub struct Handler {
    pub guild_cache: ArcMux<CachedGuilds>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            let guild_data = match GuildData::new(&ctx, guild.id).await {
                Ok(data) => data,
                Err(err) => return eprintln!("{}", err),
            };
            let mut locked_guild_cache = self.guild_cache.lock().await;
            locked_guild_cache.insert(guild.id, guild_data);
        }
        handle_guild_create(&ctx, guild.id).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        handle_interaction_create(&ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let ctx = Arc::new(ctx);

        const TIMEOUT_FOR_RETRY: u64 = 5;

        let guild_cache = self.guild_cache.clone();

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
                    for guild_id in ctx.clone().cache.guilds() {
                        let mut locked_guild_cache = guild_cache.lock().await;
                        if let Some(cache) = locked_guild_cache.get(&guild_id) {
                            let mut guild_data = match GuildData::new(&ctx, guild_id).await {
                                Ok(data) => data,
                                Err(err) => {
                                    eprintln!("{}", err);
                                    locked_guild_cache.remove(&guild_id);
                                    continue;
                                }
                            };
                            if !guild_data.is_private {
                                guild_data.players = cache.players.clone();
                            }
                            locked_guild_cache.insert(guild_id, guild_data);
                        }
                    }
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
                        let ctx = ctx.clone();
                        let guild_cache = guild_cache.clone();
                        tokio::spawn(async move {
                            parse_record(ctx.clone(), record, guild_cache.clone()).await;
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
