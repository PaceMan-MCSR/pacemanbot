use std::sync::Arc;

use serenity::client::Context;

use crate::{cache::guild_data::GuildData, utils::{format_time::format_time, millis_to_mins_secs::millis_to_mins_secs, update_leaderboard::update_leaderboard}, ws::response::{Event, Response}};

pub async fn handle_non_pace_event(ctx: Arc<Context>, response: &Response, live_link: String, stats_link: String, last_event: &Event, guild_data: &mut GuildData) {
        let player_data = match guild_data.players.get_mut(&response.nickname.to_lowercase()) {
            Some(data) => data,
            None => {
                return println!(
                    "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                    response.nickname, 
                    guild_data.name
                );
            }
        };

        let runner_name = response.nickname.to_owned();
        let (minutes, seconds) = millis_to_mins_secs(last_event.igt as u64);
    
        let finish_minutes = match player_data.finish {
            Some(mins) => mins,
            None => {
                if !guild_data.is_private && minutes >= 10 {
                    return println!(
                        "Skipping guild name: {} because it is not a sub 10 completion and the guild is public.", 
                        guild_data.name
                    );
                }
                // minutes + 1 will always be greater than minutes. 
                // This is done to send finish message always if finish time is not defined.
                minutes + 1
            }, 
        };
        if minutes >= finish_minutes {
            return println!(
                "Skipping guild name: {} because finish time is above the defined amount.",
                guild_data.name,
            )
        }

        let content = format!(
            "## {} - Finish\n[ {} ]\t[ <t:{}:R> ]\t[ {} ]",
            format_time(last_event.igt as u64),
            live_link,
            (response.last_updated / 1000) as u64,
            stats_link,
        );

        match guild_data.pace_channel.send_message(&ctx, |m| m.content(content)).await {
            Ok(_) => {
                println!(
                    "Sent pace-ping for user with name: '{}' for split: 'Finish' in guild name: {}.",
                    response.nickname, guild_data.name 
                );
            }
            Err(err) => {
                return eprintln!(
                    "HandleNonPaceEvent: send split: 'Finish' due to: {}",
                    err
                );
            }
        };

        if !guild_data.is_private || guild_data.lb_channel.is_none() {
            return println!(
                "Can't handle non pace event for guild name: {} because it is a public server or does not have a leaderboard channel.", 
                guild_data.name
            );
        }

        match update_leaderboard(&ctx, guild_data.lb_channel.unwrap(), runner_name.to_owned(), (minutes, seconds))
            .await
        {
            Ok(_) => {
                println!(
                    "Updated leaderboard in #pacemanbot-runner-leaderboard for guild name: {}, runner name: {} with time: {}.", 
                    guild_data.name, 
                    runner_name, 
                    format_time(last_event.igt as u64),
                );
            }
            Err(err) => {
                eprintln!(
                    "HandleNonPaceEvent: update leaderboard in guild name: {} for runner name: {} due to: {}",
                    guild_data.name,
                    response.nickname.to_owned(), 
                    err
                );
            }
        };
}
