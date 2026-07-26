use std::{cmp::Ordering, error::Error};

use serenity::{
    async_trait,
    builder::{CreateActionRow, CreateApplicationCommand},
    model::prelude::component::ButtonStyle,
};

use crate::{
    cache::Split,
    command::{create_select_option, Command, CommandContext},
    config::{extract_split_from_role_name, ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17},
    dispatcher::hrs_mins_secs_to_millis,
};

pub struct SendMessage;

#[async_trait]
impl Command for SendMessage {
    fn name(&self) -> &str {
        "send_message_aa"
    }

    fn description(&self) -> &str {
        "Send role message to the current channel."
    }

    fn create_options<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let ctx = context.ctx;
        let command = context.interaction;
        let roles = context.guild_id.roles(&ctx.http).await?;
        let mut errors = Vec::new();

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
                        errors.push(format!(
                            "failed to get split from role name: '{}': {}",
                            r1.name, err
                        ));
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
                        errors.push(format!(
                            "failed to get split from role name: '{}': {}",
                            r2.name, err
                        ));
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
                .placeholder("Choose a Adventuring Time Role...")
                .options(|o| {
                    match create_select_option(o, &roles, Split::AdventuringTime) {
                        Ok(_) => (),
                        Err(err) => {
                            errors.push(format!("{}", err));
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
                            errors.push(format!("{}", err));
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
                            errors.push(format!("{}", err));
                        }
                    }
                    o
                })
        });
        let mut remove_roles_action_row = CreateActionRow::default();

        remove_roles_action_row.create_button(|c| {
            c.style(ButtonStyle::Primary)
                .label("Remove ALL PMB Roles")
                .custom_id("remove_pmb_roles")
        });

        let content = "Select roles based on the splits and paces you wish to follow.";

        match command
            .channel_id
            .send_message(&ctx.http, |data| {
                data.content(content).components(|c| {
                    c.add_action_row(select_adventuring_time_role_action_row)
                        .add_action_row(select_beaconator_role_action_row)
                        .add_action_row(select_hdwgh_role_action_row)
                })
            })
            .await
        {
            Ok(_) => (),
            Err(err) => {
                let mut content = format!("failed to send role selection message: {}", err);
                if !errors.is_empty() {
                    content = format!("{}\n\t{}", content, errors.join("\n\t"));
                }
                return Err(content.into());
            }
        };
        match command
            .channel_id
            .send_message(&ctx.http, |m| {
                m.components(|c| c.add_action_row(remove_roles_action_row))
            })
            .await
        {
            Ok(_) => (),
            Err(err) => {
                let mut content = format!("failed to send remove roles message: {}", err);
                if !errors.is_empty() {
                    content = format!("{}\n\t{}", content, errors.join("\n\t"));
                }
                return Err(content.into());
            }
        };
        match command
            .edit_original_interaction_response(&ctx.http, |m| m.content("Sent message!"))
            .await
        {
            Ok(_) => (),
            Err(err) => {
                let mut content = format!("failed to edit original interaction response: {}", err);
                if !errors.is_empty() {
                    content = format!("{}\n\t{}", content, errors.join("\n\t"));
                }
                return Err(content.into());
            }
        };
        if !errors.is_empty() {
            return Err(format!("succeded but with some errors: {}", errors.join("\n\t")).into());
        }
        Ok(())
    }
}

pub const SEND_MESSAGE: SendMessage = SendMessage {};
