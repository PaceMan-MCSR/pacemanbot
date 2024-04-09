use std::collections::HashMap;

use serenity::{
    client::Context,
    model::{
        guild::Role,
        id::{ChannelId, GuildId, MessageId},
    },
};

use crate::{
    response_types::{EventId, Structure},
    utils::{
        extract_name_and_splits_from_line, extract_split_from_pb_role_name,
        extract_split_from_role_name,
    },
};

pub type CachedGuilds = HashMap<GuildId, GuildData>;
pub type Players = HashMap<String, PlayerData>;

#[derive(PartialEq, Debug, Clone)]
pub enum Split {
    FirstStructure,
    SecondStructure,
    Blind,
    EyeSpy,
    EndEnter,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "FS" => Some(Split::FirstStructure),
            "SS" => Some(Split::SecondStructure),
            "B" => Some(Split::Blind),
            "E" => Some(Split::EyeSpy),
            "EE" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_event_id(event_id: &EventId) -> Option<Split> {
        match event_id {
            EventId::RsgFirstPortal => Some(Split::Blind),
            EventId::RsgEnterStronghold => Some(Split::EyeSpy),
            EventId::RsgEnterEnd => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "first_structure" => Some(Split::FirstStructure),
            "second_structure" => Some(Split::SecondStructure),
            "blind" => Some(Split::Blind),
            "eye_spy" => Some(Split::EyeSpy),
            "end_enter" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn desc(&self, structure: &Option<Structure>) -> Option<String> {
        Some(
            match self {
                Split::FirstStructure => match structure {
                    Some(structure) => match structure {
                        Structure::Bastion => "Enter Bastion",
                        Structure::Fortress => "Enter Fortress",
                    },
                    None => return None,
                },
                Split::SecondStructure => match structure {
                    Some(structure) => match structure {
                        Structure::Bastion => "Enter Bastion",
                        Structure::Fortress => "Enter Fortress",
                    },
                    None => return None,
                },
                Split::Blind => "First Portal",
                Split::EyeSpy => "Enter Stronghold",
                Split::EndEnter => "Enter End",
            }
            .to_string(),
        )
    }

    pub fn alt_desc(&self) -> String {
        match self {
            Split::FirstStructure => "Structure 1",
            Split::SecondStructure => "Structure 2",
            Split::Blind => "Blind",
            Split::EyeSpy => "Eye Spy",
            Split::EndEnter => "End Enter",
        }
        .to_string()
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::FirstStructure => "FS",
            Split::SecondStructure => "SS",
            Split::Blind => "B",
            Split::EyeSpy => "E",
            Split::EndEnter => "EE",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct RoleData {
    pub split: Split,
    pub minutes: u8,
    pub seconds: u8,
    pub guild_role: Role,
}

impl RoleData {
    pub fn new(guild_role: Role) -> Result<Self, Box<dyn std::error::Error>> {
        let split: Split;
        let mut minutes: u8 = 0;
        let mut seconds: u8 = 0;
        if guild_role.name.contains("PB") {
            split = match extract_split_from_pb_role_name(guild_role.name.as_str()) {
                Some(tup) => tup,
                None => {
                    return Err(format!(
                        "Unable to extract split from pb role name: {}.",
                        guild_role.name
                    )
                    .into())
                }
            };
        } else {
            (split, minutes, seconds) = match extract_split_from_role_name(guild_role.name.as_str())
            {
                Ok(tup) => tup,
                Err(err) => {
                    return Err(format!(
                        "Unable to extract split from role name: {} due to: {}",
                        guild_role.name, err
                    )
                    .into())
                }
            };
        }
        Ok(Self {
            guild_role,
            split,
            minutes,
            seconds,
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlayerSplitsData {
    pub first_structure: u8,
    pub second_structure: u8,
    pub blind: u8,
    pub eye_spy: u8,
    pub end_enter: u8,
    pub finish: Option<u8>,
}

impl PlayerSplitsData {
    pub fn default() -> Self {
        Self {
            first_structure: 0,
            second_structure: 0,
            blind: 0,
            eye_spy: 0,
            end_enter: 0,
            finish: None,
        }
    }

    pub fn get(&self, split: &Split) -> Option<u8> {
        match split {
            Split::FirstStructure => Some(self.first_structure),
            Split::SecondStructure => Some(self.second_structure),
            Split::Blind => Some(self.blind),
            Split::EyeSpy => Some(self.eye_spy),
            Split::EndEnter => Some(self.end_enter),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerData {
    pub splits: PlayerSplitsData,
    pub last_pace_message: Option<MessageId>,
}

impl PlayerData {
    pub fn default() -> Self {
        Self {
            splits: PlayerSplitsData::default(),
            last_pace_message: None,
        }
    }
}

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
    pub async fn new(ctx: &Context, guild_id: GuildId) -> Result<Self, Box<dyn std::error::Error>> {
        let guild = match ctx.cache.guild(guild_id) {
            Some(name) => name,
            None => {
                return Err(
                    format!("Unable to construct guild from guild id: {}", guild_id).into(),
                );
            }
        };
        let name = guild.name;

        let channels = match ctx.cache.guild_channels(guild_id) {
            Some(channels) => channels,
            None => return Err(format!("Unable to get channels from guild name: {}", name,).into()),
        };
        let pace_channel = match channels.iter().find(|c| c.name == "pacemanbot") {
            Some(channel) => channel.id,
            None => {
                return Err(format!("Unable to find #pacemanbot in guild name: {}", name,).into());
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
                    "Unable to get first message from #pacemanbot-runner-names in guild name: {}.",
                    name
                )
                    .into())
                }
            };
            for line in first_message.content.split("\n") {
                let (name, splits) = extract_name_and_splits_from_line(line)?;
                let mut player_data = PlayerData::default();
                player_data.splits = splits;
                players.insert(name.to_lowercase(), player_data);
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
                        "Unable to make role data for role: {} in guild name: {} due to: {}",
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
