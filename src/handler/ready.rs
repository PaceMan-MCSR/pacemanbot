use std::{sync::Arc, time::Duration};

use serenity::{client::Context, model::prelude::Ready};
use tokio::time::sleep;

use crate::{
    cache::CacheManager,
    dispatcher::Dispatcher,
    ws::{consts::WS_TIMEOUT_FOR_RETRY, WSManager},
};

use super::ArcMutex;

pub async fn ws_event_loop(ctx: Arc<Context>, cache_manager: ArcMutex<CacheManager>) {
    loop {
        let mut manager = match WSManager::new().await {
            Ok(manager) => manager,
            Err(err) => {
                eprintln!("WSManager init error: {}", err);
                println!("Trying again in {} seconds...", WS_TIMEOUT_FOR_RETRY);
                sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
                continue;
            }
        };
        loop {
            let response = match manager.get_next().await {
                Some(response) => response,
                None => break,
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
        drop(manager);
    }
}

pub async fn handle_ready(ctx: Context, ready: Ready, cache_manager: ArcMutex<CacheManager>) {
    println!("{} is connected!", ready.user.name);
    let cache_manager = cache_manager.clone();
    let ctx = Arc::new(ctx);
    tokio::spawn(async move { ws_event_loop(ctx, cache_manager).await });
}
