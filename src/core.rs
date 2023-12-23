use std::{collections::HashMap, sync::Arc};

use serenity::{
    model::prelude::{GuildId, Message},
    prelude::{Context, Mentionable},
};

use crate::utils::{
    event_id_to_split, extract_split_from_role_name, format_time, get_response_from_api, get_time,
    split_to_desc, extract_split_from_pb_role_name, extract_name_and_splits_from_line,
};

pub async fn start_main_loop(ctx: Arc<Context>, guild_cache: &mut HashMap<GuildId, Vec<Message>>) {
    let mut response = match get_response_from_api().await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    response.sort_by(|r1, r2|{
        r1.event_list.len().cmp(&r2.event_list.len())
    });
    response.reverse();
    let ctx = ctx.clone();
    for record in response.iter() {
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

            if guild_cache.get(guild_id).is_none() {
                let messages = match channel_to_send_to.messages(&ctx, |m| m.limit(100)).await {
                    Ok(messages) => messages,
                    Err(err) => {
                        eprintln!(
                            "Unable to get messages from #pacemanbot for guild name: {} due to: {}",
                            guild_name, err
                        );
                        continue;
                    }
                };
                guild_cache.insert(guild_id.to_owned(), messages);
            }
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
                
                for line in player_names.iter(){
                    let (player_name, splits) = match extract_name_and_splits_from_line(line.as_str()){
                        Ok(tup) => tup,
                        Err(err) => {
                            eprintln!("Unable to parse runner-names in guild, with name {} due to: {}", guild_name, err);
                            continue 'guild_loop;
                        }
                    };
                    
                    // Nish did this :PagMan:
                    if player_name.to_lowercase() == record.nickname.to_owned().to_lowercase(){
                        let mut split_no = 0;
                        for split_minutes in splits{
                            let split = split_codes[split_no];
                            player_splits.insert(split.to_string(), split_minutes);
                            split_no += 1;
                        }
                        player_in_runner_names = true;
                        break;
                    }
                }

                if !player_in_runner_names{
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
            if event_id_to_split(last_event.event_id.as_str()).is_none() {
                println!(
                    "Skipping event id: '{}' as it is unrecognized.",
                    last_event.event_id
                );
                continue;
            }

            let mut split = event_id_to_split(last_event.event_id.as_str()).unwrap();

            let messages = guild_cache.get_mut(guild_id).unwrap();

            let split_desc = match split_to_desc(split) {
                Some(desc) => desc,
                None => {
                    eprintln!("Unable to get description for split code: {}.", split);
                    continue;
                }
            };

            if messages.iter().any(|message| {
                message.content.contains(split_desc)
                    && message
                        .content
                        .contains(&format_time(last_event.igt as u64))
                    && (message.content.contains(
                        &record
                            .user
                            .live_account
                            .to_owned()
                            .unwrap_or("".to_string()),
                    ))
            }) {
                println!(
                    "Skipping split '{}' because it's already in the channel for guild name: {}.",
                    split_desc, guild_name,
                );
                continue 'guild_loop;
            }

            if split == "Ba" {
                if !record
                    .event_list
                    .iter()
                    .filter(|evt| evt != &last_event)
                    .any(|evt| evt.event_id == "rsg.enter_fortress")
                {
                    split = &"FS";
                } else if player_in_runner_names {
                    split = &"SS";
                }
            } else if split == "F" {
                if record
                    .event_list
                    .iter()
                    .filter(|evt| evt != &last_event)
                    .any(|evt| evt.event_id == "rsg.enter_bastion")
                {
                    split = &"SS";
                } else if player_in_runner_names {
                    split = &"FS";
                }
            }
            let roles_to_ping = guild_roles
                .iter()
                .filter(|role| {
                    let (split_minutes, split_seconds) = get_time(last_event.igt as u64);
                    if role.name.contains("PB") && player_in_runner_names{
                        let role_split = extract_split_from_pb_role_name(role.name.as_str());
                        let pb_minutes = player_splits.get(&role_split).unwrap().to_owned();
                        role_split == *split && pb_minutes >= split_minutes
                    } else{
                        let (role_split_name, role_minutes, role_seconds) =
                            extract_split_from_role_name(role.name.as_str());
                        role_split_name == *split
                            && role_minutes >= split_minutes
                            && (role_minutes != split_minutes || role_seconds >= split_seconds)
                    }
                })
                .collect::<Vec<_>>();

            let live_link = match record.user.live_account.to_owned() {
                Some(acc) => format!("<https://twitch.tv/{}>", acc),
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
                "## {} - {}\n\n{}\n{}",
                format_time(last_event.igt as u64),
                split_desc,
                live_link,
                roles_to_ping
                    .iter()
                    .map(|role| role.mention().to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            let message_id;
            match channel_to_send_to
                .send_message(&ctx, |m| m.content(content))
                .await
            {
                Ok(message) => {
                    println!(
                        "Sent pace-ping for user with name: '{}' for split: '{}' in guild name: {}.", 
                         record.nickname, split_desc, guild_name
                    );
                    message_id = message.id;
                }
                Err(err) => {
                    eprintln!(
                        "Unable to send split: '{}' with roles: {:?} due to: {}",
                        split_desc, roles_to_ping, err
                    );
                    continue;
                }
            }

            let last_pace_message = match ctx.cache.message(channel_to_send_to.id, message_id) {
                Some(message) => message,
                None => {
                    eprintln!(
                        "Unable to construct last pace message from message id in guild name: {}.",
                        guild_name
                    );
                    continue;
                }
            };
            messages.push(last_pace_message.to_owned());
            if messages.len() > 100 {
                messages.remove(0);
            }
        }
    }
}
