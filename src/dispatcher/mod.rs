use serenity::client::Context;
use std::sync::Arc;
pub mod consts;
pub mod dispatcher;
pub mod get_run_info;
pub mod non_pace_event;
pub mod pace_event;
pub mod run_info;
use crate::{cache::CacheManager, handler::ArcMutex, ws::response::Response};

pub struct Dispatcher {
    pub ctx: Arc<Context>,
    pub response: Response,
    pub cache_manager: ArcMutex<CacheManager>,
}
