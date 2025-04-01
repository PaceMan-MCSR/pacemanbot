use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use serenity::{client::Context, model::prelude::Ready};
use tokio::time::sleep;

use crate::{
    cache::{CacheManager, SeedWaveInfo},
    dispatcher::Dispatcher,
    utils::get_seedwave_info::get_seedwave_info,
    ws::{consts::WS_TIMEOUT_FOR_RETRY, WSManager},
};

use super::ArcMutex;

pub async fn ws_event_loop(
    ctx: Arc<Context>,
    cache_manager: ArcMutex<CacheManager>,
    seedwave_info: ArcMutex<SeedWaveInfo>,
) {
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
                seedwave_info: seedwave_info.clone(),
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

pub async fn handle_ready(
    ctx: Context,
    ready: Ready,
    cache_manager: ArcMutex<CacheManager>,
    seedwave_info: ArcMutex<SeedWaveInfo>,
) {
    println!("{} is connected!", ready.user.name);
    let cache_manager = cache_manager.clone();
    let seedwave_info_copy = seedwave_info.clone();
    let ctx = Arc::new(ctx);
    tokio::spawn(async move { ws_event_loop(ctx, cache_manager, seedwave_info.clone()).await });
    tokio::spawn(async move {
        loop {
            let latest_info = match get_seedwave_info().await {
                Ok(info) => info,
                Err(err) => {
                    eprintln!("SeedwaveInfoError: {}", err);
                    return;
                }
            };
            let mut locked_seedwave_info = seedwave_info_copy.lock().await;
            let sleep_secs = 60;
            println!("Sleeping seedwave thread for {}s", sleep_secs);
            *locked_seedwave_info = latest_info;
            drop(locked_seedwave_info);
            sleep(Duration::from_secs(sleep_secs)).await;
        }
    });
}
