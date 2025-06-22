use std::sync::Arc;

use serenity::{client::Context, model::prelude::Ready};

use crate::{cache::CacheManager, dispatcher::Dispatcher, eprintln, ws::WSManager};

use super::ArcMutex;

pub async fn ws_event_loop(
    ctx: Arc<Context>,
    cache_manager: ArcMutex<CacheManager>,
    ws_manager: ArcMutex<WSManager>,
) {
    loop {
        loop {
            let mut locked_ws_mgr = ws_manager.lock().await;
            let response = match locked_ws_mgr.get_next().await {
                Some(response) => response,
                None => continue,
            };
            let dispatcher = Dispatcher {
                ctx: ctx.clone(),
                response,
                cache_manager: cache_manager.clone(),
            };
            match dispatcher.dispatch().await {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Dispatch error: {}", err);
                    continue;
                }
            };
        }
    }
}

pub async fn handle_ready(
    ctx: Context,
    ready: Ready,
    cache_manager: ArcMutex<CacheManager>,
    ws_manager: ArcMutex<WSManager>,
) {
    println!("{} is connected!", ready.user.name);
    let cache_manager = cache_manager.clone();
    let ctx = Arc::new(ctx);
    tokio::spawn(async move { ws_event_loop(ctx, cache_manager, ws_manager.clone()).await });
}
