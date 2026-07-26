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
        PlayerCacheEntry, RoleCacheEntry, END_EMOJI, FORT_EMOJI, NETHER_EMOJI, PORTAL_EMOJI,
        SH_EMOJI,
    },
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
    EnterNether,
    EnterFortress,
    Blind,
    EyeSpy,
    EndEnter,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "NE" => Some(Split::EnterNether),
            "F" => Some(Split::EnterFortress),
            "B" => Some(Split::Blind),
            "E" => Some(Split::EyeSpy),
            "EE" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_event_id(event_id: &EventId) -> Option<Split> {
        match event_id {
            EventId::RsgEnterNether => Some(Split::EnterNether),
            EventId::RsgEnterFortress => Some(Split::EnterFortress),
            EventId::RsgFirstPortal => Some(Split::Blind),
            EventId::RsgEnterStronghold => Some(Split::EyeSpy),
            EventId::RsgEnterEnd => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "enter_nether" => Some(Split::EnterNether),
            "enter_fortress" => Some(Split::EnterFortress),
            "blind" => Some(Split::Blind),
            "eye_spy" => Some(Split::EyeSpy),
            "end_enter" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn desc(&self) -> String {
        match self {
            Split::EnterNether => "Enter Nether",
            Split::EnterFortress => "Enter Fortress",
            Split::Blind => "First Portal",
            Split::EyeSpy => "Enter Stronghold",
            Split::EndEnter => "Enter End",
        }
        .to_string()
    }

    pub fn get_emoji(&self) -> Option<String> {
        Some(
            match self {
                Split::EnterNether => NETHER_EMOJI,
                Split::EnterFortress => FORT_EMOJI,
                Split::Blind => PORTAL_EMOJI,
                Split::EyeSpy => SH_EMOJI,
                Split::EndEnter => END_EMOJI,
            }
            .to_string(),
        )
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::EnterNether => "NE",
            Split::EnterFortress => "F",
            Split::Blind => "B",
            Split::EyeSpy => "E",
            Split::EndEnter => "EE",
        }
        .to_string()
    }
}
