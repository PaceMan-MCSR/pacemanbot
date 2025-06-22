use serenity::{
    client::Context,
    model::{guild::Role, id::GuildId},
};

use crate::{
    cache::{
        consts::{ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17, ROLE_PREFIX_AA},
        CacheManager,
    },
    eprintln,
};

use super::{update_cache::handle_update_cache, ArcMutex};

pub async fn handle_guild_role_events(
    ctx: &Context,
    new: Role,
    guild_id: GuildId,
    cache_manager: ArcMutex<CacheManager>,
) {
    if !new.name.starts_with(ROLE_PREFIX)
        || new.name.starts_with(ROLE_PREFIX_115)
        || new.name.starts_with(ROLE_PREFIX_17)
        || new.name.starts_with(ROLE_PREFIX_AA)
    {
        return println!(
            "Skipping role create event because it is not something that concerns the bot."
        );
    }
    match handle_update_cache(ctx, guild_id, cache_manager).await {
        Ok(_) => (),
        Err(err) => eprintln!("GuildRoleEvents: {}", err),
    }
}
