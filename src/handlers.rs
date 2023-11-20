use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::{
            Interaction, InteractionResponseType::ChannelMessageWithSource,
        },
        gateway::Ready,
        prelude::{message_component::MessageComponentInteraction, GuildId, RoleId},
    },
    prelude::Mentionable,
};
use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use crate::{
    components::send_role_selection_message,
    utils::{
        extract_split_from_role_name, format_time, get_response_from_api, get_time,
        sort_guildroles_based_on_split,
    },
};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let event_ids_we_care_about: HashMap<_, _> = vec![
            ("rsg.enter_bastion", "Bastion"),
            ("rsg.enter_fortress", "Fortress"),
            ("rsg.nether_travel", "Blind"),
            ("rsg.enter_stronghold", "EyeSpy"),
            ("rsg.enter_end", "EndEnter"),
        ]
        .into_iter()
        .collect();

        // Time to wait for between two consecutive queries in seconds.
        let timeout_between_consecutive_queries = 20;

        let ctx = Arc::new(ctx);
        tokio::spawn(async move {
            loop {
                let response = get_response_from_api().await;
                let ctx = ctx.clone();
                for record in response.iter() {
                    'guild_loop: for guild_id in guilds.iter() {
                        let channels = guild_id.channels(&ctx).await.unwrap();
                        let (channel_to_send_to, _) =
                            channels.iter().find(|c| c.1.name == "pacemanbot").unwrap();
                        let (channel_containing_player_names, _) = channels
                            .iter()
                            .find(|c| c.1.name == "pacemanbot-runner-names")
                            .unwrap();
                        let first_message = channel_containing_player_names
                            .messages(&ctx, |m| m.limit(1))
                            .await
                            .unwrap();

                        let player_names = first_message
                            .first()
                            .unwrap()
                            .content
                            .split("\n")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        let mut player_names_with_uuid: HashMap<String, String> = HashMap::new();
                        for name in player_names.iter() {
                            let url =
                                format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
                            let url = reqwest::Url::parse(&*url).ok().unwrap();
                            let response = match reqwest::get(url).await {
                                Ok(response) => response,
                                Err(err) => {
                                    panic!("Unabled to convert '{}' to uuid: {}", name, err)
                                }
                            };
                            let res: HashMap<String, String> =
                                match response.json::<HashMap<String, String>>().await {
                                    Ok(map) => map,
                                    Err(err) => panic!("Unable to parse to json: {}", err),
                                };
                            let uuid = &res["id"];
                            player_names_with_uuid.insert(uuid.to_owned(), name.to_owned());
                        }
                        let guild_roles = guild_id.roles(&ctx).await.unwrap();
                        let guild_roles = sort_guildroles_based_on_split(&guild_roles);
                        let name;
                        match player_names_with_uuid.get(record.user.uuid.replace("-", "").as_str())
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
                        let event = match record.event_list.last() {
                            Some(event) => event.to_owned(),
                            None => {
                                panic!("No events in event list for record: {:#?}.", record)
                            }
                        };
                        if event_ids_we_care_about.contains_key(event.event_id.as_str()) {
                            let split = event_ids_we_care_about
                                .get(event.event_id.as_str())
                                .unwrap();
                            let messages = channel_to_send_to
                                .messages(&ctx, |m| m.limit(100))
                                .await
                                .unwrap();
                            for message in messages.iter() {
                                if message.content.contains(split)
                                    && message.content.contains(&format_time(event.igt as u64))
                                    && message.content.contains(&name)
                                {
                                    println!(
                                        "Skipping split '{}' because it's already in the channel",
                                        split
                                    );
                                    continue 'guild_loop;
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
                                Some(acc) => format!("https://twitch.tv/{}", acc),
                                None => format!("Pacepal for {}", name),
                            };

                            let content = format!(
                                "{} `{}` {} split\n{}",
                                live_link,
                                format_time(event.igt as u64),
                                split,
                                roles_to_ping
                                    .iter()
                                    .map(|role| role.mention().to_string())
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            );
                            channel_to_send_to
                                .send_message(&ctx, |m| m.content(content))
                                .await
                                .unwrap();
                        }
                    }
                }
                sleep(Duration::from_secs(timeout_between_consecutive_queries));
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(command) = interaction.as_application_command() {
            let roles = match command.guild_id.unwrap().roles(&ctx.http).await {
                Ok(roles) => roles,
                Err(err) => {
                    eprintln!("Unable to get roles: {}", err);
                    return;
                }
            };
            match match command.data.name.as_str() {
                "send_message" => {
                    send_role_selection_message(
                        &ctx,
                        &roles,
                        command.channel_id.as_u64().to_owned(),
                    )
                    .await
                }
                _ => {
                    eprintln!("Unrecognized command: '{}'.", command.data.name);
                    return;
                }
            } {
                Ok(_) => (),
                Err(err) => eprintln!(
                    "Error while handling command '{}': {}",
                    command.data.name, err
                ),
            };
        }
        if let Some(message_component) = interaction.as_message_component() {
            let res = match message_component.data.custom_id.as_str() {
                "remove_pmb_roles" => handle_remove_pmb_roles(&ctx, &message_component).await,
                "select_bastion_role" => {
                    handle_select_role(&ctx, &message_component, "Bastion").await
                }
                "select_fortress_role" => {
                    handle_select_role(&ctx, &message_component, "Fortress").await
                }
                "select_blind_role" => handle_select_role(&ctx, &message_component, "Blind").await,
                "select_eye_spy_role" => {
                    handle_select_role(&ctx, &message_component, "EyeSpy").await
                }
                _ => Err("Unknown custom ID".into()),
            };
            if let Err(why) = res {
                eprintln!("Error handling interaction: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = match ready
            .guilds
            .into_iter()
            .map(|guild| guild.id)
            .collect::<Vec<GuildId>>()
            .pop()
        {
            Some(id) => id,
            None => {
                eprintln!("Error initiating guild id: Unable to get guild id from guilds.");
                return;
            }
        };

        match GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("send_message")
                    .description("Send role message to the current channel.")
            })
        })
        .await
        {
            Ok(_) => (),
            Err(err) => eprintln!("Error creating command: {}", err),
        };
    }
}

async fn handle_remove_pmb_roles(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let guild_id = message_component.guild_id.unwrap();
    let member = guild_id
        .member(&ctx, message_component.member.as_ref().unwrap().user.id)
        .await
        .unwrap();
    // Remove all PMB roles
    crate::utils::remove_roles_starting_with(&ctx, &guild_id, member, "PMB").await;

    // Respond to the interaction
    message_component
        .create_interaction_response(&ctx.http, |r| {
            r.kind(ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("PMB roles removed").ephemeral(true))
        })
        .await
        .unwrap();

    Ok(())
}

async fn handle_select_role(
    ctx: &Context,
    message_component: &MessageComponentInteraction,
    split: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let guild_id = message_component.guild_id.unwrap();
    let member = guild_id
        .member(&ctx, message_component.member.as_ref().unwrap().user.id)
        .await
        .unwrap();

    // Remove all PMB roles
    crate::utils::remove_roles_starting_with(
        &ctx,
        &guild_id,
        member,
        format!("PMB{}", split).as_str(),
    )
    .await;
    let mut member = guild_id
        .member(&ctx, message_component.member.as_ref().unwrap().user.id)
        .await
        .unwrap();
    // Add the new roles
    let mut roles_to_add = Vec::new();
    for value in &message_component.data.values {
        roles_to_add.push(RoleId(value.parse::<u64>().unwrap()));
    }
    member.add_roles(&ctx, &roles_to_add).await.unwrap();

    // Respond to the interaction
    message_component
        .create_interaction_response(&ctx.http, |r| {
            r.kind(ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("Roles updated").ephemeral(true))
        })
        .await
        .unwrap();

    Ok(())
}
