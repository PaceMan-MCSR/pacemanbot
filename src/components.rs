use std::cmp::Ordering;
use std::collections::HashMap;

use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::{GuildId, Role, RoleId};
use serenity::prelude::Context;
use serenity::utils::Color;
use serenity::{builder::CreateActionRow, model::prelude::component::ButtonStyle::Primary};

use crate::guild_types::Split;
use crate::utils::{
    create_select_option, extract_split_from_pb_role_name, extract_split_from_role_name,
};

pub async fn send_role_selection_message(
    ctx: &Context,
    roles: &HashMap<RoleId, Role>,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    command.defer(&ctx).await?;

    let mut roles = roles
        .iter()
        .map(|(_, role)| role)
        .filter(|r| r.name.starts_with("*"))
        .collect::<Vec<_>>();
    roles.sort_by(|r1, r2| {
        let r1_order;
        let r2_order;
        if r1.name.contains("PB") {
            r1_order = 0;
        } else {
            let (_, minutes, seconds) = match extract_split_from_role_name(&r1.name) {
                Ok(tup) => tup,
                Err(err) => {
                    eprintln!(
                        "Unable to get split from role name: '{}' due to: {}",
                        r1.name, err
                    );
                    return Ordering::Equal;
                }
            };
            r1_order = minutes as u64 * 60000 + seconds as u64 * 1000;
        }
        if r2.name.contains("PB") {
            r2_order = 0;
        } else {
            let (_, minutes, seconds) = match extract_split_from_role_name(&r2.name) {
                Ok(tup) => tup,
                Err(err) => {
                    eprintln!(
                        "Unable to get split from role name: '{}' due to: {}",
                        r2.name, err
                    );
                    return Ordering::Equal;
                }
            };
            r2_order = minutes as u64 * 60000 + seconds as u64 * 1000;
        }
        r1_order.cmp(&r2_order)
    });
    let mut select_bastion_role_action_row = CreateActionRow::default();
    let mut select_fortress_role_action_row = CreateActionRow::default();
    let mut select_blind_role_action_row = CreateActionRow::default();
    let mut select_eye_spy_role_action_row = CreateActionRow::default();
    let mut select_end_enter_role_action_row = CreateActionRow::default();

    let send_bastion_picker = roles.iter().any(|role| {
        if role.name.contains("PB") {
            let split = match extract_split_from_pb_role_name(&role.name) {
                Some(split) => split,
                None => {
                    eprintln!("Unable to get pb split from role name: '{}'.", role.name);
                    return false;
                }
            };
            return split == Split::FirstStructure;
        }
        let (split, _minutes, _seconds) = match extract_split_from_role_name(&role.name) {
            Ok(tup) => tup,
            Err(err) => {
                eprintln!(
                    "Unable to get split from role name: '{}' due to: {}",
                    role.name, err
                );
                return false;
            }
        };
        split == Split::FirstStructure
    });

    select_bastion_role_action_row.create_select_menu(|m| {
        m.custom_id("select_structure1_role")
            .placeholder("Choose a First Structure Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::FirstStructure) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Unable to create select option due to: {}", err);
                    }
                }
                o
            })
    });
    select_fortress_role_action_row.create_select_menu(|m| {
        m.custom_id("select_structure2_role")
            .placeholder("Choose a Second Structure Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::SecondStructure) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Unable to create select option due to: {}", err);
                    }
                }
                o
            })
    });
    select_blind_role_action_row.create_select_menu(|m| {
        m.custom_id("select_blind_role")
            .placeholder("Choose a Blind Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::Blind) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Unable to create select option due to: {}", err);
                    }
                }
                o
            })
    });
    select_eye_spy_role_action_row.create_select_menu(|m| {
        m.custom_id("select_eye_spy_role")
            .placeholder("Choose an Eye Spy Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::EyeSpy) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Unable to create select option due to: {}", err);
                    }
                }
                o
            })
    });
    select_end_enter_role_action_row.create_select_menu(|m| {
        m.custom_id("select_end_enter_role")
            .placeholder("Choose an End Enter Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::EndEnter) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("Unable to create select option due to: {}", err);
                    }
                }
                o
            })
    });
    let mut remove_roles_action_row = CreateActionRow::default();

    remove_roles_action_row.create_button(|c| {
        c.style(Primary)
            .label("Remove ALL PMB Roles")
            .custom_id("remove_pmb_roles")
    });

    let content = "Select roles based on the splits and paces you wish to follow.";

    command
        .edit_original_interaction_response(&ctx.http, |data| {
            data.content(content).components(|c| {
                if send_bastion_picker {
                    c.add_action_row(select_bastion_role_action_row)
                        .add_action_row(select_fortress_role_action_row)
                        .add_action_row(select_blind_role_action_row)
                        .add_action_row(select_eye_spy_role_action_row)
                        .add_action_row(select_end_enter_role_action_row)
                } else {
                    c.add_action_row(select_fortress_role_action_row)
                        .add_action_row(select_blind_role_action_row)
                        .add_action_row(select_eye_spy_role_action_row)
                        .add_action_row(select_end_enter_role_action_row)
                        .add_action_row(remove_roles_action_row.to_owned())
                }
            })
        })
        .await?;
    if send_bastion_picker {
        command
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content("")
                    .components(|c| c.add_action_row(remove_roles_action_row))
            })
            .await?;
    }
    Ok(())
}

pub async fn setup_default_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    command.defer_ephemeral(&ctx).await?;

    let default_roles = [
        "*FS2:0", "*FS2:3", "*FS3:0", "*SS6:0", "*SS5:3", "*SS5:0", "*SS4:3", "*B8:0", "*B7:3",
        "*B7:0", "*B6:3", "*B6:0", "*B5:3", "*E9:3", "*E9:0", "*E8:3", "*E8:0", "*EE8:3", "*EE9:0",
        "*EE9:3", "*EE10:0",
    ];

    let roles = guild.roles(&ctx.http).await?;
    for role in default_roles.iter() {
        if (&roles).iter().any(|(_, r)| r.name == &role[..]) {
            continue;
        }
        guild
            .create_role(&ctx.http, |r| {
                r.name(role).colour(Color::from_rgb(54, 57, 63).0.into())
            })
            .await?;
    }
    command
        .edit_original_interaction_response(&ctx.http, |data| {
            data.content("Default pace-roles have been setup!")
        })
        .await?;
    Ok(())
}

pub async fn setup_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    command.defer_ephemeral(&ctx).await?;

    let mut split_name = "".to_string();
    let mut split_start = 0;
    let mut split_end = 0;
    for option in command.data.options.iter() {
        match option.name.as_str() {
            "split_name" => {
                split_name = match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => return Err("Unable to convert 'split_name' into '&str'.".into()),
                    },
                    None => return Err("Unable to get value for option name: 'split_name'.".into()),
                }
            }
            "split_start" => {
                split_start = match option.value.to_owned() {
                    Some(value) => match value.as_u64() {
                        Some(int) => int,
                        None => return Err("Unable to convert 'split_start' into 'u64'.".into()),
                    },
                    None => {
                        return Err("Unable to get value for option name: 'split_start'.".into())
                    }
                }
            }
            "split_end" => {
                split_end = match option.value.to_owned() {
                    Some(value) => match value.as_u64() {
                        Some(int) => int,
                        None => return Err("Unable to convert 'split_end' into 'u64'.".into()),
                    },
                    None => return Err("Unable to get value for option name: 'split_end'.".into()),
                }
            }
            _ => return Err("Unrecognized option name.".into()),
        };
    }

    let role_split = match Split::from_command_param(split_name.as_str()) {
        Some(split) => split,
        None => return Err(format!("Unrecognized split name: '{}'.", split_name).into()),
    };

    let roles = guild.roles(&ctx).await?;
    for minutes in split_start..split_end {
        let seconds = 0;
        let role = format!("*{}{}:{}", role_split.to_str(), minutes, seconds);
        if !roles.iter().any(|(_, r)| r.name == role) {
            guild
                .create_role(ctx, |r| {
                    r.name(role).colour(Color::from_rgb(54, 57, 63).0.into())
                })
                .await?;
        }
        let seconds = 3;
        let role = format!("*{}{}:{}", role_split.to_str(), minutes, seconds);
        if !roles.iter().any(|(_, r)| r.name == role) {
            guild
                .create_role(ctx, |r| {
                    r.name(role).colour(Color::from_rgb(54, 57, 63).0.into())
                })
                .await?;
        }
    }
    let seconds = 0;
    let role = format!("*{}{}:{}", role_split.to_str(), split_end, seconds);
    if !roles.iter().any(|(_, r)| r.name == role) {
        guild
            .create_role(ctx, |r| {
                r.name(role).colour(Color::from_rgb(54, 57, 63).0.into())
            })
            .await?;
    }

    let response_content = format!(
        "Pace-roles for split name: {} with lower bound: {} minutes and upper bound: {} minutes have been setup!",
        split_name, split_start, split_end
    );
    command
        .edit_original_interaction_response(&ctx.http, |data| data.content(response_content))
        .await?;

    Ok(())
}

pub async fn setup_pb_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    command.defer_ephemeral(&ctx).await?;
    let splits: Vec<Split> = vec![
        Split::FirstStructure,
        Split::SecondStructure,
        Split::Blind,
        Split::EyeSpy,
        Split::EndEnter,
    ];
    let roles = guild.roles(&ctx).await?;
    for split in splits {
        let role_name = format!("*{}PB", split.to_str());
        if !roles.iter().any(|(_, role)| role.name == role_name) {
            guild
                .create_role(ctx, |r| {
                    r.name(role_name)
                        .colour(Color::from_rgb(54, 57, 63).0.into())
                })
                .await?;
        }
    }
    command
        .edit_original_interaction_response(&ctx.http, |data| {
            data.content("PB Pace pace-roles have been setup!")
        })
        .await?;
    Ok(())
}

pub async fn setup_default_commands(ctx: &Context, guild_id: GuildId) {
    match GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        commands.create_application_command(|command| {
            command
                .name("send_message")
                .description("Send role message to the current channel.")
        });
        commands.create_application_command(|command| {
            command
                .name("setup_default_roles")
                .description("Setup default pace-roles for sub 10.")
        });
        commands.create_application_command(|command| {
            command.name("setup_pb_roles").description(
                "Setup split PB pace-roles(as specified per runner in #pacemanbot-runner-names).",
            )
        });
        commands.create_application_command(|command| {
            command
            .name("setup_roles")
            .description(
                "Setup pace-roles based on split, start time and end time in increments of 30s.",
            )
            .create_option(|option| {
                option
                    .name("split_name")
                    .description("The name of the split.")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice("First Structure", "first_structure")
                    .add_string_choice("Second Structure", "second_structure")
                    .add_string_choice("Blind", "blind")
                    .add_string_choice("Eye Spy", "eye_spy")
                    .add_string_choice("End Enter", "end_enter")
            })
            .create_option(|option| {
                option
                    .name("split_start")
                    .description("The lower bound for the split in minutes.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("split_end")
                    .description("The upper bound for the split in minutes.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
        })
    })
    .await
    {
        Ok(_) => (),
        Err(err) => eprintln!("Error creating command: {}", err),
    };
}
