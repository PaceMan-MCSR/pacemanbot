use std::sync::Arc;

use serenity::{builder::CreateEmbedAuthor, client::Context};

use crate::{
    cache::{consts::PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL, guild_data::GuildData},
    eprintln,
    utils::{
        format_time::format_time, millis_to_mins_secs::millis_to_hrs_mins,
        update_leaderboard::update_leaderboard,
    },
    ws::response::{Advancement, Response},
};

use super::consts::{CREDITS_EMOJI, OFFLINE_EMOJI, SPECIAL_UNDERSCORE, TWITCH_EMOJI};

pub async fn handle_non_pace_event(
    ctx: Arc<Context>,
    response: &Response,
    live_link: String,
    stats_link: String,
    author: CreateEmbedAuthor,
    last_advancement: &Advancement,
    guild_data: &mut GuildData,
) {
    let player_data = match guild_data
        .players
        .get_mut(&response.nickname.to_lowercase())
    {
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
    let (hours, minutes) = millis_to_hrs_mins(last_advancement.igt as u64);

    let finish_minutes = match player_data.finish {
        Some(mins) => mins,
        None => {
            if !guild_data.is_private && hours >= 3 {
                return println!(
                        "Skipping guild name: {} because it is not a sub 3 completion and the guild is public.", 
                        guild_data.name
                    );
            }
            0
        }
    };
    let finish_hours = (finish_minutes / 60) as u8;
    if player_data.finish.is_some() && hours >= finish_hours {
        return println!(
            "Skipping guild name: {} because finish time is above the defined amount.",
            guild_data.name,
        );
    }

    let finish_content = format!(
        "{}  {} - Finish",
        CREDITS_EMOJI,
        format_time(last_advancement.igt as u64),
    );

    match guild_data
        .pace_channel
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.set_author(author);
                e.field(finish_content, "", false);
                if response.user.live_account.is_some() {
                    e.field(format!("{} {}", TWITCH_EMOJI, live_link.clone()), "", false);
                } else {
                    e.field(format!("{}  Offline", OFFLINE_EMOJI), "", false);
                }
                e.field("Splits", format!("[Link]({})", stats_link), true);
                e.field(
                    "Time",
                    format!("<t:{}:R>", (response.last_updated / 1000) as u64),
                    true,
                );
                e
            })
        })
        .await
    {
        Ok(_) => {
            println!(
                "Sent pace-ping for user with name: '{}' for split: 'Finish' in guild name: {}.",
                response.nickname, guild_data.name
            );
        }
        Err(err) => {
            return eprintln!("HandleNonPaceEvent: send split: 'Finish' due to: {}", err);
        }
    };

    if !guild_data.is_private || guild_data.lb_channel.is_none() {
        return println!(
                "Can't handle non pace event for guild name: {} because it is a public server or does not have a leaderboard channel.", 
                guild_data.name
            );
    }

    match update_leaderboard(
        &ctx,
        guild_data.lb_channel.unwrap(),
        runner_name.to_owned().replace("_", SPECIAL_UNDERSCORE),
        (hours, minutes),
    )
    .await
    {
        Ok(_) => {
            println!(
                "Updated leaderboard in #{} for guild name: {}, runner name: {} with time: {}.",
                PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL,
                guild_data.name,
                runner_name,
                format_time(last_advancement.igt as u64),
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
