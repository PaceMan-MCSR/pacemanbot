use crate::{consts::TIMEOUT_BETWEEN_CONSECUTIVE_QUERIES, handler_utils::*};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::Interaction,
        gateway::Ready,
        prelude::{Guild, GuildId, Message},
    },
};
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::time::sleep;

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

        let mut guild_cache: HashMap<GuildId, Vec<Message>> = HashMap::new();
        tokio::spawn(async move {
            loop {
                start_main_loop(ctx.clone(), &mut guild_cache).await;
                sleep(Duration::from_secs(TIMEOUT_BETWEEN_CONSECUTIVE_QUERIES)).await;
            }
        });
    }
}
