use std::error::Error;

use serenity::{
    client::Context, model::prelude::application_command::ApplicationCommandInteraction,
};

use crate::command::{get_default_commands, CommandContext};

pub async fn handle_application_command_interaction(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn Error>> {
    let guild_id = match command.guild_id {
        Some(guild_id) => guild_id,
        None => {
            let content = format!(
                "failed to get guild id for command: '{}'.",
                command.data.name
            );
            command
                .create_interaction_response(&ctx.http, |i| {
                    i.interaction_response_data(|m| m.content(content.to_string()))
                })
                .await?;
            return Err(content.into());
        }
    };
    let default_command = get_default_commands()
        .into_iter()
        .find(|default_command| default_command.name() == command.data.name.as_str());
    let default_command = match default_command {
        Some(default_command) => default_command,
        None => {
            let content = format!("failed to handle command: {}", command.data.name);
            return Err(content.into());
        }
    };

    match default_command
        .execute(CommandContext {
            ctx,
            guild_id,
            interaction: command,
        })
        .await
    {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("failed to execute command: {}", err).into());
        }
    };
    Ok(())
}
