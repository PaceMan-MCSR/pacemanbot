use serenity::{
    client::Context,
    model::{id::GuildId, prelude::application_command::ApplicationCommandInteraction},
};

use crate::{
    cache::{consts::ROLE_PREFIX, guild_data::GuildData, split::Split},
    utils::{create_guild_role::create_guild_role, remove_runner_pings::remove_runner_pings},
    Result,
};

pub async fn setup_pings(
    ctx: &Context,
    guild_id: GuildId,
    command: &ApplicationCommandInteraction,
) -> Result<()> {
    command.defer_ephemeral(&ctx).await?;
    let mut action = String::new();
    let mut ign = String::new();
    let mut split = String::new();
    let mut time = 0;
    for option in command.data.options.iter() {
        match option.name.as_str() {
            "action" => {
                action = match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err("SetupPingsError: convert 'action' value to string.".into())
                        }
                    },
                    None => {
                        return Err("SetupPingsError: get value for 'action' for command".into())
                    }
                }
            }
            "ign" => {
                ign = match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err("SetupPingsError: convert 'ign' value to string.".into())
                        }
                    },
                    None => return Err("SetupPingsError: get value for 'ign' for command".into()),
                }
            }
            "split" => {
                split = match option.value.to_owned() {
                    Some(value) => match value.as_str() {
                        Some(str) => str.to_owned(),
                        None => {
                            return Err("SetupPingsError: convert 'split' value to string.".into())
                        }
                    },
                    None => {
                        return Err("SetupPingsError: get value for 'split' for command.".into())
                    }
                }
            }
            "time" => {
                time = match option.value.to_owned() {
                    Some(value) => match value.as_u64() {
                        Some(int) => int as u8,
                        None => return Err("SetupPingsError: convert 'time' value to u64".into()),
                    },
                    None => return Err("SetupPingsError: get value for 'time' for command.".into()),
                }
            }
            _ => (),
        }
    }
    let split = match Split::from_str(split.as_str()) {
        Some(split) => split,
        None => {
            return Err(format!("SetupPingsError: construct Split from str: '{}'.", split).into())
        }
    };
    let guild_data = GuildData::new(&ctx, guild_id).await?;
    if guild_data.is_private && !guild_data.players.contains_key(&ign.to_lowercase()) {
        let response_content = format!(
            "SetupPingsError: Runner with name: '{}' not found in guild.",
            ign
        );
        return Err(response_content.into());
    }
    let mut sender = match command.member.to_owned() {
        Some(sender) => sender,
        None => return Err("SetupPingsError: get member for '/setup_pings'.".into()),
    };
    match action.as_str() {
        "add_or_update" => {
            if time == 0 {
                let content = "SetupPingsError: Parameter 'time' is undefined for 'add_or_update'.";
                command
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(content.to_string())
                    })
                    .await?;
                return Err(content.into());
            }
            remove_runner_pings(
                &ctx,
                &guild_id,
                &mut sender,
                ROLE_PREFIX,
                split.to_owned(),
                ign.to_owned(),
            )
            .await?;
            let role_name = format!("{}{}{}:0+{}", ROLE_PREFIX, split.to_str(), time, ign);
            let roles = guild_id.roles(&ctx.http).await?;
            let guild_has_role = roles.iter().any(|(_, r)| r.name == role_name);
            if !guild_has_role {
                create_guild_role(&ctx, &guild_id, &role_name).await?;
            }
            let roles = guild_id.roles(&ctx.http).await?;
            sender
                .add_role(
                    &ctx.http,
                    roles.iter().find(|(_, r)| r.name == role_name).unwrap().0,
                )
                .await?;
            command
                .edit_original_interaction_response(&ctx.http, |m| {
                    m.content(format!(
                        "Added/Updated pings for runner with ign: '{}' for split: '{}' with time: '{}m'",
                        ign,
                        split.desc(),
                        time
                    ))
                })
                .await?;
        }
        "remove" => {
            let roles = guild_id.roles(&ctx.http).await?;
            let role = match roles.iter().find(|(_, r)| {
                r.name.contains(split.to_str().as_str())
                    && r.name.starts_with(ROLE_PREFIX)
                    && r.name.contains(ign.as_str())
            }) {
                Some(name) => name,
                None => {
                    return Err("SetupPingsError: get role name for 'remove' action.".into());
                }
            };
            let role_name = role.1.name.as_str();
            remove_runner_pings(
                &ctx,
                &guild_id,
                &mut sender,
                ROLE_PREFIX,
                split.to_owned(),
                ign.to_owned(),
            )
            .await?;
            let roles = guild_id.roles(&ctx.http).await?;
            let guild_has_role = roles.iter().any(|(_, r)| r.name == role_name);
            if guild_has_role {
                guild_id
                    .delete_role(
                        &ctx,
                        roles.iter().find(|(_, r)| r.name == role_name).unwrap().0,
                    )
                    .await?;
            }
            command
                .edit_original_interaction_response(&ctx.http, |m| {
                    m.content(format!(
                        "Removed pings for runner with ign: '{}' for split: '{}'",
                        ign,
                        split.desc(),
                    ))
                })
                .await?;
        }
        _ => (),
    }
    Ok(())
}
