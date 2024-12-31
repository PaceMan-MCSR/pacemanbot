use serenity::{
    client::Context,
    model::{id::GuildId, prelude::GuildChannel},
};

use crate::cache::{
    consts::{
        PACEMANBOT_CHANNEL, PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL, PACEMANBOT_RUNNER_NAMES_CHANNEL,
    },
    CacheManager,
};

use super::{update_cache::handle_update_cache, ArcMutex};

pub async fn handle_channel_events(
    ctx: &Context,
    channel: &GuildChannel,
    guild_id: GuildId,
    cache_manager: ArcMutex<CacheManager>,
) {
    match channel.name.as_str() {
        PACEMANBOT_RUNNER_NAMES_CHANNEL
        | PACEMANBOT_CHANNEL
        | PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL => {
            match handle_update_cache(ctx, guild_id, cache_manager).await {
                Ok(_) => (),
                Err(err) => eprintln!("ChannelEventsError: {}", err),
            };
        }
        _ => {
            return println!(
                "Skipping channel event because it is not something that concerns the bot."
            )
        }
    }
}
