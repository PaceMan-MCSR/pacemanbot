use std::collections::HashMap;

use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId},
};

use crate::{utils::extract_name_and_splits_from_line::extract_name_and_splits_from_line, Result};

use super::{players::Players, role_data::RoleData};

#[derive(Debug)]
pub struct GuildData {
    pub name: String,
    pub pace_channel: ChannelId,
    pub lb_channel: Option<ChannelId>,
    pub players: Players,
    pub is_private: bool,
    pub roles: Vec<RoleData>,
}

impl GuildData {
    pub async fn new(ctx: &Context, guild_id: GuildId) -> Result<Self> {
        let guild = match ctx.cache.guild(guild_id) {
            Some(name) => name,
            None => {
                return Err(format!(
                    "GuildDataError: construct guild from guild id: {}",
                    guild_id
                )
                .into());
            }
        };
        let name = guild.name;

        let channels = match ctx.cache.guild_channels(guild_id) {
            Some(channels) => channels,
            None => {
                return Err(
                    format!("GuildDataError: get channels from guild name: {}", name,).into(),
                )
            }
        };
        let pace_channel = match channels.iter().find(|c| c.name == "pacemanbot") {
            Some(channel) => channel.id,
            None => {
                return Err(
                    format!("GuildDataError: find #pacemanbot in guild name: {}", name,).into(),
                );
            }
        };
        let is_private = channels.iter().any(|c| c.name == "pacemanbot-runner-names");
        let lb_channel = match channels
            .iter()
            .find(|c| c.name == "pacemanbot-runner-leaderboard")
        {
            Some(channel) => Some(channel.id),
            None => None,
        };

        let mut players: Players = HashMap::new();
        if is_private {
            let players_channel = channels
                .iter()
                .find(|c| c.name == "pacemanbot-runner-names")
                .unwrap();
            let messages = players_channel.messages(&ctx.http, |m| m.limit(1)).await?;
            let first_message = match messages.last() {
                Some(msg) => msg,
                None => {
                    return Err(format!(
                    "GuildDataError: get first message from #pacemanbot-runner-names in guild name: {}.",
                    name
                )
                    .into())
                }
            };
            for line in first_message.content.split("\n") {
                if line == "```" || line == "" {
                    continue;
                }
                let (name, splits) = extract_name_and_splits_from_line(line)?;
                players.insert(name.to_lowercase(), splits);
            }
        }

        let mut roles: Vec<RoleData> = vec![];
        for role in guild
            .roles
            .iter()
            .map(|(_, role)| role)
            .filter(|r| r.name.starts_with("*"))
            .collect::<Vec<_>>()
        {
            let role_data = match RoleData::new(role.to_owned()) {
                Ok(data) => data,
                Err(err) => {
                    return Err(format!(
                        "GuildDataError: make role data for role: {} in guild name: {} due to: {}",
                        role.name, name, err
                    )
                    .into())
                }
            };
            roles.push(role_data);
        }

        Ok(Self {
            name,
            is_private,
            pace_channel,
            lb_channel,
            players,
            roles,
        })
    }
}
