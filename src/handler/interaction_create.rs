use serenity::{client::Context, model::prelude::Interaction};

use super::{
    application_command_interaction::handle_application_command_interaction,
    message_component_interaction::handle_message_component_interaction,
};

pub async fn handle_interaction_create(ctx: &Context, interaction: Interaction) {
    if let Some(command) = interaction.as_application_command() {
        match handle_application_command_interaction(ctx, command).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }
    if let Some(message_component) = interaction.as_message_component() {
        match message_component.defer_ephemeral(&ctx).await {
            Ok(_) => (),
            Err(err) => {
                return eprintln!(
                    "InteractionCreateError: defer_ephemeral on message_component failed: {}",
                    err
                );
            }
        };
        match handle_message_component_interaction(ctx, message_component).await {
            Ok(_) => (),
            Err(err) => {
                return eprintln!("InteractionCreateError: {}", err);
            }
        };
    }
}
