use serenity::{
    client::Context,
    model::{guild::Member, id::GuildId},
};

use crate::Result;

pub async fn remove_roles_starting_with(
    ctx: &Context,
    guild_id: &GuildId,
    member: &mut Member,
    role_prefix: &str,
    skip_pb_roles: bool,
) -> Result<()> {
    let guild_roles = guild_id.roles(&ctx.http).await?;
    for role_id in member.roles.clone() {
        let role = guild_roles.get(&role_id).unwrap().clone();
        if role.name.starts_with(role_prefix) {
            if skip_pb_roles && role.name.contains("PB") {
                continue;
            }
            member.remove_role(&ctx.http, role_id).await?;
        }
    }
    Ok(())
}
