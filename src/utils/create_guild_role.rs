use serenity::{client::Context, model::id::GuildId};

use crate::Result;

use super::consts::ROLE_COLOR;

pub async fn create_guild_role(ctx: &Context, guild: &GuildId, role_name: &String) -> Result<()> {
    let roles = guild.roles(&ctx.http).await?;
    if !roles
        .iter()
        .any(|(_, role)| role.name == role_name.to_string())
    {
        guild
            .create_role(ctx, |r| r.name(role_name).colour(ROLE_COLOR.into()))
            .await?;
    }
    Ok(())
}
