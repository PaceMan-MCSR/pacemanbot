use std::error::Error;

use serenity::{async_trait, builder::CreateApplicationCommand};

use crate::{
    command::{create_guild_role, Command, CommandContext},
    config::ROLE_PREFIX,
};

pub struct SetupDefaultRoles;

#[async_trait]
impl Command for SetupDefaultRoles {
    fn name(&self) -> &str {
        "setup_default_roles"
    }

    fn description(&self) -> &str {
        "Setup default pace-roles for sub 10."
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

        let default_roles = [
            "FS2:0", "FS2:3", "FS3:0", "SS6:0", "SS5:3", "SS5:0", "SS4:3", "B8:0", "B7:3", "B7:0",
            "B6:3", "B6:0", "B5:3", "E9:3", "E9:0", "E8:3", "E8:0", "EE8:3", "EE9:0", "EE9:3",
            "EE10:0",
        ];

        for role in default_roles.iter() {
            match create_guild_role(ctx, &context.guild_id, &format!("{}{}", ROLE_PREFIX, role))
                .await
            {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!("failed to setup default roles: {}", err).into());
                }
            }
        }
        command
            .edit_original_interaction_response(&ctx.http, |data| {
                data.content("Default pace-roles have been setup!")
            })
            .await?;
        Ok(())
    }
}

pub const SETUP_DEFAULT_ROLES: SetupDefaultRoles = SetupDefaultRoles {};
