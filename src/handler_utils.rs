use std::{sync::Arc, time::Duration};

use serenity::{
    client::Context,
    futures::StreamExt,
    model::{
        channel::GuildChannel,
        guild::Role,
        id::ChannelId,
        prelude::{
            application_command::ApplicationCommandInteraction,
            message_component::MessageComponentInteraction, Activity, GuildId, Interaction, RoleId,
        },
        user::OnlineStatus,
    },
};
use tokio::time::sleep;

use crate::{
    components::{
        send_role_selection_message, setup_default_commands, setup_default_roles, setup_pb_roles,
        setup_roles, validate_config,
    },
    consts::WS_TIMEOUT_FOR_RETRY,
    controller::Controller,
    guild_types::{CachedGuilds, GuildData, Split},
    response_types::Response,
    utils::{get_response_stream_from_api, remove_roles_starting_with},
    ArcMux,
};

pub async fn handle_guild_create(
    ctx: &Context,
    guild_id: GuildId,
    guild_cache: ArcMux<CachedGuilds>,
) {
    setup_default_commands(&ctx, guild_id).await;
    ctx.set_presence(Some(Activity::watching("paceman.gg")), OnlineStatus::Online)
        .await;
    let mut locked_guild_cache = guild_cache.lock().await;
    let guild_data = match GuildData::new(ctx, guild_id).await {
        Ok(data) => data,
        Err(err) => {
            return eprintln!(
                "Unable to fetch guild data for guild id: {} due to: {}",
                guild_id, err
            );
        }
    };
    locked_guild_cache.insert(guild_id, guild_data);
}

pub async fn handle_interaction_create(ctx: &Context, interaction: Interaction) {
    if let Some(command) = interaction.as_application_command() {
        handle_application_command_interaction(ctx, command).await;
    }
    if let Some(message_component) = interaction.as_message_component() {
        match message_component.defer_ephemeral(&ctx).await {
            Ok(_) => (),
            Err(err) => {
                eprintln!(
                    "Unable to defer_ephemeral on message_component due to: {}",
                    err
                );
                return;
            }
        };
        handle_message_component_interaction(ctx, message_component).await;
    }
}

pub async fn handle_update_cache(
    ctx: &Context,
    guild_id: GuildId,
    guild_cache: ArcMux<CachedGuilds>,
) {
    let mut locked_guild_cache = guild_cache.lock().await;
    let guild_data = match locked_guild_cache.get_mut(&guild_id) {
        Some(data) => data,
        None => {
            return eprintln!("Unable to get guild data for guild id: {}", guild_id);
        }
    };
    match guild_data.refetch(&ctx, guild_id).await {
        Ok(_) => (),
        Err(err) => {
            eprintln!(
                "Unable to refetch guild data for guild id: {} due to: {}",
                guild_id, err
            )
        }
    }
}

pub async fn handle_message_events(
    ctx: &Context,
    channel_id: ChannelId,
    guild_id: GuildId,
    guild_cache: ArcMux<CachedGuilds>,
) {
    let name = match channel_id.name(&ctx.cache).await {
        Some(name) => name,
        None => {
            return eprintln!("Unable to get guild name for channel id: {}.", channel_id);
        }
    };
    if name != "pacemanbot-runner-names" {
        return println!(
            "Skipping message delete because it was not sent in #pacemanbot-runner-names.",
        );
    }
    handle_update_cache(ctx, guild_id, guild_cache).await;
}

pub async fn handle_channel_events(
    ctx: &Context,
    channel: &GuildChannel,
    guild_id: GuildId,
    guild_cache: ArcMux<CachedGuilds>,
) {
    match channel.name.as_str() {
        "pacemanbot-runner-names" | "pacemanbot" | "pacemanbot-runner-leaderboard" => {
            handle_update_cache(ctx, guild_id, guild_cache).await;
        }
        _ => {
            return println!(
                "Skipping channel event because it is not something that concerns the bot."
            )
        }
    }
}

pub async fn handle_guild_role_events(
    ctx: &Context,
    new: Role,
    guild_id: GuildId,
    guild_cache: ArcMux<CachedGuilds>,
) {
    if !new.name.starts_with("*") {
        return println!(
            "Skipping role create event because it is not something that concerns the bot."
        );
    }
    handle_update_cache(ctx, guild_id, guild_cache).await
}

pub async fn handle_remove_pmb_roles(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
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

    remove_roles_starting_with(&ctx, &guild_id, &mut member, "*", false).await?;

    message_component
        .edit_original_interaction_response(&ctx.http, |r| r.content("PaceManBot roles removed"))
        .await?;
    Ok(())
}

pub async fn handle_select_role(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
    split: Split,
) -> Result<(), Box<dyn std::error::Error>> {
    let split_str = split.to_str();
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

    let mut remove_roles = true;
    let mut roles_to_add = Vec::new();
    for value in &message_component.data.values {
        let role_id = RoleId(value.parse::<u64>()?);
        let role_name = match role_id.to_role_cached(&ctx.cache) {
            Some(role) => role.name,
            None => {
                return Err(format!(
                    "Unable to convert role id: {} to role for guild id: {}.",
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
            format!("*{}", split_str).as_str(),
            true,
        )
        .await?;
    } else {
        let member_roles = match member.roles(&ctx) {
            Some(roles) => roles,
            None => {
                return Err(format!(
                    "Unable to get roles for member with name: {}.",
                    member.display_name()
                )
                .into())
            }
        };
        for role in member_roles {
            if role.name.starts_with(&format!("*{}", split_str)) && role.name.contains("PB") {
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
        "setup_pb_roles" => setup_pb_roles(&ctx, guild_id, command).await,
        "validate_config" => validate_config(&ctx, guild_id, command).await,
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
        "select_structure1_role" => {
            handle_select_role(&ctx, &message_component, Split::FirstStructure).await
        }
        "select_structure2_role" => {
            handle_select_role(&ctx, &message_component, Split::SecondStructure).await
        }
        "select_blind_role" => handle_select_role(&ctx, &message_component, Split::Blind).await,
        "select_eye_spy_role" => handle_select_role(&ctx, &message_component, Split::EyeSpy).await,
        "select_end_enter_role" => {
            handle_select_role(&ctx, &message_component, Split::EndEnter).await
        }
        _ => Err(format!("Unknown custom id: {}.", message_component.data.custom_id).into()),
    };
    match custom_id {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error while handling interaction: {}", err);
        }
    };
}

pub async fn handle_ready(ctx: Arc<Context>, guild_cache: ArcMux<CachedGuilds>) {
    let mut locked_guild_cache = guild_cache.lock().await;
    for guild_id in ctx.cache.guilds() {
        let guild_data = match GuildData::new(&ctx, guild_id).await {
            Ok(data) => data,
            Err(err) => {
                eprintln!(
                    "Unable to generate cache for guild id: {} due to: {}",
                    guild_id, err
                );
                continue;
            }
        };
        locked_guild_cache.insert(guild_id, guild_data);
    }
    drop(locked_guild_cache);
    loop {
        let mut response_stream = match get_response_stream_from_api().await {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("{}", err);
                println!("Trying again in {} seconds...", WS_TIMEOUT_FOR_RETRY);
                sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
                continue;
            }
        };
        loop {
            let msg = match match response_stream.next().await {
                Some(msg_result) => msg_result,
                None => {
                    println!(
                        "Invalid response from response stream.\nTrying again in {} seconds...",
                        WS_TIMEOUT_FOR_RETRY
                    );
                    sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
                    break;
                }
            } {
                Ok(msg) => msg,
                Err(err) => {
                    eprintln!("Unable to get message from response stream due to: {}", err);
                    println!("Trying again in {} seconds...", WS_TIMEOUT_FOR_RETRY);
                    sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
                    break;
                }
            };
            let text_response = match msg.to_text() {
                Ok(text) => text,
                Err(err) => {
                    eprintln!(
                        "Unable to get text response from response stream due to: {}",
                        err
                    );
                    println!("Trying again in {} seconds...", WS_TIMEOUT_FOR_RETRY);
                    sleep(Duration::from_secs(WS_TIMEOUT_FOR_RETRY)).await;
                    break;
                }
            };
            let record: Response = match serde_json::from_str(text_response) {
                Ok(response) => response,
                Err(err) => {
                    eprintln!(
                        "Unable to convert text response: '{}' to json due to: {}",
                        text_response, err
                    );
                    continue;
                }
            };
            let ctx = ctx.clone();
            let guild_cache = guild_cache.clone();
            Controller::new(ctx, record, guild_cache).start().await;
        }
    }
}
