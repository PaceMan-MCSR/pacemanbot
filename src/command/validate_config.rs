use std::error::Error;

use serenity::{async_trait, builder::CreateApplicationCommand};

use crate::{
    command::{Command, CommandContext},
    config::{Config, PACEMANBOT_CHANNEL},
};

pub struct ValidateConfig;

#[async_trait]
impl Command for ValidateConfig {
    fn name(&self) -> &str {
        "validate_config_115"
    }

    fn description(&self) -> &str {
        "Check if the current server configuration is valid and if the bot will work properly or not."
    }

    fn create_options<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let ctx = context.ctx;
        let command = context.interaction;

        let reply_content;
        match Config::parse_config_for_guild(ctx, context.guild_id).await {
            Ok(_) => {
                reply_content = format!(
                    "Config validation successful! Bot will send paces in #{}.",
                    PACEMANBOT_CHANNEL
                )
            }
            Err(err) => reply_content = format!("Error: {}", err),
        };

        command
            .edit_original_interaction_response(&ctx.http, |m| m.content(reply_content))
            .await?;
        Ok(())
    }
}

pub const VALIDATE_CONFIG: ValidateConfig = ValidateConfig {};
