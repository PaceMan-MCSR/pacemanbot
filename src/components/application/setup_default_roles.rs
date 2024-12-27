use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::{utils::create_guild_role::create_guild_role, Result};

pub async fn setup_default_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    command.defer_ephemeral(&ctx).await?;

    let default_roles = [
        "*115F6:0",
        "*115F5:3",
        "*115F5:0",
        "*115F4:3",
        "*115B8:0",
        "*115B7:3",
        "*115B7:0",
        "*115B6:3",
        "*115B6:0",
        "*115B5:3",
        "*115E9:3",
        "*115E9:0",
        "*115E8:3",
        "*115E8:0",
        "*115EE8:3",
        "*115EE9:0",
        "*115EE9:3",
        "*115EE10:0",
    ];

    for role in default_roles.iter() {
        match create_guild_role(&ctx, &guild, &role.to_string()).await {
            Ok(_) => (),
            Err(err) => {
                let content = format!("SetupDefaultRolesError: {}", err);
                return Err(content.into());
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
