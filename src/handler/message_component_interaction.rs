use serenity::{client::Context, model::prelude::message_component::MessageComponentInteraction};

use crate::{
    cache::split::Split,
    components::message::{
        remove_pmb_roles::handle_remove_pmb_roles, select_role::handle_select_role,
    },
    Result,
};

pub async fn handle_message_component_interaction(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<()> {
    let custom_id = match message_component.data.custom_id.as_str() {
        "remove_pmb_roles" => handle_remove_pmb_roles(&ctx, &message_component).await,
        "select_tower_start_role" => {
            handle_select_role(&ctx, &message_component, Split::TowerStart).await
        }
        "select_end_enter_role" => {
            handle_select_role(&ctx, &message_component, Split::EndEnter).await
        }
        _ => Err(format!("Unknown custom id: {}.", message_component.data.custom_id).into()),
    };
    match custom_id {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error while handling interaction: {}", err).into());
        }
    };
    Ok(())
}
