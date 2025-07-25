use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::{
    cache::{
        consts::PACEMANBOT_RUNNER_NAMES_CHANNEL,
        players::{PlayerSplitsData, Players},
    },
    utils::{
        extract_name_or_uuid_and_splits_from_line::extract_name_or_uuid_and_splits_from_line,
        get_new_config_contents::get_new_config_contents,
    },
    Result,
};

pub async fn whitelist(
    ctx: &Context,
    guild_id: GuildId,
    command: &ApplicationCommandInteraction,
    use_uuid: bool,
) -> Result<()> {
    command.defer_ephemeral(&ctx).await?;
    let channels = match ctx.cache.guild_channels(guild_id) {
        Some(channels) => channels,
        None => {
            return Err(format!("WhitelistError: get channels for guild id: {}", guild_id).into());
        }
    };
    let mut action = String::new();
    let mut ign = String::new();
    let mut uuid = String::new();
    let mut splits_data = PlayerSplitsData::default();

    for option in command.data.options.iter() {
        match option.name.as_str() {
            "action" => {
                action = match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse string for action option.",
                            )
                            .into())
                        }
                    },
                    None => {
                        return Err(
                            String::from("WhitelistError: get value for action option.").into()
                        )
                    }
                }
            }
            "ign" => match option.value.to_owned() {
                Some(value) => {
                    ign = match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse string for ign option.",
                            )
                            .into())
                        }
                    }
                }
                None => {
                    return Err(String::from("WhitelistError: get value for ign option.").into())
                }
            },
            "uuid" => match option.value.to_owned() {
                Some(value) => {
                    uuid = match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse string for uuid option.",
                            )
                            .into())
                        }
                    }
                }
                None => {
                    return Err(String::from("WhitelistError: get value for uuid option.").into())
                }
            },
            "first_structure" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.first_structure = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse u64 for first structure option.",
                            )
                            .into())
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(String::from(
                            "WhitelistError: get value for first structure option.",
                        )
                        .into());
                    }
                }
            },
            "second_structure" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.second_structure = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse u64 for second structure option.",
                            )
                            .into())
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(String::from(
                            "WhitelistError: get value for second structure option.",
                        )
                        .into());
                    }
                }
            },
            "blind" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.blind = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(
                                String::from("WhitelistError: parse u64 for blind option.").into()
                            )
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(
                            String::from("WhitelistError: get value for blind option.").into()
                        );
                    }
                }
            },
            "eye_spy" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.eye_spy = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse u64 for eye spy option.",
                            )
                            .into())
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(
                            String::from("WhitelistError: get value for eye spy option.").into(),
                        );
                    }
                }
            },
            "end_enter" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.end_enter = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse u64 for end enter option.",
                            )
                            .into())
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(String::from(
                            "WhitelistError: get value for end enter option.",
                        )
                        .into());
                    }
                }
            },
            "finish" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.finish = match value.as_u64() {
                        Some(int) => Some(int as u8),
                        None => {
                            return Err(String::from(
                                "WhitelistError: parse u64 for end enter option.",
                            )
                            .into())
                        }
                    }
                }
                None => splits_data.finish = None,
            },
            _ => return Err(format!("Unrecognized command option: '{}'", option.name).into()),
        };
    }

    let channel = channels
        .iter()
        .filter(|c| c.name == PACEMANBOT_RUNNER_NAMES_CHANNEL)
        .collect::<Vec<_>>();
    let channel = match channel.first() {
        Some(channel) => channel,
        None => {
            return Err(format!(
                "WhitelistError: find #{} in guild id: {}",
                PACEMANBOT_RUNNER_NAMES_CHANNEL, guild_id
            )
            .into())
        }
    };
    let message = channel.messages(&ctx.http, |m| m.limit(1)).await?;
    let mut players: Players = Players::new();
    match message.last() {
        Some(message) => {
            if !message.author.bot {
                let response_content = format!(
                    "WhitelistError: The first message in #{} is not from the bot.",
                    PACEMANBOT_RUNNER_NAMES_CHANNEL
                );
                command
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(response_content.to_string())
                    })
                    .await?;
                return Err(response_content.into());
            }
            for line in message.content.split("\n") {
                if line == "```" || line == "" {
                    continue;
                }
                let (name, split_data) = extract_name_or_uuid_and_splits_from_line(line)?;
                players.insert(name, split_data);
            }
            if action == "remove" {
                if use_uuid {
                    players.remove(&uuid);
                } else {
                    players.remove(&ign);
                }
            } else {
                if use_uuid {
                    players.insert(uuid, splits_data);
                } else {
                    players.insert(ign, splits_data);
                }
            }
            let new_config = get_new_config_contents(players);
            message
                .to_owned()
                .edit(&ctx.http, |m| {
                    m.content(format!("```\n{}\n```", new_config))
                })
                .await?;
        }
        None => {
            if action == "remove" {
                let response_content = format!(
                    "WhitelistError: No names to remove from in guild id: {}",
                    guild_id
                );
                command
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(response_content.to_string())
                    })
                    .await?;
                return Err(response_content.into());
            }
            if use_uuid {
                players.insert(uuid, splits_data);
            } else {
                players.insert(ign, splits_data);
            }
            let new_config = get_new_config_contents(players);
            channel
                .send_message(&ctx.http, |m| {
                    m.content(format!("```\n{}\n```", new_config))
                })
                .await?;
        }
    };
    command
        .edit_original_interaction_response(&ctx.http, |m| m.content("Updated config!"))
        .await?;
    Ok(())
}
