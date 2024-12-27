use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::{cache::split::Split, utils::create_guild_role::create_guild_role, Result};

pub async fn setup_pb_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    command.defer_ephemeral(&ctx).await?;
    let splits: Vec<Split> = vec![Split::TowerStart, Split::EndEnter];
    for split in splits {
        let role_name = format!("*17{}PB", split.to_str());
        create_guild_role(&ctx, &guild, &role_name).await?;
    }
    command
        .edit_original_interaction_response(&ctx.http, |data| {
            data.content("PB Pace pace-roles have been setup!")
        })
        .await?;
    Ok(())
}
