use std::{collections::HashMap, error::Error};

use serenity::{
    client::Context,
    model::{guild::Role, id::GuildId},
};

use crate::{
    cache::{GuildCacheEntry, PlayerCacheEntry, RoleCacheEntry, Split},
    config::{
        extract_name_or_uuid_and_splits_from_config_line, extract_split_from_pb_role_name,
        extract_split_from_role_name, extract_splits_and_name_from_role_name, PACEMANBOT_CHANNEL,
        PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL, PACEMANBOT_RUNNER_NAMES_CHANNEL, ROLE_PREFIX,
        ROLE_PREFIX_115, ROLE_PREFIX_17,
    },
};

pub struct Config;

impl Config {
    pub async fn parse_config_for_guild(
        ctx: &Context,
        guild_id: GuildId,
    ) -> Result<GuildCacheEntry, Box<dyn Error>> {
        let guild = match ctx.cache.guild(guild_id) {
            Some(name) => name,
            None => {
                return Err(format!("failed to construct guild from guild id: {}", guild_id).into())
            }
        };
        let name = guild.name;

        let channels = match ctx.cache.guild_channels(guild_id) {
            Some(channels) => channels,
            None => return Err(format!("failed to get channels from guild name: {}", name,).into()),
        };
        let pace_channel = match channels.iter().find(|c| c.name == PACEMANBOT_CHANNEL) {
            Some(channel) => channel.id,
            None => {
                return Err(format!(
                    "failed to find #{} in guild name: {}",
                    PACEMANBOT_CHANNEL, name,
                )
                .into());
            }
        };
        let is_private =
            GuildCacheEntry::is_private_from_channels(channels.iter().map(|c| c.to_owned()));
        let lb_channel = match channels
            .iter()
            .find(|c| c.name == PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL)
        {
            Some(channel) => Some(channel.id),
            None => None,
        };

        let mut players: HashMap<String, PlayerCacheEntry> = HashMap::new();
        if is_private {
            let players_channel = channels
                .iter()
                .find(|c| c.name == PACEMANBOT_RUNNER_NAMES_CHANNEL)
                .unwrap();
            let messages = players_channel.messages(&ctx.http, |m| m.limit(1)).await?;
            let first_message = match messages.last() {
                Some(msg) => msg,
                None => {
                    return Err(format!(
                        "failed to get first message from #{} in guild name: {}.",
                        PACEMANBOT_RUNNER_NAMES_CHANNEL, name
                    )
                    .into())
                }
            };
            for line in first_message.content.split("\n") {
                if line == "```" || line == "" {
                    continue;
                }
                let (name_or_uuid, splits) =
                    extract_name_or_uuid_and_splits_from_config_line(line)?;
                players.insert(name_or_uuid.to_lowercase(), splits);
            }
        }

        let mut roles: Vec<RoleCacheEntry> = vec![];
        for role in guild
            .roles
            .iter()
            .map(|(_, role)| role)
            .filter(|r| {
                r.name.starts_with(ROLE_PREFIX)
                    && !r.name.starts_with(ROLE_PREFIX_115)
                    && !r.name.starts_with(ROLE_PREFIX_17)
            })
            .collect::<Vec<_>>()
        {
            let role_data = match RoleCacheEntry::new(role.to_owned()) {
                Ok(data) => data,
                Err(err) => {
                    return Err(format!(
                        "failed to make role data for role: {} in guild name: {} due to: {}",
                        role.name, name, err
                    )
                    .into())
                }
            };
            roles.push(role_data);
        }
        Ok(GuildCacheEntry {
            name,
            pace_channel,
            lb_channel,
            player_whitelist: players,
            roles,
        })
    }

    pub fn parse_role_config_for_role(role: Role) -> Result<RoleCacheEntry, Box<dyn Error>> {
        let split: Split;
        let mut hours: u8 = 0;
        let mut minutes: u8 = 0;
        let mut runner: String = String::new();
        if role.name.contains("PB") {
            split = match extract_split_from_pb_role_name(role.name.as_str()) {
                Some(tup) => tup,
                None => {
                    return Err(format!(
                        "failed to extract split from pb role name: {}.",
                        role.name
                    )
                    .into())
                }
            };
        } else if role.name.contains("+") {
            (split, hours, minutes, runner) =
                match extract_splits_and_name_from_role_name(role.name.as_str()) {
                    Ok(tup) => tup,
                    Err(err) => {
                        return Err(format!(
                            "failed to extract split from pb role name: {} due to: {}",
                            role.name, err
                        )
                        .into())
                    }
                }
        } else {
            (split, hours, minutes) = match extract_split_from_role_name(role.name.as_str()) {
                Ok(tup) => tup,
                Err(err) => {
                    return Err(format!(
                        "failed to extract split from role name: {} due to: {}",
                        role.name, err
                    )
                    .into())
                }
            };
        }
        Ok(RoleCacheEntry {
            role,
            split,
            hours,
            minutes,
            runner,
        })
    }
}
