use std::{collections::HashMap, error::Error};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        id::GuildId,
        prelude::{application_command::ApplicationCommandInteraction, command::CommandOptionType},
    },
};

use crate::{
    cache::PlayerCacheEntry,
    command::{get_new_config_contents, Command, CommandContext},
    config::{extract_name_or_uuid_and_splits_from_config_line, PACEMANBOT_RUNNER_NAMES_CHANNEL},
};

pub struct Whitelist;

#[async_trait]
impl Command for Whitelist {
    fn name(&self) -> &str {
        "whitelist_17"
    }

    fn description(&self) -> &str {
        "Whitelist new players or edit old players' configurations in the server based on ign."
    }

    fn create_options<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .create_option(|option| {
                option
                    .name("action")
                    .description("Action to perform out of 'add_or_update' or 'remove'.")
                    .required(true)
                    .kind(CommandOptionType::String)
                    .add_string_choice("Add or Update", "add_or_update")
                    .add_string_choice("Remove", "remove")
            })
            .create_option(|option| {
                option
                    .name("ign")
                    .description("In-game name of the runner that you want to add.")
                    .required(true)
                    .kind(CommandOptionType::String)
            })
            .create_option(|option| {
                option
                    .name("tower_start")
                    .description("The time for tower start that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("end_enter")
                    .description("The time for end enter that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("finish")
                    .description(
                        "The time for completion that you want to setup for the runner(optional).",
                    )
                    .kind(CommandOptionType::Integer)
            })
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let response_content =
            update_whitelist(context.ctx, context.guild_id, context.interaction, false).await?;
        context
            .interaction
            .edit_original_interaction_response(&context.ctx.http, |data| {
                data.content(response_content)
            })
            .await?;
        Ok(())
    }
}

pub(super) async fn update_whitelist(
    ctx: &Context,
    guild_id: GuildId,
    command: &ApplicationCommandInteraction,
    use_uuid: bool,
) -> Result<String, Box<dyn Error>> {
    let channels = match ctx.cache.guild_channels(guild_id) {
        Some(channels) => channels,
        None => {
            return Err(format!("failed to get channels for guild id: {}", guild_id).into());
        }
    };
    let mut action = String::new();
    let mut ign = String::new();
    let mut uuid = String::new();
    let mut splits_data = PlayerCacheEntry::default();

    for option in command.data.options.iter() {
        match option.name.as_str() {
            "action" => {
                action = match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err(
                                String::from("failed to parse string for action option.").into()
                            )
                        }
                    },
                    None => {
                        return Err(String::from("failed to get value for action option.").into())
                    }
                }
            }
            "ign" => match option.value.to_owned() {
                Some(value) => {
                    ign = match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err(
                                String::from("failed to parse string for ign option.").into()
                            )
                        }
                    }
                }
                None => return Err(String::from("failed to get value for ign option.").into()),
            },
            "uuid" => match option.value.to_owned() {
                Some(value) => {
                    uuid = match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err(
                                String::from("failed to parse string for uuid option.").into()
                            )
                        }
                    }
                }
                None => return Err(String::from("failed to get value for uuid option.").into()),
            },
            "tower_start" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.tower_start = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(
                                String::from("failed to parse u64 for tower start option.").into()
                            )
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(String::from(
                            "failed to get value for first structure option.",
                        )
                        .into());
                    }
                }
            },
            "end_enter" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.end_enter = match value.as_u64() {
                        Some(int) => int as u8,
                        None => {
                            return Err(
                                String::from("failed to parse u64 for end enter option.").into()
                            )
                        }
                    }
                }
                None => {
                    if action != "remove" {
                        return Err(
                            String::from("failed to get value for end enter option.").into()
                        );
                    }
                }
            },
            "finish" => match option.value.to_owned() {
                Some(value) => {
                    splits_data.finish = match value.as_u64() {
                        Some(int) => Some(int as u8),
                        None => {
                            return Err(
                                String::from("failed to parse u64 for end enter option.").into()
                            )
                        }
                    }
                }
                None => splits_data.finish = None,
            },
            _ => return Err(format!("unrecognized command option: '{}'", option.name).into()),
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
                "failed to find #{} in guild id: {}",
                PACEMANBOT_RUNNER_NAMES_CHANNEL, guild_id
            )
            .into())
        }
    };
    let message = channel.messages(&ctx.http, |m| m.limit(1)).await?;
    let mut players: HashMap<String, PlayerCacheEntry> = HashMap::new();
    match message.last() {
        Some(message) => {
            if !message.author.bot {
                return Err(format!(
                    "failed as the first message in #{} is not from the bot.",
                    PACEMANBOT_RUNNER_NAMES_CHANNEL
                )
                .into());
            }
            for line in message.content.split("\n") {
                if line == "```" || line == "" {
                    continue;
                }
                let (name, split_data) = extract_name_or_uuid_and_splits_from_config_line(line)?;
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
                return Err(
                    format!("failed to remove names from in guild id: {}", guild_id).into(),
                );
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
    Ok("Updated config!".to_string())
}

pub const WHITELIST: Whitelist = Whitelist {};
