use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::Result;

pub async fn migrate(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    command.defer_ephemeral(&ctx).await?;
    let channels = match ctx.cache.guild_channels(guild) {
        Some(channels) => channels,
        None => return Err(format!("MigrateError: get channels for guild id: {}", guild).into()),
    };
    let runner_names_channel = match channels
        .iter()
        .find(|c| c.name == "pacemanbot-runner-names")
    {
        Some(channel) => channel,
        None => {
            let response_content = format!(
                "MigrateError: find channel name: pacemanbot-runner-name in guild id: {}",
                guild
            );
            command
                .edit_original_interaction_response(&ctx.http, |m| {
                    m.content(response_content.to_owned())
                })
                .await?;
            return Err(response_content.into());
        }
    };
    let old_config_messages = match runner_names_channel
        .messages(&ctx.http, |m| m.limit(1))
        .await
    {
        Ok(messages) => messages,
        Err(err) => {
            return Err(format!(
                "MigrateError: get messages in #pacemanbot-runner-names in guild id: {} due to: {}",
                guild, err
            )
            .into())
        }
    };
    let old_config_message = match old_config_messages.last() {
        Some(message) => message,
        None => {
            return Err(format!(
                "MigrateError: get first message in #pacemanbot-runner-names in guild id: {}",
                guild
            )
            .into())
        }
    };
    let is_code_block = old_config_message.content.starts_with("```")
        && old_config_message.content.ends_with("```");
    if old_config_message.author.bot && is_code_block {
        let response_content = format!(
            "MigrateError: Migration has already been performed for guild id: {}.",
            guild
        );
        return Err(response_content.into());
    }
    runner_names_channel
        .send_message(&ctx.http, |m| {
            m.content(format!(
                "```\n{}\n```",
                old_config_message.content.to_string()
            ))
        })
        .await?;
    command
        .edit_original_interaction_response(&ctx.http, |m| {
            m.content(String::from(
                "Migrated old config from first message in #pacemanbot-runner-names! \
                    You can now delete the original first message.",
            ))
        })
        .await?;
    Ok(())
}
