use serenity::model::id::GuildId;

use crate::{cache::CacheManager, eprintln};

use super::ArcMutex;

pub async fn handle_guild_delete(cache_manager: ArcMutex<CacheManager>, guild_id: GuildId) {
    let mut locked_guild_cache = cache_manager.lock().await;
    match locked_guild_cache.remove_guild(guild_id).await {
        Ok(_) => (),
        Err(err) => return eprintln!("GuildDeleteError: {}", err),
    };
}
