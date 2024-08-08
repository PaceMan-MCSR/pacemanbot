use serenity::{
    client::Context, model::prelude::application_command::ApplicationCommandInteraction,
};

use crate::{
    components::application::{
        migrate::migrate, send_role_selection_message::send_role_selection_message,
        setup_default_roles::setup_default_roles, setup_pb_roles::setup_pb_roles,
        setup_pings::setup_pings, setup_roles::setup_roles, validate_config::validate_config,
        whitelist::whitelist,
    },
    Result,
};

pub async fn handle_application_command_interaction(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    let guild_id = match command.guild_id {
        Some(guild_id) => guild_id,
        None => {
            let content = format!(
                "ApplicationCommandInteractionError: get guild id for command: '{}'.",
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
    let roles = match guild_id.roles(&ctx.http).await {
        Ok(roles) => roles,
        Err(err) => {
            let content = format!(
                "ApplicationCommandInteractionError: get roles for guild id {}: {}",
                guild_id, err
            );
            command
                .create_interaction_response(&ctx.http, |i| {
                    i.interaction_response_data(|m| m.content(content.to_string()))
                })
                .await?;
            return Err(content.into());
        }
    };
    match match command.data.name.as_str() {
        "send_message" => send_role_selection_message(&ctx, &roles, command).await,
        "setup_default_roles" => setup_default_roles(&ctx, guild_id, command).await,
        "setup_pings" => setup_pings(&ctx, guild_id, command).await,
        "setup_roles" => setup_roles(&ctx, guild_id, command).await,
        "setup_pb_roles" => setup_pb_roles(&ctx, guild_id, command).await,
        "whitelist" => whitelist(&ctx, guild_id, command).await,
        "migrate" => migrate(&ctx, guild_id, command).await,
        "validate_config" => validate_config(&ctx, guild_id, command).await,
        _ => {
            return Err(format!(
                "ApplicationCommandInteractionError: Unrecognized command: {}.",
                command.data.name
            )
            .into());
        }
    } {
        Ok(_) => (),
        Err(err) => {
            let content = format!(
                "ApplicationCommandInteractionError: handle command: {}",
                err
            );
            return Err(content.into());
        }
    };
    Ok(())
}
