use std::error::Error;

use serenity::{
    client::Context,
    model::{
        guild::Member,
        id::{GuildId, RoleId},
        prelude::message_component::MessageComponentInteraction,
    },
};

use crate::{
    cache::Split,
    config::{ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17, ROLE_PREFIX_AA},
};

pub async fn handle_remove_pmb_roles(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<(), Box<dyn Error>> {
    let guild_id = match message_component.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Err(format!(
                "failed to get guild id for message component: {:#?}.",
                message_component,
            )
            .into())
        }
    };
    let member = match message_component.member.as_ref() {
        Some(member) => member,
        None => {
            return Err(format!(
                "failed to get member for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let mut member = guild_id.member(&ctx, member.user.id).await?;

    remove_roles_starting_with(&ctx, &guild_id, &mut member, ROLE_PREFIX, false).await?;

    message_component
        .edit_original_interaction_response(&ctx.http, |r| r.content("PaceManBot roles removed"))
        .await?;
    Ok(())
}

pub async fn remove_roles_starting_with(
    ctx: &Context,
    guild_id: &GuildId,
    member: &mut Member,
    role_prefix: &str,
    skip_pb_roles: bool,
) -> Result<(), Box<dyn Error>> {
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

pub async fn handle_select_role(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
    split: Split,
) -> Result<(), Box<dyn Error>> {
    let split_str = split.to_str();
    let guild_id = match message_component.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Err(format!(
                "failed to get guild id for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let member = match message_component.member.as_ref() {
        Some(member) => member,
        None => {
            return Err(format!(
                "failed to get member for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let mut member = guild_id.member(&ctx, member.user.id).await?;

    let mut remove_roles = true;
    let mut roles_to_add = Vec::new();
    for value in &message_component.data.values {
        let role_id = RoleId(value.parse::<u64>()?);
        let role_name = match role_id.to_role_cached(&ctx.cache) {
            Some(role) => role.name,
            None => {
                return Err(format!(
                    "failed to convert role id: {} to role for guild id: {}.",
                    role_id, guild_id
                )
                .into())
            }
        };
        if role_name.contains("PB") {
            remove_roles = false;
        }
        roles_to_add.push(role_id);
    }

    if remove_roles {
        remove_roles_starting_with(
            &ctx,
            &guild_id,
            &mut member,
            format!("{}{}", ROLE_PREFIX, split_str).as_str(),
            true,
        )
        .await?;
    } else {
        let member_roles = match member.roles(&ctx) {
            Some(roles) => roles,
            None => {
                return Err(format!(
                    "failed to get roles for member with name: {}.",
                    member.display_name()
                )
                .into())
            }
        };
        for role in member_roles {
            if role
                .name
                .starts_with(&format!("{}{}", ROLE_PREFIX, split_str))
                && role.name.contains("PB")
            {
                member.remove_role(&ctx, role.id).await?;
            }
        }
    }

    member.add_roles(&ctx, &roles_to_add).await?;

    message_component
        .edit_original_interaction_response(&ctx.http, |r| r.content("Roles updated"))
        .await?;

    Ok(())
}
