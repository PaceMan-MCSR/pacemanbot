use std::error::Error;

use serenity::{
    async_trait, builder::CreateApplicationCommand, model::prelude::command::CommandOptionType,
};

use crate::{
    cache::Split,
    command::{create_guild_role, Command, CommandContext},
    config::ROLE_PREFIX,
};

pub struct SetupRoles;

#[async_trait]
impl Command for SetupRoles {
    fn name(&self) -> &str {
        "setup_roles_17"
    }

    fn description(&self) -> &str {
        "Setup pace-roles based on split, start time and end time in increments of 30s."
    }

    fn create_options<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .create_option(|option| {
                option
                    .name("split_name")
                    .description("The name of the split.")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice("Tower Start", "tower_start")
                    .add_string_choice("End Enter", "end_enter")
            })
            .create_option(|option| {
                option
                    .name("split_start")
                    .description("The lower bound for the split in minutes.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("split_end")
                    .description("The upper bound for the split in minutes.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let ctx = context.ctx;
        let command = context.interaction;

        let mut split_name = "".to_string();
        let mut split_start = 0;
        let mut split_end = 0;
        for option in command.data.options.iter() {
            match option.name.as_str() {
                "split_name" => {
                    split_name = match option.value.to_owned() {
                        Some(value) => match value.as_str() {
                            Some(str) => str.to_owned(),
                            None => {
                                return Err("failed to convert 'split_name' into '&str'.".into())
                            }
                        },
                        None => {
                            return Err("failed to get value for option name: 'split_name'.".into())
                        }
                    }
                }
                "split_start" => {
                    split_start = match option.value.to_owned() {
                        Some(value) => match value.as_u64() {
                            Some(int) => int,
                            None => {
                                return Err("failed to convert 'split_start' into 'u64'.".into())
                            }
                        },
                        None => {
                            return Err("failed to get value for option name: 'split_start'.".into())
                        }
                    }
                }
                "split_end" => {
                    split_end = match option.value.to_owned() {
                        Some(value) => match value.as_u64() {
                            Some(int) => int,
                            None => return Err("failed to convert 'split_end' into 'u64'.".into()),
                        },
                        None => {
                            return Err("failed to get value for option name: 'split_end'.".into())
                        }
                    }
                }
                _ => return Err("failed to get option name.".into()),
            };
        }

        let role_split = match Split::from_command_param(split_name.as_str()) {
            Some(split) => split,
            None => return Err(format!("failed to get split name: '{}'.", split_name).into()),
        };

        for minutes in split_start..split_end {
            let seconds = 0;
            let role = format!(
                "{}{}{}:{}",
                ROLE_PREFIX,
                role_split.to_str(),
                minutes,
                seconds
            );
            create_guild_role(ctx, &context.guild_id, &role).await?;

            let seconds = 3;
            let role = format!(
                "{}{}{}:{}",
                ROLE_PREFIX,
                role_split.to_str(),
                minutes,
                seconds
            );
            create_guild_role(ctx, &context.guild_id, &role).await?;
        }
        let seconds = 0;
        let role = format!(
            "{}{}{}:{}",
            ROLE_PREFIX,
            role_split.to_str(),
            split_end,
            seconds
        );
        create_guild_role(ctx, &context.guild_id, &role).await?;

        let response_content = format!(
        "Pace-roles for split name: {} with lower bound: {} minutes and upper bound: {} minutes have been setup!",
        split_name, split_start, split_end
    );

        command
            .edit_original_interaction_response(&ctx.http, |data| data.content(response_content))
            .await?;
        Ok(())
    }
}

pub const SETUP_ROLES: SetupRoles = SetupRoles {};
