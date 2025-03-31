use serde::Deserialize;
use std::collections::HashMap;
use std::default::Default;

use guild_data::GuildData;
use serenity::model::id::GuildId;

pub mod cache;
pub mod consts;
pub mod guild_data;
pub mod players;
pub mod role_data;
pub mod split;

pub type CachedGuilds = HashMap<GuildId, GuildData>;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreviousSeedWave {
    pub level: String,
    pub ended_at: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeedWaveInfo {
    pub seedwave: u8,
    pub expires_at: u64,
    pub is_bloodseed: bool,
    pub previous_seedwave: PreviousSeedWave,
    pub timezone: String,
}

pub struct CacheManager {
    pub cache: CachedGuilds,
}
