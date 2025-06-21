use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::{
    cache::{consts::ROLE_PREFIX, split::Split},
    utils::create_guild_role::create_guild_role,
    Result,
};

pub async fn setup_roles(
    ctx: &Context,
    guild: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
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
                        None => {
                            return Err("SetupRolesError: convert 'split_name' into '&str'.".into())
                        }
                    },
                    None => {
                        return Err(
                            "SetupRolesError: get value for option name: 'split_name'.".into()
                        )
                    }
                }
            }
            "split_start" => {
                split_start = match option.value.to_owned() {
                    Some(value) => match value.as_u64() {
                        Some(int) => int,
                        None => {
                            return Err("SetupRolesError: convert 'split_start' into 'u64'.".into())
                        }
                    },
                    None => {
                        return Err(
                            "SetupRolesError: get value for option name: 'split_start'.".into()
                        )
                    }
                }
            }
            "split_end" => {
                split_end = match option.value.to_owned() {
                    Some(value) => match value.as_u64() {
                        Some(int) => int,
                        None => {
                            return Err("SetupRolesError: convert 'split_end' into 'u64'.".into())
                        }
                    },
                    None => {
                        return Err(
                            "SetupRolesError: get value for option name: 'split_end'.".into()
                        )
                    }
                }
            }
            _ => return Err("SetupRolesError: Unrecognized option name.".into()),
        };
    }

    let role_split = match Split::from_command_param(split_name.as_str()) {
        Some(split) => split,
        None => {
            return Err(format!(
                "SetupRolesError: Unrecognized split name: '{}'.",
                split_name
            )
            .into())
        }
    };

    for hours in split_start..split_end {
        let minutes = 0;
        let role = format!(
            "{}{}{}:{}",
            ROLE_PREFIX,
            role_split.to_str(),
            hours,
            minutes,
        );
        create_guild_role(&ctx, &guild, &role).await?;

        let minutes = 30;
        let role = format!(
            "{}{}{}:{}",
            ROLE_PREFIX,
            role_split.to_str(),
            hours,
            minutes,
        );
        create_guild_role(&ctx, &guild, &role).await?;
    }
    let minutes = 0;
    let role = format!(
        "{}{}{}:{}",
        ROLE_PREFIX,
        role_split.to_str(),
        split_end,
        minutes,
    );
    create_guild_role(&ctx, &guild, &role).await?;

    let response_content = format!(
        "Pace-roles for split name: {} with lower bound: {} hours and upper bound: {} hours have been setup!",
        split_name, split_start, split_end
    );
    command
        .edit_original_interaction_response(&ctx.http, |data| data.content(response_content))
        .await?;

    Ok(())
}
