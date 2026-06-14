use std::error::Error;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

pub struct CommandContext<'a> {
    pub ctx: &'a Context,
    pub guild_id: GuildId,
    pub interaction: &'a ApplicationCommandInteraction,
}

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn create_options<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand;
    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>>;
}
