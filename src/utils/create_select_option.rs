use serenity::{
    builder::{CreateSelectMenuOption, CreateSelectMenuOptions},
    model::guild::Role,
};

use crate::{cache::split::Split, Result};

use super::{
    extract_split_from_pb_role_name::extract_split_from_pb_role_name,
    extract_split_from_role_name::extract_split_from_role_name,
};

pub fn create_select_option<'a>(
    o: &'a mut CreateSelectMenuOptions,
    roles: &Vec<&Role>,
    target_split: Split,
) -> Result<&'a mut CreateSelectMenuOptions> {
    for role in roles {
        if role.name.contains("PB") {
            let split = match extract_split_from_pb_role_name(&role.name) {
                Some(split) => split,
                None => {
                    return Err(format!(
                        "CreateSelectOptionError: extract split from pb role name: {}",
                        role.name
                    )
                    .into())
                }
            };
            if split == target_split {
                o.add_option(
                    CreateSelectMenuOption::default()
                        .label(format!("PB Pace {}", target_split.desc()))
                        .value(role.id.to_string())
                        .to_owned(),
                );
            }
        } else {
            let (split, hours, minutes) = extract_split_from_role_name(&role.name)?;
            if split == target_split {
                let label = format!("Sub {}:{:02} {}", hours, minutes, target_split.desc());
                o.add_option(
                    CreateSelectMenuOption::default()
                        .label(label)
                        .value(role.id.to_string())
                        .to_owned(),
                );
            }
        }
    }
    Ok(o)
}
