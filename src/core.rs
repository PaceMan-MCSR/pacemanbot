use std::sync::Arc;

use serenity::prelude::{Context, Mentionable};

use crate::utils::{
    event_id_to_split, extract_split_from_role_name, format_time, get_response_from_api, get_time,
    sort_guildroles_based_on_split, split_to_desc,
};

pub async fn start_main_loop(ctx: Arc<Context>) {
    let response = match get_response_from_api().await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
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
            let channels = match guild_id.channels(&ctx).await {
                Ok(channels) => channels,
                Err(err) => {
                    eprintln!(
                        "Error getting channels in guild name: {} due to: {}",
                        guild_name, err
                    );
                    continue;
                }
            };
            let (channel_to_send_to, _) = match channels.iter().find(|c| c.1.name == "pacemanbot") {
                Some(tup) => tup,
                None => {
                    eprintln!(
                        "Error finding #pacemanbot channel in guild name: {}.",
                        guild_name
                    );
                    continue;
                }
            };
            let guild_roles = match guild_id.roles(&ctx).await {
                Ok(roles) => roles,
                Err(err) => {
                    eprintln!(
                        "Unable to get roles in guild name: {} due to: {}",
                        guild_name, err
                    );
                    continue;
                }
            };
            let guild_roles = sort_guildroles_based_on_split(&guild_roles);
            if channels
                .iter()
                .any(|c| c.1.name == "pacemanbot-runner-names")
            {
                let (channel_containing_player_names, _) = channels
                    .iter()
                    .find(|c| c.1.name == "pacemanbot-runner-names")
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

                let player_names = match messages.last() {
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

                if !player_names
                    .iter()
                    .any(|name| name.to_owned() == record.nickname.to_owned())
                {
                    println!(
                            "Skipping because user, with name '{}', is not in this guild, with guild name: {}, or is not in the runners' channel.",
                            record.nickname,
                            guild_name,
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
                if record
                    .event_list
                    .iter()
                    .filter(|evt| evt != &last_event)
                    .any(|evt| evt.event_id == "rsg.enter_fortress")
                {
                    split = &"SS";
                } else {
                    split = &"FS";
                }
            } else if split == "F" {
                if record
                    .event_list
                    .iter()
                    .filter(|evt| evt != &last_event)
                    .any(|evt| evt.event_id == "rsg.enter_bastion")
                {
                    split = &"SS";
                } else {
                    split = &"FS";
                }
            }
            let roles_to_ping = guild_roles
                .iter()
                .filter(|role| {
                    let (role_split_name, role_minutes, role_seconds) =
                        extract_split_from_role_name(role.name.as_str());
                    let (split_minutes, split_seconds) = get_time(last_event.igt as u64);
                    role_split_name == *split
                        && role_minutes >= split_minutes
                        && (role_minutes != split_minutes || role_seconds >= split_seconds)
                })
                .collect::<Vec<_>>();

            let live_link = match record.user.live_account.to_owned() {
                Some(acc) => format!("<https://twitch.tv/{}>", acc),
                None => {
                    println!(
                        "Skipping split: '{}' because user with name: '{}' is not live.",
                        split_desc, record.nickname,
                    );
                    continue;
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
                }
            }
        }
    }
}
