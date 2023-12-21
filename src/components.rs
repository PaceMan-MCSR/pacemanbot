use std::collections::HashMap;

use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::{GuildId, Role, RoleId};
use serenity::prelude::Context;
use serenity::utils::Color;
use serenity::{
    builder::{CreateActionRow, CreateSelectMenuOption},
    model::prelude::component::ButtonStyle::Primary,
};

use crate::utils::extract_split_from_role_name;

pub async fn send_role_selection_message(
    ctx: &Context,
    roles: &HashMap<RoleId, Role>,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    command.defer(&ctx).await?;

    let roles = roles.iter().map(|(_, role)| role).collect::<Vec<_>>();
    let mut select_bastion_role_action_row = CreateActionRow::default();
    let mut select_fortress_role_action_row = CreateActionRow::default();
    let mut select_blind_role_action_row = CreateActionRow::default();
    let mut select_eye_spy_role_action_row = CreateActionRow::default();
    let mut select_end_enter_role_action_row = CreateActionRow::default();
    let send_bastion_picker = roles.iter().any(|role| {
        let (split, _minutes, _seconds) = extract_split_from_role_name(&role.name);
        split == "FS"
    });
    select_bastion_role_action_row.create_select_menu(|m| {
        m.custom_id("select_structure1_role")
            .placeholder("Choose a First Structure Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("*") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "FS" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} Structure 1", minutes, seconds))
                                    .value(role.id.to_string())
                                    .to_owned(),
                            );
                        }
                    }
                }
                o
            })
    });
    select_fortress_role_action_row.create_select_menu(|m| {
        m.custom_id("select_structure2_role")
            .placeholder("Choose a Second Structure Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("*") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "SS" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} Structure 2", minutes, seconds))
                                    .value(role.id.to_string())
                                    .to_owned(),
                            );
                        }
                    }
                }
                o
            })
    });
    select_blind_role_action_row.create_select_menu(|m| {
        m.custom_id("select_blind_role")
            .placeholder("Choose a Blind Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("*") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "B" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} Blind", minutes, seconds))
                                    .value(role.id.to_string())
                                    .to_owned(),
                            );
                        }
                    }
                }
                o
            })
    });
    select_eye_spy_role_action_row.create_select_menu(|m| {
        m.custom_id("select_eye_spy_role")
            .placeholder("Choose an Eye Spy Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("*") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "E" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} Eye Spy", minutes, seconds))
                                    .value(role.id.to_string())
                                    .to_owned(),
                            );
                        }
                    }
                }
                o
            })
    });
    select_end_enter_role_action_row.create_select_menu(|m| {
        m.custom_id("select_end_enter_role")
            .placeholder("Choose an End Enter Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("*") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "EE" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} End Enter", minutes, seconds))
                                    .value(role.id.to_string())
                                    .to_owned(),
                            );
                        }
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

    let role_prefix;
    match split_name.as_str() {
        "first_structure" => role_prefix = "FS",
        "second_structure" => role_prefix = "SS",
        "blind" => role_prefix = "B",
        "eye_spy" => role_prefix = "E",
        "end_enter" => role_prefix = "EE",
        _ => return Err(format!("Unrecognized split name: '{}'.", split_name).into()),
    }

    let roles = guild.roles(&ctx).await?;
    for minutes in split_start..split_end {
        let seconds = 0;
        let role = format!("*{}{}:{}", role_prefix, minutes, seconds);
        if !roles.iter().any(|(_, r)| r.name == role) {
            guild
                .create_role(ctx, |r| {
                    r.name(role).colour(Color::from_rgb(54, 57, 63).0.into())
                })
                .await?;
        }
        let seconds = 3;
        let role = format!("*{}{}:{}", role_prefix, minutes, seconds);
        if !roles.iter().any(|(_, r)| r.name == role) {
            guild
                .create_role(ctx, |r| {
                    r.name(role).colour(Color::from_rgb(54, 57, 63).0.into())
                })
                .await?;
        }
    }
    let seconds = 0;
    let role = format!("*{}{}:{}", role_prefix, split_end, seconds);
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
