use serenity::{
    client::Context,
    model::{id::GuildId, prelude::Activity, user::OnlineStatus},
};

use crate::{
    cache::CacheManager, components::application::default_commands::setup_default_commands,
};

use super::ArcMutex;

pub async fn handle_guild_create(
    ctx: &Context,
    guild_id: GuildId,
    cache_manager: ArcMutex<CacheManager>,
) {
    setup_default_commands(&ctx, guild_id).await;
    ctx.set_presence(Some(Activity::watching("paceman.gg")), OnlineStatus::Online)
        .await;
    let mut locked_guild_cache = cache_manager.lock().await;
    match locked_guild_cache.add_or_update_guild(&ctx, guild_id).await {
        Ok(_) => (),
        Err(err) => {
            return eprintln!("GuildCreateError: {}", err);
        }
    };
}
