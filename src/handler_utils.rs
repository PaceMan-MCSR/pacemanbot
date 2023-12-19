use serenity::{
    client::Context,
    model::{
        prelude::{
            application_command::ApplicationCommandInteraction,
            message_component::MessageComponentInteraction, Activity, GuildId, Interaction, RoleId,
        },
        user::OnlineStatus,
    },
};

use crate::{
    components::{
        send_role_selection_message, setup_default_commands, setup_default_roles, setup_roles,
    },
    utils::remove_roles_starting_with,
};

pub async fn handle_guild_create(ctx: &Context, guild_id: GuildId) {
    setup_default_commands(&ctx, guild_id).await;
    ctx.set_presence(Some(Activity::watching("paceman.gg")), OnlineStatus::Online)
        .await;
}

pub async fn handle_interaction_create(ctx: &Context, interaction: Interaction) {
    if let Some(command) = interaction.as_application_command() {
        handle_application_command_interaction(ctx, command).await;
    }
    if let Some(message_component) = interaction.as_message_component() {
        handle_message_component_interaction(ctx, message_component).await;
    }
}

pub async fn handle_remove_pmb_roles(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    message_component.defer_ephemeral(&ctx).await?;

    let guild_id = match message_component.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Err(format!(
                "Unable to get guild id for message component: {:#?}.",
                message_component,
            )
            .into())
        }
    };
    let member = match message_component.member.as_ref() {
        Some(member) => member,
        None => {
            return Err(format!(
                "Unable to get member for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let mut member = guild_id.member(&ctx, member.user.id).await?;

    // Remove all PMB roles
    remove_roles_starting_with(&ctx, &guild_id, &mut member, "*").await;

    // Respond to the interaction
    message_component
        .edit_original_interaction_response(&ctx.http, |r| r.content("PaceManBot roles removed"))
        .await?;
    Ok(())
}

pub async fn handle_select_role(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
    split: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    message_component.defer_ephemeral(&ctx).await?;

    let guild_id = match message_component.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Err(format!(
                "Unable to get guild id for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let member = match message_component.member.as_ref() {
        Some(member) => member,
        None => {
            return Err(format!(
                "Unable to get member for message component: {:#?}.",
                message_component
            )
            .into())
        }
    };
    let mut member = guild_id.member(&ctx, member.user.id).await?;

    // Remove all PMB roles
    remove_roles_starting_with(&ctx, &guild_id, &mut member, format!("*{}", split).as_str()).await;

    // Add the new roles
    let mut roles_to_add = Vec::new();
    for value in &message_component.data.values {
        roles_to_add.push(RoleId(value.parse::<u64>()?));
    }
    member.add_roles(&ctx, &roles_to_add).await?;

    // Respond to the interaction
    message_component
        .edit_original_interaction_response(&ctx.http, |r| r.content("Roles updated"))
        .await?;

    Ok(())
}

pub async fn handle_application_command_interaction(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) {
    let guild_id = match command.guild_id {
        Some(guild_id) => guild_id,
        None => {
            eprintln!(
                "Unable to get guild id for the command: {}.",
                command.data.name
            );
            return;
        }
    };
    let roles = match guild_id.roles(&ctx.http).await {
        Ok(roles) => roles,
        Err(err) => {
            eprintln!(
                "Unable to get roles for guild id: {} due to: {}",
                guild_id, err
            );
            return;
        }
    };
    match match command.data.name.as_str() {
        "send_message" => send_role_selection_message(&ctx, &roles, command).await,
        "setup_default_roles" => setup_default_roles(&ctx, guild_id, command).await,
        "setup_roles" => setup_roles(&ctx, guild_id, command).await,
        _ => {
            eprintln!("Unrecognized command: {}.", command.data.name);
            return;
        }
    } {
        Ok(_) => (),
        Err(err) => eprintln!(
            "Unable to handle command: {} due to: {}",
            command.data.name, err
        ),
    };
}

pub async fn handle_message_component_interaction(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) {
    let custom_id = match message_component.data.custom_id.as_str() {
        "remove_pmb_roles" => handle_remove_pmb_roles(&ctx, &message_component).await,
        "select_structure1_role" => handle_select_role(&ctx, &message_component, "FS").await,
        "select_structure2_role" => handle_select_role(&ctx, &message_component, "SS").await,
        "select_blind_role" => handle_select_role(&ctx, &message_component, "B").await,
        "select_eye_spy_role" => handle_select_role(&ctx, &message_component, "E").await,
        "select_end_enter_role" => handle_select_role(&ctx, &message_component, "EE").await,
        _ => Err(format!("Unknown custom id: {}.", message_component.data.custom_id).into()),
    };
    match custom_id {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error while handling interaction: {}", err);
        }
    };
}
