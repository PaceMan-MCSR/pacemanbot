use std::{collections::HashMap, sync::Arc};

use serenity::prelude::{Context, Mentionable};

use crate::{
    types::Response,
    utils::{
        event_id_to_split, extract_name_and_splits_from_line, extract_split_from_pb_role_name,
        extract_split_from_role_name, format_time, get_time, split_to_desc, update_leaderboard,
    },
};

pub async fn start_guild_loop(ctx: Arc<Context>, record: Response) {
    let ctx = ctx.clone();
    'guild_loop: for guild_id in &ctx.cache.guilds() {
        let guild_name = match guild_id.name(&ctx.cache) {
            Some(name) => name,
            None => {
                eprintln!("Error getting name for guild id: {}.", guild_id);
                continue;
            }
        };
        let channels = match ctx.cache.guild_channels(guild_id) {
            Some(channels) => channels.to_owned(),
            None => {
                eprintln!("Unable to get channels for guild with name: {}", guild_name);
                continue;
            }
        };
        let channel_to_send_to = match channels.iter().find(|c| c.name == "pacemanbot") {
            Some(channel) => channel,
            None => {
                eprintln!(
                    "Error finding #pacemanbot channel in guild name: {}.",
                    guild_name
                );
                continue;
            }
        };
        let guild_roles = match ctx.cache.guild_roles(guild_id) {
            Some(roles) => roles,
            None => {
                eprintln!("Unable to get roles in guild name: {}.", guild_name);
                continue;
            }
        };
        let guild_roles = guild_roles
            .iter()
            .filter(|(_, role)| role.name.starts_with("*"))
            .map(|(_, role)| role)
            .collect::<Vec<_>>();

        let mut player_in_runner_names = false;
        let mut player_splits: HashMap<String, u8> = HashMap::new();
        if channels.iter().any(|c| c.name == "pacemanbot-runner-names") {
            let channel_containing_player_names = channels
                .iter()
                .find(|c| c.name == "pacemanbot-runner-names")
                .unwrap();

            let messages = match channel_containing_player_names
                .messages(&ctx, |m| m.limit(1))
                .await
            {
                Ok(messages) => messages,
                Err(err) => {
                    eprintln!(
                        "Error getting messages from #pacemanbot-runner-names for guild name: {} due to: {}",
                        guild_name, err
                    );
                    continue;
                }
            };

            let player_names = 
                    match messages.last() {
                        Some(message) => message,
                        None => {
                            eprintln!(
                                "Error getting first message from #pacemanbot-runner-names for guild name: {}",
                                guild_name
                            );
                            continue;
                        }
                    }
                    .content
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

            let split_codes = vec!["FS", "SS", "B", "E", "EE"];

            for line in player_names.iter() {
                let (player_name, splits) = match extract_name_and_splits_from_line(line.as_str()) {
                    Ok(tup) => tup,
                    Err(err) => {
                        eprintln!(
                            "Unable to parse runner-names in guild, with name {} due to: {}",
                            guild_name, err
                        );
                        continue 'guild_loop;
                    }
                };

                // Nish did this :PagMan:
                if player_name.to_lowercase() == record.nickname.to_owned().to_lowercase() {
                    let mut split_no = 0;
                    for split_minutes in splits {
                        let split = split_codes[split_no];
                        player_splits.insert(split.to_string(), split_minutes);
                        split_no += 1;
                    }
                    player_in_runner_names = true;
                    break;
                }
            }

            if !player_in_runner_names {
                println!(
                    "Skipping because player, with name '{}' is not in this guild, with guild name: '{}', or is not in the runners channel.", 
                     record.nickname.to_owned(),
                     guild_name
                 );
                continue;
            }
        }
        let last_event = match record.event_list.last() {
            Some(event) => event.to_owned(),
            None => {
                eprintln!("No events in event list for record: {:#?}.", record);
                continue;
            }
        };

        if last_event.event_id.as_str() == "rsg.credits" && player_in_runner_names {
            let runner_name = record.nickname.to_owned();
            let (minutes, seconds) = get_time(last_event.igt as u64);
            match update_leaderboard(&ctx, guild_id, runner_name.to_owned(), (minutes, seconds))
                .await
            {
                Ok(_) => {
                    println!(
                        "Updated leaderboard in #pacemanbot-runner-leaderboard for guild name: {}, runner name: {} with time: {}.", 
                        guild_name, 
                        runner_name, 
                        format_time(last_event.igt as u64),
                    );
                    continue;
                }
                Err(err) => {
                    eprintln!(
                        "Unable to update leaderboard in guild name: {} for runner name: {} due to: {}",
                        guild_name, 
                        record.nickname.to_owned(), 
                        err
                    );
                    continue;
                }
            };
        }
        if event_id_to_split(last_event.event_id.as_str()).is_none() {
            if last_event.event_id.as_str() == "rsg.credits" {
                println!(
                    "Skipping guild with name '{}' for event id: '{}'.", 
                    guild_name, 
                    last_event.event_id
                );
                // Check other guilds here because we would want to check all guilds for a
                // completion.
                continue;
            }
            println!(
                "Skipping event id: '{}' as it is unrecognized.",
                last_event.event_id
            );
            // Skip checking other guilds as the event id is not gonna be recognized in them as
            // well.
            return;
        }

        let mut split = event_id_to_split(last_event.event_id.as_str()).unwrap();
        let mut bastionless_content = "";

        let split_desc = match split_to_desc(split) {
            Some(desc) => desc,
            None => {
                eprintln!("Unable to get description for split code: {}.", split);
                continue;
            }
        };

        if split == "Ba" {
            let bastion_ss_context_check = record
                .context_event_list
                .iter()
                .any(|ctx| ctx.event_id == "rsg.obtain_blaze_rod");

            if bastion_ss_context_check {
                split = &"SS";
            } else {
                split = &"FS";
            }
        } else if split == "F" {
            let fort_ss_check = record
                .event_list
                .iter()
                .filter(|evt| evt != &last_event)
                .any(|evt| evt.event_id == "rsg.enter_bastion");

            let mut fort_ss_context_check = false;
            let mut context_hits = 0;
            for ctx in record.context_event_list.iter() {
                let context_check = ctx.event_id == "rsg.obtain_crying_obsidian" 
                    || ctx.event_id == "rsg.obtain_obsidian" 
                    || ctx.event_id == "rsg.loot_bastion";
                if context_check {
                    context_hits += 1;
                } 
            }
            if context_hits >= 2 {
                fort_ss_context_check = true;
            }

            if fort_ss_check && fort_ss_context_check {
                split = &"SS";
            }
        }

        if split == "B" {
            if !record
                .event_list
                .iter()
                .filter(|evt| evt != &last_event)
                .any(|evt| evt.event_id == "rsg.enter_bastion")
            {
                bastionless_content = "(Bastionless)";
            }
        }

        let roles_to_ping = guild_roles
            .iter()
            .filter(|role| {
                let (split_minutes, split_seconds) = get_time(last_event.igt as u64);
                if role.name.contains("PB") {
                    if !player_in_runner_names {
                        return false;
                    }
                    let role_split = extract_split_from_pb_role_name(role.name.as_str());
                    let pb_minutes = player_splits.get(&role_split).unwrap().to_owned();
                    role_split == *split && pb_minutes > split_minutes
                } else {
                    let (role_split_name, role_minutes, role_seconds) =
                        match extract_split_from_role_name(role.name.as_str()) {
                            Ok(tup) => tup,
                            Err(err) => {
                                eprintln!(
                                    "Unable to extract split from role name: '{}' due to: {}",
                                    role.name, err
                                );
                                return false;
                            }
                        };
                    role_split_name == *split
                        && role_minutes >= split_minutes
                        && (role_minutes != split_minutes || role_seconds > split_seconds)
                }
            })
            .collect::<Vec<_>>();

        let live_link = match record.user.live_account.to_owned() {
            Some(acc) => format!("[{}](<https://twitch.tv/{}>)", record.nickname, acc),
            None => {
                if !player_in_runner_names {
                    println!(
                        "Skipping split: '{}' because user with name: '{}' is not live.",
                        split_desc, record.nickname,
                    );
                    continue;
                } else {
                    format!("Offline - {}", record.nickname.to_owned())
                }
            }
        };

        if roles_to_ping.is_empty() {
            println!(
                "Skipping split: '{}' because there are no roles to ping in guild name: {}.",
                split_desc, guild_name
            );
            continue;
        }

        let content = format!(
            "## {} - {} {}\n{}\t<t:{}:R>\n{}",
            format_time(last_event.igt as u64),
            split_desc,
            bastionless_content,
            live_link,
            (record.last_updated / 1000) as u64,
            roles_to_ping
                .iter()
                .map(|role| role.mention().to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
        match channel_to_send_to
            .send_message(&ctx, |m| m.content(content))
            .await
        {
            Ok(_) => {
                println!(
                    "Sent pace-ping for user with name: '{}' for split: '{}' in guild name: {}.",
                    record.nickname, split_desc, guild_name
                );
            }
            Err(err) => {
                eprintln!(
                    "Unable to send split: '{}' with roles: {:?} due to: {}",
                    split_desc, roles_to_ping, err
                );
                continue;
            }
        }
    }
}
