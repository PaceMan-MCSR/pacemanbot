use serenity::{client::Context, model::prelude::message_component::MessageComponentInteraction};

use crate::{utils::remove_roles_starting_with::remove_roles_starting_with, Result};

pub async fn handle_remove_pmb_roles(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<()> {
    let guild_id = match message_component.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Err(format!(
                "RemoveRolesError: get guild id for message component: {:#?}.",
                message_component,
            )
            .into())
        }
    };
    let member = match message_component.member.as_ref() {
        Some(member) => member,
        None => {
            return Err(format!(
                "RemoveRolesError: get member for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let mut member = guild_id.member(&ctx, member.user.id).await?;

    remove_roles_starting_with(&ctx, &guild_id, &mut member, "*115", false).await?;

    message_component
        .edit_original_interaction_response(&ctx.http, |r| r.content("PaceManBot roles removed"))
        .await?;
    Ok(())
}
