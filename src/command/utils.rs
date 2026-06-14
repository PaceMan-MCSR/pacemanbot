use std::{collections::HashMap, error::Error};

use serenity::{
    builder::{CreateSelectMenuOption, CreateSelectMenuOptions},
    client::Context,
    model::{
        guild::{Member, Role},
        id::GuildId,
    },
};

use crate::{
    cache::{PlayerCacheEntry, Split},
    command::ROLE_COLOR,
    config::{
        extract_split_from_pb_role_name, extract_split_from_role_name, ROLE_PREFIX_115,
        ROLE_PREFIX_17, ROLE_PREFIX_AA,
    },
};

pub fn create_select_option<'a>(
    o: &'a mut CreateSelectMenuOptions,
    roles: &Vec<&Role>,
    target_split: Split,
) -> Result<&'a mut CreateSelectMenuOptions, Box<dyn Error>> {
    for role in roles {
        if role.name.contains("PB") {
            let split = match extract_split_from_pb_role_name(&role.name) {
                Some(split) => split,
                None => {
                    return Err(
                        format!("failed to extract split from pb role name: {}", role.name).into(),
                    )
                }
            };
            if split == target_split {
                o.add_option(
                    CreateSelectMenuOption::default()
                        .label(format!("PB Pace {}", target_split.alt_desc()))
                        .value(role.id.to_string())
                        .to_owned(),
                );
            }
        } else {
            let (split, minutes, seconds) = extract_split_from_role_name(&role.name)?;
            if split == target_split {
                o.add_option(
                    CreateSelectMenuOption::default()
                        .label(format!(
                            "Sub {}:{:02} {}",
                            minutes,
                            seconds,
                            target_split.alt_desc()
                        ))
                        .value(role.id.to_string())
                        .to_owned(),
                );
            }
        }
    }
    Ok(o)
}

pub async fn create_guild_role(
    ctx: &Context,
    guild: &GuildId,
    role_name: &String,
) -> Result<(), Box<dyn Error>> {
    let roles = guild.roles(&ctx.http).await?;
    if !roles
        .iter()
        .any(|(_, role)| role.name == role_name.to_string())
    {
        guild
            .create_role(ctx, |r| r.name(role_name).colour(ROLE_COLOR.into()))
            .await?;
    }
    Ok(())
}

pub fn get_new_config_contents(players: HashMap<String, PlayerCacheEntry>) -> String {
    let mut new_config = String::new();
    let mut keys: Vec<&String> = players.keys().collect();
    keys.sort_by_key(|name| name.to_lowercase());
    for key in keys {
        let players_unchecked = players.get(key);
        if players_unchecked.is_none() {
            continue;
        }

        let splits = players_unchecked.unwrap();
        let finish_config = if splits.finish.is_some() {
            format!("/{}", splits.finish.unwrap())
        } else {
            "".to_string()
        };
        let line = format!(
            "{}:{}/{}/{}/{}/{}{}",
            key,
            splits.first_structure,
            splits.second_structure,
            splits.blind,
            splits.eye_spy,
            splits.end_enter,
            finish_config
        );
        new_config = format!("{}\n{}", new_config, line);
    }
    new_config
}

pub async fn remove_runner_pings(
    ctx: &Context,
    guild_id: &GuildId,
    member: &mut Member,
    role_prefix: &str,
    split: Split,
    ign: String,
) -> Result<(), Box<dyn Error>> {
    let guild_roles = guild_id.roles(&ctx.http).await?;
    for role_id in member.roles.clone() {
        let role = guild_roles.get(&role_id).unwrap().clone();
        if role.name.starts_with(role_prefix)
            && !role.name.starts_with(ROLE_PREFIX_115)
            && !role.name.starts_with(ROLE_PREFIX_17)
            && !role.name.starts_with(ROLE_PREFIX_AA)
            && role.name.contains(ign.as_str())
            && role.name.contains(split.to_str().as_str())
        {
            member.remove_role(&ctx.http, role.id).await?;
        }
    }
    Ok(())
}
