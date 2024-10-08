use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId},
};

use crate::cache::CacheManager;

use super::{update_cache::handle_update_cache, ArcMutex};

pub async fn handle_message_events(
    ctx: &Context,
    channel_id: ChannelId,
    guild_id: GuildId,
    guild_cache: ArcMutex<CacheManager>,
) {
    let name = match channel_id.name(&ctx.cache).await {
        Some(name) => name,
        None => {
            return eprintln!(
                "MessageEventsError: get guild name for channel id: {}.",
                channel_id
            );
        }
    };
    if name != "pacemanbot-runner-names" {
        return println!(
            "Skipping message delete because it was not sent in #pacemanbot-runner-names.",
        );
    }
    match handle_update_cache(ctx, guild_id, guild_cache).await {
        Ok(_) => (),
        Err(err) => {
            return eprintln!("MessageEventsError: {}", err);
        }
    };
}
