use std::sync::Arc;

use serenity::{builder::CreateEmbedAuthor, client::Context, prelude::Mentionable};

use crate::{
    cache::{guild_data::GuildData, players::PlayerSplitsData},
    eprintln,
    utils::{format_time::format_time, millis_to_mins_secs::millis_to_hrs_mins},
    ws::response::{Advancement, Response},
};

use super::{
    consts::{OFFLINE_EMOJI, SPECIAL_UNDERSCORE, TWITCH_EMOJI},
    get_run_info::get_run_info,
};

pub async fn handle_pace_event(
    ctx: Arc<Context>,
    response: &Response,
    live_link: String,
    stats_link: String,
    author: CreateEmbedAuthor,
    last_advancement: &Advancement,
    guild_data: &mut GuildData,
) {
    let run_info = match get_run_info(response, last_advancement) {
        Some(info) => info,
        None => {
            return eprintln!(
                "HandlePaceEvent: Unrecognized event id: {:#?}.",
                last_advancement.event_id
            );
        }
    };

    let player_data = match guild_data
        .players
        .get_mut(&response.nickname.to_lowercase())
    {
        Some(data) => data,
        None => {
            if guild_data.is_private {
                return println!(
                        "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                        response.nickname, 
                        guild_data.name
                    );
            }
            let player_data = PlayerSplitsData::default();
            guild_data
                .players
                .insert(response.nickname.to_owned().to_lowercase(), player_data);
            guild_data
                .players
                .get_mut(&response.nickname.to_lowercase())
                .unwrap()
        }
    };
    let split_desc = run_info.split.desc();

    let split_emoji = run_info.split.get_emoji();

    let roles_to_ping = guild_data
        .roles
        .iter()
        .filter(|role| {
            let (split_hours, split_minutes) = millis_to_hrs_mins(last_advancement.igt as u64);
            if role.guild_role.name.contains("PB") {
                if !guild_data.is_private {
                    return false;
                }
                let pb_minutes = player_data.get(&role.split).unwrap().to_owned();
                let pb_hours = (pb_minutes / 60) as u8;
                let pb_minutes = (pb_minutes % 60) as u8;
                role.split == run_info.split && pb_hours > split_hours && pb_minutes > split_minutes
            } else if role.guild_role.name.contains("+") {
                role.split == run_info.split
                    && role.runner.to_lowercase() == response.nickname.to_lowercase()
                    && role.hours >= split_hours
                    && (role.hours != split_hours || role.minutes > split_minutes)
            } else {
                role.split == run_info.split
                    && role.hours >= split_hours
                    && (role.hours != split_hours || role.minutes > split_minutes)
            }
        })
        .collect::<Vec<_>>();

    if roles_to_ping.is_empty() {
        return println!(
            "Skipping split: '{}' because there are no roles to ping in guild name: {}.",
            split_desc, guild_data.name
        );
    }

    let live_indicator = if response.user.live_account.is_some() {
        "🔴"
    } else {
        "⚪"
    };

    let metadata = format!(
        "{} {} - {} {}",
        live_indicator,
        format_time(last_advancement.igt as u64),
        split_desc,
        response.nickname.replace("_", SPECIAL_UNDERSCORE)
    );

    let ping_content = format!(
        "{}\n-# {}",
        metadata.clone(),
        roles_to_ping
            .iter()
            .map(|role| role.guild_role.mention().to_string())
            .collect::<Vec<_>>()
            .join(" "),
    );

    let pace_content = format!(
        "{}  {} - {}",
        split_emoji,
        format_time(last_advancement.igt as u64),
        split_desc,
    );

    match guild_data
        .pace_channel
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.set_author(author.clone());
                e.field(pace_content.clone(), "", false);
                if response.user.live_account.is_some() {
                    e.field(format!("{} {}", TWITCH_EMOJI, live_link.clone()), "", false);
                } else {
                    e.field(format!("{}  Offline", OFFLINE_EMOJI), "", false);
                }
                e.field("Splits", format!("[Link]({})", stats_link.clone()), true);
                e.field(
                    "Time",
                    format!("<t:{}:R>", (response.last_updated / 1000) as u64),
                    true,
                );
                e
            })
            .content(ping_content.to_owned())
        })
        .await
    {
        Ok(mut message) => {
            println!(
                "Sent pace-ping for user with name: '{}' for split: '{}' in guild name: {}.",
                response.nickname, split_desc, guild_data.name
            );
            let removable_roles = roles_to_ping
                .iter()
                .filter(|r| r.runner.as_str() != "")
                .map(|r| r.guild_role.mention())
                .collect::<Vec<_>>();
            let mut new_content = ping_content.to_owned();
            for role in removable_roles {
                let replacable_str = format!("{} ", role);
                new_content = new_content.replace(replacable_str.as_str(), "");
            }
            let content_removed_metadata =
                new_content.replace(format!("{}\n", metadata).as_str(), "");
            match message
                .edit(&ctx.http, |m| {
                    m.embed(|e| {
                        e.set_author(author);
                        e.field(pace_content, "", false);
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
                    .content(content_removed_metadata)
                })
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    return eprintln!("HandlePaceEvent: edit message due to: {}", err);
                }
            };
        }
        Err(err) => {
            return eprintln!(
                "HandlePaceEvent: send split: '{}' with roles: {:?} due to: {}",
                split_desc, roles_to_ping, err
            );
        }
    };
}
