use serenity::{
    async_trait, builder::CreateApplicationCommand, model::prelude::command::CommandOptionType,
};
use std::error::Error;

use crate::{
    cache::Split,
    command::{Command, CommandContext},
    config::{
        extract_split_from_role_name, ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17, ROLE_PREFIX_AA,
    },
};

pub struct RemoveRoles;

#[async_trait]
impl Command for RemoveRoles {
    fn name(&self) -> &str {
        "remove_roles"
    }

    fn description(&self) -> &str {
        "Remove all pace-roles based on split."
    }

    fn create_options<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command.create_option(|option| {
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
    }

    async fn execute(&self, context: CommandContext<'_>) -> Result<(), Box<dyn Error>> {
        let ctx = context.ctx;
        let command = context.interaction;
        let split_name = match command.data.options.get(0) {
            Some(option) => match option.name.as_str() {
                "split_name" => match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => return Err("failed to convert 'split_name' into '&str'.".into()),
                    },
                    None => return Err("failed to get value for option name: 'split_name'.".into()),
                },
                _ => {
                    return Err("unrecognized option name.".into());
                }
            },
            None => {
                return Err("no options provided.".into());
            }
        };
        let split = match Split::from_command_param(&split_name) {
            Some(split) => split,
            None => return Err(format!("unrecognized split name: '{}'.", split_name).into()),
        };
        let mut errors = Vec::new();
        let roles = context.guild_id.roles(&ctx).await?;
        let removeable_roles = roles
            .iter()
            .filter(|(_, r)| {
                r.name.starts_with(ROLE_PREFIX)
                    && !r.name.starts_with(ROLE_PREFIX_115)
                    && !r.name.starts_with(ROLE_PREFIX_17)
                    && !r.name.starts_with(ROLE_PREFIX_AA)
                    && !r.name.contains("PB") // Skip PB roles
                    && !r.name.contains("+") // Skip player pings
            })
            .filter(|(_, r)| {
                let (role_split, _, _) = match extract_split_from_role_name(r.name.as_str()) {
                    Ok(tup) => tup,
                    Err(err) => {
                        errors.push(format!(
                            "failed to extract split from role name: '{}': {}",
                            r.name, err
                        ));
                        return false;
                    }
                };
                role_split == split
            })
            .collect::<Vec<_>>();
        for (role_id, _) in removeable_roles {
            match context.guild_id.delete_role(&ctx, role_id).await {
                Ok(_) => (),
                Err(err) => {
                    let mut content = format!("failed to delete role: {}", err);
                    if !errors.is_empty() {
                        content = format!("{}\n\t{}", content, errors.join("\n\t"));
                    }
                    return Err(content.into());
                }
            };
        }
        match command
            .edit_original_interaction_response(&ctx.http, |m| {
                m.content(format!("Removed all roles for split name: {}", split_name))
            })
            .await
        {
            Ok(_) => (),
            Err(err) => {
                let mut content = format!("failed to edit message: {}", err);
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

pub const REMOVE_ROLES: RemoveRoles = RemoveRoles {};
