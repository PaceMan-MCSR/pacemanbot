use std::collections::HashMap;

use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{GuildId, InteractionResponseType, Role, RoleId};
use serenity::prelude::Context;
use serenity::utils::Color;
use serenity::{
    builder::{CreateActionRow, CreateSelectMenuOption},
    model::prelude::component::ButtonStyle::Primary,
};

use crate::utils::{extract_split_from_role_name, sort_guildroles_based_on_split};

pub async fn send_role_selection_message(
    ctx: &Context,
    roles: &HashMap<RoleId, Role>,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let roles = sort_guildroles_based_on_split(roles);
    let mut select_fortress_role_action_row = CreateActionRow::default();
    let mut select_blind_role_action_row = CreateActionRow::default();
    let mut select_eye_spy_role_action_row = CreateActionRow::default();
    let mut select_end_enter_role_action_row = CreateActionRow::default();
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
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.content(content).components(|c| {
                    c.add_action_row(select_fortress_role_action_row)
                        .add_action_row(select_blind_role_action_row)
                        .add_action_row(select_eye_spy_role_action_row)
                        .add_action_row(select_end_enter_role_action_row)
                        .add_action_row(remove_roles_action_row)
                })
            })
        })
        .await?;
    Ok(())
}

pub async fn setup_default_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<(), Box<dyn std::error::Error>> {
    let default_roles = [
        "*SS6:0", "*SS5:3", "*SS5:0", "*SS4:3", "*B8:0", "*B7:3", "*B7:0", "*B6:3", "*B6:0",
        "*B5:3", "*E9:3", "*E9:0", "*E8:3", "*E8:0", "*EE8:3", "*EE9:0", "*EE9:3", "*EE10:0",
    ];

    let roles = guild.roles(&ctx.http).await?;
    for role in default_roles.iter() {
        if (&roles).iter().any(|(_, r)| r.name == &role[..]) {
            continue;
        }
        guild
            .create_role(&ctx.http, |r| r.name(role))
            .await?
            .edit(&ctx.http, |r| {
                r.colour(Color::from_rgb(54, 57, 63).0.into())
            })
            .await?;
    }
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| {
                    data.content("Default pace-roles have been setup!")
                        .ephemeral(true)
                })
        })
        .await?;
    Ok(())
}
