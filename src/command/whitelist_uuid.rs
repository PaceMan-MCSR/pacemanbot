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
        "whitelist_uuid_17"
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
