use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::{
            Interaction, InteractionResponseType::ChannelMessageWithSource,
        },
        gateway::Ready,
        prelude::{
            message_component::MessageComponentInteraction, Activity, Guild, GuildId, RoleId,
        },
        user::OnlineStatus,
    },
    prelude::Mentionable,
};
use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use crate::{
    components::{send_role_selection_message, setup_default_roles},
    consts::TIMEOUT_BETWEEN_CONSECUTIVE_QUERIES,
    types::MojangResponse,
    utils::{
        event_id_to_split, extract_split_from_role_name, format_time, get_response_from_api,
        get_time, sort_guildroles_based_on_split, split_to_desc,
    },
};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let guild_id = guild.id;
        match GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("send_message")
                    .description("Send role message to the current channel.")
            });
            commands.create_application_command(|command| {
                command
                    .name("setup_default_roles")
                    .description("Setup default pace-roles.")
            })
        })
        .await
        {
            Ok(_) => (),
            Err(err) => eprintln!("Error creating command: {}", err),
        };
        ctx.set_presence(Some(Activity::watching("paceman.gg")), OnlineStatus::Online)
            .await;
    }
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        tokio::spawn(async move {
            loop {
                let response = match get_response_from_api().await {
                    Ok(response) => response,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
                let ctx = ctx.clone();
                for record in response.iter() {
                    'guild_loop: for guild_id in guilds.iter() {
                        let channels = match guild_id.channels(&ctx).await {
                            Ok(channels) => channels,
                            Err(err) => {
                                eprintln!(
                                    "Error getting channels in guild id: {} due to: {}",
                                    guild_id, err
                                );
                                continue;
                            }
                        };
                        let (channel_to_send_to, _) =
                            match channels.iter().find(|c| c.1.name == "pacemanbot") {
                                Some(tup) => tup,
                                None => {
                                    eprintln!(
                                        "Error finding #pacemanbot channel in guild id: {}.",
                                        guild_id
                                    );
                                    continue;
                                }
                            };
                        let name;
                        let guild_roles = match guild_id.roles(&ctx).await {
                            Ok(roles) => roles,
                            Err(err) => {
                                eprintln!(
                                    "Unable to get roles in guild id: {} due to: {}",
                                    guild_id, err
                                );
                                continue;
                            }
                        };
                        let guild_roles = sort_guildroles_based_on_split(&guild_roles);
                        if channels
                            .iter()
                            .any(|c| c.1.name == "pacemanbot-runner-names")
                        {
                            let (channel_containing_player_names, _) = channels
                                .iter()
                                .find(|c| c.1.name == "pacemanbot-runner-names")
                                .unwrap();

                            let messages = match channel_containing_player_names
                                .messages(&ctx, |m| m.limit(1))
                                .await
                            {
                                Ok(messages) => messages,
                                Err(err) => {
                                    eprintln!(
                                        "Error getting messages from #pacemanbot-runner-names for guild id: {} due to: {}",
                                        guild_id, err
                                    );
                                    continue;
                                }
                            };

                            let player_names = match messages.first() {
                                Some(message) => message,
                                None => {
                                    eprintln!(
                                        "Error getting first message from #pacemanbot-runner-names for guild id: {}",
                                        guild_id
                                    );
                                    continue;
                                }
                            }
                            .content
                            .split("\n")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                            let mut player_names_with_uuid: HashMap<String, String> =
                                HashMap::new();
                            for name in player_names.iter() {
                                let raw_url = format!(
                                    "https://api.mojang.com/users/profiles/minecraft/{}",
                                    name
                                );
                                let url = reqwest::Url::parse(&*raw_url).ok().unwrap();
                                let response = match reqwest::get(url.to_owned()).await {
                                    Ok(response) => response,
                                    Err(err) => {
                                        eprintln!(
                                            "Unabled to convert '{}' to uuid due to: {}",
                                            name, err
                                        );
                                        continue;
                                    }
                                };
                                let res: HashMap<String, String> = match response
                                    .json::<HashMap<String, String>>()
                                    .await
                                {
                                    Ok(map) => map,
                                    Err(err) => {
                                        eprintln!(
                                            "Unable to parse API response for url '{}' to json due to: {}",
                                            raw_url, err
                                        );
                                        continue;
                                    }
                                };
                                let uuid = &res["id"];
                                player_names_with_uuid.insert(uuid.to_owned(), name.to_owned());
                            }
                            match player_names_with_uuid
                                .get(record.user.uuid.replace("-", "").as_str())
                            {
                                Some(user_name) => name = user_name.to_owned(),
                                None => {
                                    eprintln!(
                                "Skipping because user, with uuid '{}', is not in this guild or is not in the runners' channel.",
                                record.user.uuid
                            );
                                    continue;
                                }
                            };
                        } else {
                            let raw_url = format!(
                                "https://sessionserver.mojang.com/session/minecraft/profile/{}",
                                record.user.uuid
                            );
                            let url = reqwest::Url::parse(&*raw_url).ok().unwrap();
                            let response = match reqwest::get(url.to_owned()).await {
                                Ok(response) => response,
                                Err(err) => {
                                    eprintln!(
                                        "Unable to convert uuid '{}' to name due to: {}",
                                        record.user.uuid, err
                                    );
                                    continue;
                                }
                            };
                            let res: MojangResponse = match response.json::<MojangResponse>().await
                            {
                                Ok(map) => map,
                                Err(err) => {
                                    eprintln!(
                                        "Unable to parse API response for url '{}' to json due to: {}",
                                        raw_url, err
                                    );
                                    continue;
                                }
                            };
                            name = res.name.to_owned();
                        }
                        let event = match record.event_list.last() {
                            Some(event) => event.to_owned(),
                            None => {
                                eprintln!("No events in event list for record: {:#?}.", record);
                                continue;
                            }
                        };
                        if event_id_to_split(event.event_id.as_str()).is_some() {
                            let mut split = event_id_to_split(event.event_id.as_str()).unwrap();

                            let messages = match channel_to_send_to
                                .messages(&ctx, |m| m.limit(100))
                                .await
                            {
                                Ok(messages) => messages,
                                Err(err) => {
                                    eprintln!(
                                            "Unable to get messages from #pacemanbot for guild id: {} due to: {}",
                                            guild_id, err
                                        );
                                    continue;
                                }
                            };

                            let split_desc = match split_to_desc(split) {
                                Some(desc) => desc,
                                None => {
                                    eprintln!("Unable to get description for split: {}.", split);
                                    continue;
                                }
                            };
                            for message in messages.iter() {
                                if message.content.contains(split_desc)
                                    && message.content.contains(&format_time(event.igt as u64))
                                    && (message.content.contains(&name)
                                        || message.content.contains(
                                            &record
                                                .user
                                                .live_account
                                                .to_owned()
                                                .unwrap_or("".to_string()),
                                        ))
                                {
                                    println!(
                                        "Skipping split '{}' because it's already in the channel",
                                        split_desc
                                    );
                                    continue 'guild_loop;
                                }
                            }
                            let last_event = match record.event_list.last() {
                                Some(event) => event,
                                None => {
                                    eprintln!(
                                        "Unable to get last event in event list for record: {:#?}.",
                                        record
                                    );
                                    continue;
                                }
                            };
                            if split == "Ba" {
                                if record
                                    .event_list
                                    .iter()
                                    .filter(|evt| evt != &last_event)
                                    .any(|evt| evt.event_id == "rsg.enter_fortress")
                                {
                                    split = &"SS";
                                } else {
                                    split = &"FS";
                                }
                            } else if split == "F" {
                                if record
                                    .event_list
                                    .iter()
                                    .filter(|evt| evt != &last_event)
                                    .any(|evt| evt.event_id == "rsg.enter_bastion")
                                {
                                    split = &"SS";
                                } else {
                                    split = &"FS";
                                }
                            }
                            let roles_to_ping = guild_roles
                                .iter()
                                .filter(|role| {
                                    let (role_split_name, role_minutes, role_seconds) =
                                        extract_split_from_role_name(role.name.as_str());
                                    let (split_minutes, split_seconds) = get_time(event.igt as u64);
                                    role_split_name == *split
                                        && role_minutes >= split_minutes
                                        && (role_minutes != split_minutes
                                            || role_seconds >= split_seconds)
                                })
                                .collect::<Vec<_>>();

                            let live_link = match record.user.live_account.to_owned() {
                                Some(acc) => format!("<https://twitch.tv/{}>", acc),
                                None => {
                                    println!(
                                        "Skipping split: '{}' because user: {} is not live.",
                                        split_desc, name
                                    );
                                    continue;
                                }
                            };

                            if roles_to_ping.is_empty() {
                                println!("Skipping split: '{}' because there are no roles to ping in guild id: {}.", split, guild_id);
                                continue;
                            }

                            let content = format!(
                                "## {} - {}\n\n{}\n{}",
                                format_time(event.igt as u64),
                                split_desc,
                                live_link,
                                roles_to_ping
                                    .iter()
                                    .map(|role| role.mention().to_string())
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            );

                            match channel_to_send_to
                                .send_message(&ctx, |m| m.content(content))
                                .await
                            {
                                Ok(_) => (),
                                Err(err) => {
                                    eprintln!(
                                        "Unable to send split: '{}' with roles: {:?} due to: {}",
                                        split_desc, roles_to_ping, err
                                    );
                                }
                            }
                        }
                    }
                }
                sleep(Duration::from_secs(TIMEOUT_BETWEEN_CONSECUTIVE_QUERIES));
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(command) = interaction.as_application_command() {
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
        if let Some(message_component) = interaction.as_message_component() {
            let custom_id = match message_component.data.custom_id.as_str() {
                "remove_pmb_roles" => handle_remove_pmb_roles(&ctx, &message_component).await,
                "select_structure1_role" => {
                    handle_select_role(&ctx, &message_component, "FS").await
                }
                "select_structure2_role" => {
                    handle_select_role(&ctx, &message_component, "SS").await
                }
                "select_blind_role" => handle_select_role(&ctx, &message_component, "B").await,
                "select_eye_spy_role" => handle_select_role(&ctx, &message_component, "E").await,
                "select_end_enter_role" => handle_select_role(&ctx, &message_component, "EE").await,
                _ => {
                    Err(format!("Unknown custom id: {}.", message_component.data.custom_id).into())
                }
            };
            match custom_id {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error while handling interaction: {}", err);
                }
            };
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn handle_remove_pmb_roles(
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

    // Remove all PMB roles
    crate::utils::remove_roles_starting_with(&ctx, &guild_id, &mut member, "*").await;

    // Respond to the interaction
    message_component
        .create_interaction_response(&ctx.http, |r| {
            r.kind(ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.content("PaceManBot roles removed").ephemeral(true)
                })
        })
        .await?;
    Ok(())
}

async fn handle_select_role(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
    split: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
    crate::utils::remove_roles_starting_with(
        &ctx,
        &guild_id,
        &mut member,
        format!("*{}", split).as_str(),
    )
    .await;

    // Add the new roles
    let mut roles_to_add = Vec::new();
    for value in &message_component.data.values {
        roles_to_add.push(RoleId(value.parse::<u64>()?));
    }
    member.add_roles(&ctx, &roles_to_add).await?;

    // Respond to the interaction
    message_component
        .create_interaction_response(&ctx.http, |r| {
            r.kind(ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("Roles updated").ephemeral(true))
        })
        .await?;

    Ok(())
}
