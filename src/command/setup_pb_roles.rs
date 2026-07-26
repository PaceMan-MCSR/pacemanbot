use std::error::Error;

use serenity::{async_trait, builder::CreateApplicationCommand};

use crate::{
    cache::Split,
    command::{create_guild_role, Command, CommandContext},
    config::ROLE_PREFIX,
};

pub struct SetupPBRoles;

#[async_trait]
impl Command for SetupPBRoles {
    fn name(&self) -> &str {
        "setup_pb_roles_17"
    }

    fn description(&self) -> &str {
        "Setup split PB pace-roles(as specified per runner)."
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

        let splits: Vec<Split> = vec![Split::TowerStart, Split::EndEnter];
        for split in splits {
            let role_name = format!("{}{}PB", ROLE_PREFIX, split.to_str());
            create_guild_role(ctx, &context.guild_id, &role_name).await?;
        }

        command
            .edit_original_interaction_response(&ctx.http, |data| {
                data.content("PB pace-roles have been setup!")
            })
            .await?;
        Ok(())
    }
}

pub const SETUP_PB_ROLES: SetupPBRoles = SetupPBRoles {};
