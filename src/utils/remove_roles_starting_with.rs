use serenity::{
    client::Context,
    model::{guild::Member, id::GuildId},
};

use crate::{
    cache::consts::{ROLE_PREFIX_115, ROLE_PREFIX_17, ROLE_PREFIX_AA},
    Result,
};

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
        if role.name.starts_with(role_prefix)
            && !role.name.starts_with(ROLE_PREFIX_115)
            && !role.name.starts_with(ROLE_PREFIX_17)
            && !role.name.starts_with(ROLE_PREFIX_AA)
        {
            if skip_pb_roles && role.name.contains("PB") {
                continue;
            }
            member.remove_role(&ctx.http, role_id).await?;
        }
    }
    Ok(())
}
