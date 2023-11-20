use std::collections::HashMap;

use serenity::model::prelude::{GuildId, Role, RoleId};
use serenity::prelude::Context;
use serenity::utils::Color;
use serenity::{
    builder::{CreateActionRow, CreateSelectMenuOption},
    model::{id::ChannelId, prelude::component::ButtonStyle::Primary},
};

use crate::utils::{extract_split_from_role_name, sort_guildroles_based_on_split};

pub async fn send_role_selection_message(
    ctx: &Context,
    roles: &HashMap<RoleId, Role>,
    channel_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel_id = ChannelId(channel_id);

    let roles = sort_guildroles_based_on_split(roles);
    let mut select_bastion_role_action_row = CreateActionRow::default();
    let mut select_fortress_role_action_row = CreateActionRow::default();
    let mut select_blind_role_action_row = CreateActionRow::default();
    let mut select_eye_spy_role_action_row = CreateActionRow::default();
    select_bastion_role_action_row.create_select_menu(|m| {
        m.custom_id("select_bastion_role")
            .placeholder("Choose a Bastion Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("PMB") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "Bastion" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} Bastion", minutes, seconds))
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
        m.custom_id("select_fortress_role")
            .placeholder("Choose a Fortress Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("PMB") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "Fortress" {
                            o.add_option(
                                CreateSelectMenuOption::default()
                                    .label(format!("Sub {}:{:02} Fort", minutes, seconds))
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
                    if role.name.starts_with("PMB") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "Blind" {
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
            .placeholder("Choose a Eye Spy Role...")
            .options(|o| {
                for role in &roles {
                    if role.name.starts_with("PMB") {
                        let (split, minutes, seconds) = extract_split_from_role_name(&role.name);
                        if split == "EyeSpy" {
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
    let mut remove_roles_action_row = CreateActionRow::default();

    remove_roles_action_row.create_button(|c| {
        c.style(Primary)
            .label("Remove ALL PMB Roles")
            .custom_id("remove_pmb_roles")
    });

    channel_id
            .send_message(&ctx.http, |m| {
                m.content("
Choose roles corresponding to a speedrunning split and time, such as \"PMBBastionSub2:30.\".
You'll receive pings for the selected split and any faster paces within that category. Select roles based on the splits and paces you wish to follow.
")
                    .components(|c| {
                        c.add_action_row(select_bastion_role_action_row)
                        .add_action_row(select_fortress_role_action_row)
                        .add_action_row(select_blind_role_action_row)
                        .add_action_row(select_eye_spy_role_action_row)
                            .add_action_row(remove_roles_action_row)
                    })
            })
            .await?;

    Ok(())
}

pub async fn setup_default_roles(
    ctx: &Context,
    guild: GuildId,
) -> Result<(), Box<dyn std::error::Error>> {
    let default_roles = [
        "PMBBastionSub3:00",
        "PMBBastionSub2:30",
        "PMBBastionSub2:00",
        "PMBFortressSub6:00",
        "PMBFortressSub5:30",
        "PMBFortressSub5:00",
        "PMBFortressSub4:30",
        "PMBBlindSub8:00",
        "PMBBlindSub7:30",
        "PMBBlindSub7:00",
        "PMBBlindSub6:30",
        "PMBBlindSub6:00",
        "PMBBlindSub5:30",
        "PMBEyeSpySub9:30",
        "PMBEyeSpySub9:00",
        "PMBEyeSpySub8:30",
        "PMBEyeSpySub8:00",
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
                r.colour(Color::from_rgb(255, 255, 0).0.into())
            })
            .await?;
    }
    Ok(())
}

pub async fn remove_all_pmb_roles_from_guild(
    ctx: &Context,
    guild: GuildId,
) -> Result<(), Box<dyn std::error::Error>> {
    let roles = guild.roles(&ctx.http).await.unwrap();
    for (role_id, role) in roles.iter() {
        if role.name.starts_with("PMB") {
            guild
                .delete_role(&ctx.http, *role_id)
                .await
                .expect("Failed to delete role");
        }
    }
    Ok(())
}
