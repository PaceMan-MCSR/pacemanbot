use std::sync::Arc;

use serenity::futures::lock::Mutex;

use crate::cache::CacheManager;

pub mod application_command_interaction;
pub mod channel_events;
pub mod guild_create;
pub mod guild_delete;
pub mod guild_role_events;
pub mod handler;
pub mod interaction_create;
pub mod message_component_interaction;
pub mod message_events;
pub mod ready;
pub mod update_cache;

pub type ArcMutex<T> = Arc<Mutex<T>>;

pub struct Handler {
    pub cache_manager: ArcMutex<CacheManager>,
}
