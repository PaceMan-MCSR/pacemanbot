use std::{collections::HashMap, error::Error, sync::Arc};

use serenity::{
    client::Context,
    model::{
        id::{ChannelId, GuildId},
        prelude::GuildChannel,
    },
};

use crate::{
    cache::{PlayerCacheEntry, RoleCacheEntry, END_EMOJI, PORTAL_EMOJI},
    config::PACEMANBOT_RUNNER_NAMES_CHANNEL,
    ws::EventId,
};

pub struct GuildCacheEntry {
    pub name: String,
    pub pace_channel: ChannelId,
    pub lb_channel: Option<ChannelId>,
    pub player_whitelist: HashMap<String, PlayerCacheEntry>,
    pub roles: Vec<RoleCacheEntry>,
}

impl GuildCacheEntry {
    pub fn is_private(
        name: String,
        ctx: Arc<Context>,
        guild_id: &GuildId,
    ) -> Result<bool, Box<dyn Error>> {
        let channels = match ctx.cache.guild_channels(guild_id) {
            Some(channels) => channels,
            None => return Err(format!("failed to get channels from guild name: {}", name).into()),
        };
        Ok(GuildCacheEntry::is_private_from_channels(
            channels.iter().map(|c| c.to_owned()),
        ))
    }

    pub fn is_private_from_channels(mut channels: impl Iterator<Item = GuildChannel>) -> bool {
        channels.any(|c| c.name == PACEMANBOT_RUNNER_NAMES_CHANNEL)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Split {
    TowerStart,
    EndEnter,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "T" => Some(Split::TowerStart),
            "EE" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_event_id(event_id: &EventId) -> Option<Split> {
        match event_id {
            EventId::RsgTowerStart => Some(Split::TowerStart),
            EventId::RsgEnterEnd => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "tower_start" => Some(Split::TowerStart),
            "end_enter" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn desc(&self) -> String {
        match self {
            Split::TowerStart => "Tower Start",
            Split::EndEnter => "Enter End",
        }
        .to_string()
    }

    pub fn get_emoji(&self) -> Option<String> {
        Some(
            match self {
                Split::TowerStart => PORTAL_EMOJI,
                Split::EndEnter => END_EMOJI,
            }
            .to_string(),
        )
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::TowerStart => "T",
            Split::EndEnter => "EE",
        }
        .to_string()
    }
}
