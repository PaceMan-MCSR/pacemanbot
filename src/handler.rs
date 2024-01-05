use crate::{handler_utils::*, utils::get_response_from_api};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{application::interaction::Interaction, gateway::Ready, prelude::Guild},
};
use std::sync::Arc;
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite::Message;

use crate::core::start_main_loop;
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
        ctx.cache.set_max_messages(100);

        tokio::spawn(async move {
            let mut response_stream = match get_response_from_api().await {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            while let Some(msg) = response_stream.next().await {
                if let Ok(Message::Text(text_response)) = msg {
                    let response = match serde_json::from_str(text_response.as_str()) {
                        Ok(response) => response,
                        Err(err) => {
                            eprintln!(
                                "Unable to convert text response: {} to json response due to: {}",
                                text_response, err
                            );
                            continue;
                        }
                    };
                    start_main_loop(ctx.clone(), response).await;
                }
            }
        });
    }
}
