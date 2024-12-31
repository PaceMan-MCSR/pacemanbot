use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::{cache::consts::ROLE_PREFIX, utils::create_guild_role::create_guild_role, Result};

pub async fn setup_default_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    command.defer_ephemeral(&ctx).await?;

    let default_roles = [
        "FS2:0", "FS2:3", "FS3:0", "SS6:0", "SS5:3", "SS5:0", "SS4:3", "B8:0", "B7:3", "B7:0",
        "B6:3", "B6:0", "B5:3", "E9:3", "E9:0", "E8:3", "E8:0", "EE8:3", "EE9:0", "EE9:3",
        "EE10:0",
    ];

    for role in default_roles.iter() {
        match create_guild_role(&ctx, &guild, &format!("{}{}", ROLE_PREFIX, role)).await {
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
