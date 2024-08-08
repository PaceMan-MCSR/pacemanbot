use serenity::{client::Context, model::id::GuildId};

use crate::{cache::CacheManager, Result};

use super::ArcMutex;

pub async fn handle_update_cache(
    ctx: &Context,
    guild_id: GuildId,
    cache_manager: ArcMutex<CacheManager>,
) -> Result<()> {
    let mut locked_guild_cache = cache_manager.lock().await;
    match locked_guild_cache.add_or_update_guild(ctx, guild_id).await {
        Ok(_) => (),
        Err(err) => return Err(format!("UpdateCacheError: {}", err).into()),
    };
    Ok(())
}
