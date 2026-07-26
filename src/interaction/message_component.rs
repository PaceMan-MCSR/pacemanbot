use std::error::Error;

use serenity::{client::Context, model::prelude::message_component::MessageComponentInteraction};

use crate::{
    cache::Split,
    interaction::{handle_remove_pmb_roles, handle_select_role},
};

pub async fn handle_message_component_interaction(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<(), Box<dyn Error>> {
    let custom_id = match message_component.data.custom_id.as_str() {
        "remove_pmb_roles" => handle_remove_pmb_roles(&ctx, &message_component).await,
        "select_tower_start_role" => {
            handle_select_role(&ctx, &message_component, Split::TowerStart).await
        }
        "select_end_enter_role" => {
            handle_select_role(&ctx, &message_component, Split::EndEnter).await
        }
        _ => Err(format!("unknown custom id: {}.", message_component.data.custom_id).into()),
    };
    match custom_id {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("error while handling interaction: {}", err).into());
        }
    };
    Ok(())
}
