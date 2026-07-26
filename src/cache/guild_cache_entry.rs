use std::{collections::HashMap, error::Error, sync::Arc};

use serenity::{
    client::Context,
    model::{
        id::{ChannelId, GuildId},
        prelude::GuildChannel,
    },
};

use crate::{
    cache::{
        PlayerCacheEntry, RoleCacheEntry, ADVENTURING_TIME_EMOJI, BEACONATOR_EMOJI, HDWGH_EMOJI,
    },
    config::PACEMANBOT_RUNNER_NAMES_CHANNEL,
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
    AdventuringTime,
    Beaconator,
    HDWGH,
    Finish,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "AT" => Some(Split::AdventuringTime),
            "B" => Some(Split::Beaconator),
            "H" => Some(Split::HDWGH),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "adventuring_time" => Some(Split::AdventuringTime),
            "beaconator" => Some(Split::Beaconator),
            "hdwgh" => Some(Split::HDWGH),
            _ => None,
        }
    }

    pub fn desc(&self) -> String {
        match self {
            Split::AdventuringTime => "Adventuring Time",
            Split::Beaconator => "Beaconator",
            Split::HDWGH => "How Did We Get Here?",
            _ => "",
        }
        .to_string()
    }

    pub fn get_emoji(&self) -> Option<String> {
        Some(
            match self {
                Split::AdventuringTime => ADVENTURING_TIME_EMOJI,
                Split::Beaconator => BEACONATOR_EMOJI,
                Split::HDWGH => HDWGH_EMOJI,
                _ => "",
            }
            .to_string(),
        )
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::AdventuringTime => "AT",
            Split::Beaconator => "B",
            Split::HDWGH => "H",
            _ => "",
        }
        .to_string()
    }
}
