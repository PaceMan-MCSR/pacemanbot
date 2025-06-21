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
    cache::{
        consts::{ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17},
        split::Split,
    },
    eprintln,
    utils::{
        create_select_option::create_select_option,
        extract_split_from_role_name::extract_split_from_role_name,
        mins_secs_to_millis::hrs_mins_secs_to_millis,
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
        .filter(|r| {
            r.name.starts_with(ROLE_PREFIX)
                && !r.name.starts_with(ROLE_PREFIX_115)
                && !r.name.starts_with(ROLE_PREFIX_17)
        })
        .collect::<Vec<_>>();
    roles.sort_by(|r1, r2| {
        let r1_order;
        let r2_order;
        if r1.name.contains("PB") {
            r1_order = 0;
        } else {
            let (_, hours, minutes) = match extract_split_from_role_name(&r1.name) {
                Ok(tup) => tup,
                Err(err) => {
                    eprintln!(
                        "RoleSelectionMessageSendError: get split from role name: '{}': {}",
                        r1.name, err
                    );
                    return Ordering::Equal;
                }
            };
            r1_order = hrs_mins_secs_to_millis((hours, minutes));
        }
        if r2.name.contains("PB") {
            r2_order = 0;
        } else {
            let (_, hours, minutes) = match extract_split_from_role_name(&r2.name) {
                Ok(tup) => tup,
                Err(err) => {
                    eprintln!(
                        "RoleSelectionMessageSendError: get split from role name: '{}': {}",
                        r2.name, err
                    );
                    return Ordering::Equal;
                }
            };
            r2_order = hrs_mins_secs_to_millis((hours, minutes));
        }
        r1_order.cmp(&r2_order)
    });
    let mut select_adventuring_time_role_action_row = CreateActionRow::default();
    let mut select_beaconator_role_action_row = CreateActionRow::default();
    let mut select_hdwgh_role_action_row = CreateActionRow::default();

    select_adventuring_time_role_action_row.create_select_menu(|m| {
        m.custom_id("select_adventuring_time_role")
            .placeholder("Choose an Adventuring Time Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::AdventuringTime) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("RoleSelectionMessageSendError: {}", err);
                    }
                }
                o
            })
    });
    select_beaconator_role_action_row.create_select_menu(|m| {
        m.custom_id("select_beaconator_role")
            .placeholder("Choose a Beaconator Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::Beaconator) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("RoleSelectionMessageSendError: {}", err);
                    }
                }
                o
            })
    });
    select_hdwgh_role_action_row.create_select_menu(|m| {
        m.custom_id("select_hdwgh_role")
            .placeholder("Choose a HDWGH Role...")
            .options(|o| {
                match create_select_option(o, &roles, Split::HDWGH) {
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
                c.add_action_row(select_adventuring_time_role_action_row)
                    .add_action_row(select_beaconator_role_action_row)
                    .add_action_row(select_hdwgh_role_action_row)
                    .add_action_row(remove_roles_action_row.to_owned())
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

    Ok(())
}
