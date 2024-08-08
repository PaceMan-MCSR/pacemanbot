use serenity::{client::Context, model::id::GuildId};

use crate::Result;

use super::{guild_data::GuildData, CacheManager, CachedGuilds};

impl CacheManager {
    pub fn new() -> Self {
        let cache = CachedGuilds::new();
        Self { cache }
    }

    pub async fn add_or_update_guild(&mut self, ctx: &Context, guild_id: GuildId) -> Result<()> {
        let guild_data = match GuildData::new(ctx, guild_id).await {
            Ok(data) => data,
            Err(err) => return Err(format!("CacheManagerError: {}", err).into()),
        };
        self.cache.insert(guild_id, guild_data);
        Ok(())
    }

    pub async fn remove_guild(&mut self, guild_id: GuildId) -> Result<()> {
        match self.cache.remove(&guild_id) {
            Some(_) => (),
            None => {
                return Err(
                    format!("CacheMangerError: Cannot remove guild id: {}", guild_id).into(),
                )
            }
        };
        Ok(())
    }
}
