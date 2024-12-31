use std::collections::HashMap;

use guild_data::GuildData;
use serenity::model::id::GuildId;

pub mod cache;
pub mod consts;
pub mod guild_data;
pub mod players;
pub mod role_data;
pub mod split;

pub type CachedGuilds = HashMap<GuildId, GuildData>;

pub struct CacheManager {
    pub cache: CachedGuilds,
}
