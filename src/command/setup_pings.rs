use std::{error::Error, sync::Arc};

use serenity::{
    async_trait, builder::CreateApplicationCommand, model::prelude::command::CommandOptionType,
};

use crate::{
    cache::{GuildCacheEntry, Split},
    command::{create_guild_role, remove_runner_pings, Command, CommandContext},
    config::{Config, ROLE_PREFIX},
};

pub struct SetupPings;

#[async_trait]
impl Command for SetupPings {
    fn name(&self) -> &str {
        "setup_pings_115"
    }

    fn description(&self) -> &str {
        "Setup pings for specific runners."
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
                    .description("In-game name of the runner you want to setup pings for.")
                    .required(true)
                    .kind(CommandOptionType::String)
            })
            .create_option(|option| {
                option
                    .name("split")
                    .description("Split name for the runner that you want to change.")
                    .required(true)
                    .kind(CommandOptionType::String)
                    .add_string_choice("Enter Nether", Split::EnterNether.to_str())
                    .add_string_choice("Enter Fortress", Split::EnterFortress.to_str())
                    .add_string_choice("Blind", Split::Blind.to_str())
                    .add_string_choice("Eye Spy", Split::EyeSpy.to_str())
                    .add_string_choice("End Enter", Split::EndEnter.to_str())
            })
            .create_option(|option| {
                option
                    .name("time")
                    .description("The time of the split that you want for the runner.")
                    .kind(CommandOptionType::Integer)
            })
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let ctx = context.ctx;
        let guild_id = context.guild_id;
        let command = context.interaction;

        let mut action = String::new();
        let mut ign = String::new();
        let mut split = String::new();
        let mut time = 0;
        for option in command.data.options.iter() {
            match option.name.as_str() {
                "action" => {
                    action = match option.value.to_owned() {
                        Some(value) => match value.as_str() {
                            Some(str) => str.to_owned(),
                            None => {
                                return Err(
                                    "failed to setup pings: convert 'action' value to string."
                                        .into(),
                                )
                            }
                        },
                        None => {
                            return Err(
                                "failed to setup pings: get value for 'action' for command".into()
                            )
                        }
                    }
                }
                "ign" => {
                    ign = match option.value.to_owned() {
                        Some(value) => match value.as_str() {
                            Some(str) => str.to_owned(),
                            None => {
                                return Err(
                                    "failed to setup pings: convert 'ign' value to string.".into()
                                )
                            }
                        },
                        None => {
                            return Err(
                                "failed to setup pings: get value for 'ign' for command".into()
                            )
                        }
                    }
                }
                "split" => {
                    split = match option.value.to_owned() {
                        Some(value) => match value.as_str() {
                            Some(str) => str.to_owned(),
                            None => {
                                return Err(
                                    "failed to setup pings: convert 'split' value to string."
                                        .into(),
                                )
                            }
                        },
                        None => {
                            return Err(
                                "failed to setup pings: get value for 'split' for command.".into()
                            )
                        }
                    }
                }
                "time" => {
                    time = match option.value.to_owned() {
                        Some(value) => match value.as_u64() {
                            Some(int) => int as u8,
                            None => {
                                return Err(
                                    "failed to setup pings: convert 'time' value to u64".into()
                                )
                            }
                        },
                        None => {
                            return Err(
                                "failed to setup pings: get value for 'time' for command.".into()
                            )
                        }
                    }
                }
                _ => (),
            }
        }
        let split = match Split::from_str(split.as_str()) {
            Some(split) => split,
            None => {
                return Err(format!(
                    "failed to setup pings: construct Split from str: '{}'.",
                    split
                )
                .into())
            }
        };
        let guild_data = Config::parse_config_for_guild(&ctx, guild_id).await?;
        let is_private = match GuildCacheEntry::is_private(
            guild_data.name.to_string(),
            Arc::new(ctx.clone()),
            &guild_id,
        ) {
            Ok(is_private) => is_private,
            Err(err) => {
                return Err(format!("failed to setup pings: {}", err).into());
            }
        };
        if is_private
            && !guild_data
                .player_whitelist
                .contains_key(&ign.to_lowercase())
        {
            let response_content = format!(
                "failed to setup pings: Runner with name: '{}' not found in guild.",
                ign
            );
            return Err(response_content.into());
        }
        let mut sender = match command.member.to_owned() {
            Some(sender) => sender,
            None => return Err("failed to setup pings: get member for '/setup_pings'.".into()),
        };
        match action.as_str() {
            "add_or_update" => {
                if time == 0 {
                    return Err(
                        "failed to setup pings: Parameter 'time' is undefined for 'add_or_update'."
                            .into(),
                    );
                }
                remove_runner_pings(
                    ctx,
                    &guild_id,
                    &mut sender,
                    ROLE_PREFIX,
                    split.to_owned(),
                    ign.to_owned(),
                )
                .await?;
                let role_name = format!("{}{}{}:0+{}", ROLE_PREFIX, split.to_str(), time, ign);
                let roles = guild_id.roles(&ctx.http).await?;
                let guild_has_role = roles.iter().any(|(_, r)| r.name == role_name);
                if !guild_has_role {
                    create_guild_role(ctx, &guild_id, &role_name).await?;
                }
                let roles = guild_id.roles(&ctx.http).await?;
                sender
                    .add_role(
                        &ctx.http,
                        roles.iter().find(|(_, r)| r.name == role_name).unwrap().0,
                    )
                    .await?;
                command
                .edit_original_interaction_response(&ctx.http, |m| {
                    m.content(format!(
                        "Added/Updated pings for runner with ign: '{}' for split: '{}' with time: '{}m'",
                        ign,
                        split.desc(),
                        time
                    ))
                })
                .await?;
            }
            "remove" => {
                let roles = guild_id.roles(&ctx.http).await?;
                let role = match roles.iter().find(|(_, r)| {
                    r.name.contains(split.to_str().as_str())
                        && r.name.starts_with(ROLE_PREFIX)
                        && r.name.contains(ign.as_str())
                }) {
                    Some(name) => name,
                    None => {
                        return Err(
                            "failed to setup pings: get role name for 'remove' action.".into()
                        );
                    }
                };
                let role_name = role.1.name.as_str();
                remove_runner_pings(
                    ctx,
                    &guild_id,
                    &mut sender,
                    ROLE_PREFIX,
                    split.to_owned(),
                    ign.to_owned(),
                )
                .await?;
                let roles = guild_id.roles(&ctx.http).await?;
                let guild_has_role = roles.iter().any(|(_, r)| r.name == role_name);
                if guild_has_role {
                    guild_id
                        .delete_role(
                            ctx,
                            roles.iter().find(|(_, r)| r.name == role_name).unwrap().0,
                        )
                        .await?;
                }
                command
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(format!(
                            "Removed pings for runner with ign: '{}' for split: '{}'",
                            ign,
                            split.desc()
                        ))
                    })
                    .await?;
            }
            _ => (),
        }
        Ok(())
    }
}

pub const SETUP_PINGS: SetupPings = SetupPings {};
