use std::{cmp::Ordering, collections::HashMap};

use serenity::{
    builder::CreateActionRow,
    client::Context,
    model::{
        guild::Role,
        id::RoleId,
        prelude::{
            application_command::ApplicationCommandInteraction, component::ButtonStyle::Primary,
        },
    },
};

use crate::{
    cache::split::Split,
    utils::{
        create_select_option::create_select_option,
        extract_split_from_pb_role_name::extract_split_from_pb_role_name,
        extract_split_from_role_name::extract_split_from_role_name,
        mins_secs_to_millis::mins_secs_to_millis,
    },
    Result,
};

pub async fn send_role_selection_message(
    ctx: &Context,
    roles: &HashMap<RoleId, Role>,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
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
                        "RoleSelectionMessageSendError: get split from role name: '{}': {}",
                        r1.name, err
                    );
                    return Ordering::Equal;
                }
            };
            r1_order = mins_secs_to_millis((minutes, seconds));
        }
        if r2.name.contains("PB") {
            r2_order = 0;
        } else {
            let (_, minutes, seconds) = match extract_split_from_role_name(&r2.name) {
                Ok(tup) => tup,
                Err(err) => {
                    eprintln!(
                        "RoleSelectionMessageSendError: get split from role name: '{}': {}",
                        r2.name, err
                    );
                    return Ordering::Equal;
                }
            };
            r2_order = mins_secs_to_millis((minutes, seconds));
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
                    eprintln!(
                        "RoleSelectionMessageSendError: get pb split from role name: '{}'.",
                        role.name
                    );
                    return false;
                }
            };
            return split == Split::FirstStructure;
        }
        let (split, _minutes, _seconds) = match extract_split_from_role_name(&role.name) {
            Ok(tup) => tup,
            Err(err) => {
                eprintln!(
                    "RoleSelectionMessageSendError: get split from role name: '{}': {}",
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
                        eprintln!("RoleSelectionMessageSendError: {}", err);
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
                        eprintln!("RoleSelectionMessageSendError: {}", err);
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
                        eprintln!("RoleSelectionMessageSendError: {}", err);
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
                        eprintln!("RoleSelectionMessageSendError: {}", err);
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
                        eprintln!("RoleSelectionMessageSendError: {}", err);
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

    match command
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
        .await
    {
        Ok(_) => (),
        Err(err) => {
            let content = format!(
                "RoleSelectionMessageSendError: role selection message: {}",
                err
            );
            command
                .edit_original_interaction_response(&ctx.http, |m| m.content(content.to_string()))
                .await?;
            return Err(content.into());
        }
    };
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
