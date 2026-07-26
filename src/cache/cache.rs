use std::collections::HashMap;

use serenity::model::id::GuildId;

use crate::cache::GuildCacheEntry;

pub type CacheKey = GuildId;
pub struct Cache {
    pub entries: HashMap<CacheKey, GuildCacheEntry>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}
