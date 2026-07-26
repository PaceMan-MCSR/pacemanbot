use std::error::Error;

use serenity::{
    async_trait, builder::CreateApplicationCommand, model::prelude::command::CommandOptionType,
};

use super::whitelist::update_whitelist;

use crate::command::{Command, CommandContext};

pub struct WhitelistUUID;

#[async_trait]
impl Command for WhitelistUUID {
    fn name(&self) -> &str {
        "whitelist_uuid_aa"
    }

    fn description(&self) -> &str {
        "Whitelist new players or edit old players' configurations in the server based on uuid."
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
                    .name("uuid")
                    .description("UUID (with hyphens '-') of the runner that you want to add.")
                    .required(true)
                    .kind(CommandOptionType::String)
            })
            .create_option(|option| {
                option
                    .name("adventuring_time_hours")
                    .description(
                        "The time for adventuring time (hours) that you want to setup for the runner.",
                    )
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("adventuring_time_minutes")
                    .description(
                        "The time for adventuring time (minutes) that you want to setup for the runner.",
                    )
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("beaconator_hours")
                    .description(
                        "The time for beaconator (hours) that you want to setup for the runner.",
                    )
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("beaconator_minutes")
                    .description(
                        "The time for beaconator (minutes) that you want to setup for the runner.",
                    )
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("hdwgh_hours")
                    .description("The time for hdwgh (hours) that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("hdwgh_minutes")
                    .description("The time for hdwgh (minutes) that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("finish_hours")
                    .description(
                        "The time for completion (hours) that you want to setup for the runner(optional).",
                    )
                    .kind(CommandOptionType::Integer)
            })
						.create_option(|option| {
                option
                    .name("finish_minutes")
                    .description(
                        "The time for completion (minutes) that you want to setup for the runner(optional).",
                    )
                    .kind(CommandOptionType::Integer)
            })
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let response_content =
            update_whitelist(context.ctx, context.guild_id, context.interaction, true).await?;
        context
            .interaction
            .edit_original_interaction_response(&context.ctx.http, |data| {
                data.content(response_content)
            })
            .await?;
        Ok(())
    }
}

pub const WHITELIST_UUID: WhitelistUUID = WhitelistUUID {};
