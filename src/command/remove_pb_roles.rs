use serenity::{async_trait, builder::CreateApplicationCommand};
use std::error::Error;

use crate::{
    command::{Command, CommandContext},
    config::{ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17, ROLE_PREFIX_AA},
};

pub struct RemovePBRoles;

#[async_trait]
impl Command for RemovePBRoles {
    fn name(&self) -> &str {
        "remove_pb_roles"
    }

    fn description(&self) -> &str {
        "Remove all PB pace-roles."
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
        let roles = context.guild_id.roles(&ctx).await?;
        let removeable_roles = roles
            .iter()
            .filter(|(_, r)| {
                r.name.starts_with(ROLE_PREFIX)
                    && !r.name.starts_with(ROLE_PREFIX_115)
                    && !r.name.starts_with(ROLE_PREFIX_17)
                    && !r.name.starts_with(ROLE_PREFIX_AA)
                    && r.name.contains("PB") // should be PB role
                    && !r.name.contains("+") // Skip player pings
            })
            .collect::<Vec<_>>();
        for (role_id, _) in removeable_roles {
            match context.guild_id.delete_role(&ctx, role_id).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!("failed to delete role: {}", err).into());
                }
            };
        }
        match command
            .edit_original_interaction_response(&ctx.http, |m| {
                m.content(format!("Removed all PB pace-roles!",))
            })
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("failed to edit message: {}", err).into());
            }
        };
        Ok(())
    }
}

pub const REMOVE_PB_ROLES: RemovePBRoles = RemovePBRoles {};
