use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serenity::{
    client::Context,
    futures::lock::Mutex,
    model::{
        guild::Role,
        id::{ChannelId, GuildId, MessageId},
    },
};

use crate::utils::{
    extract_name_and_splits_from_line, extract_split_from_pb_role_name,
    extract_split_from_role_name,
};

pub type ArcMux<T> = Arc<Mutex<T>>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: String,
    pub rta: i64,
    pub igt: i64,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        let event_id_check = self.event_id == other.event_id;
        let rta_check = self.rta == other.rta;
        let igt_check = self.igt == other.rta;
        event_id_check && rta_check && igt_check
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: String,
    pub live_account: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub world_id: String,
    pub event_list: Vec<Event>,
    pub context_event_list: Vec<Event>,
    pub user: User,
    pub is_cheated: bool,
    pub is_hidden: bool,
    pub last_updated: i64,
    pub nickname: String,
}

pub struct ResponseError {
    reason: String,
}

impl ResponseError {
    pub fn new<T: std::fmt::Display>(err: T) -> Self {
        Self {
            reason: format!("ResponseError: {}", err),
        }
    }
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.reason))
    }
}

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

    pub fn from_event_id(event_id: &str) -> Option<Split> {
        match event_id {
            "rsg.first_portal" => Some(Split::Blind),
            "rsg.enter_stronghold" => Some(Split::EyeSpy),
            "rsg.enter_end" => Some(Split::EndEnter),
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

    pub fn desc(&self, structure: Option<&str>) -> String {
        match self {
            Split::FirstStructure => {
                if let Some(structure) = structure {
                    match structure {
                        "Bastion" => "Enter Bastion",
                        "Fortress" => "Enter Fortress",
                        _ => "",
                    }
                } else {
                    ""
                }
            }
            Split::SecondStructure => {
                if let Some(structure) = structure {
                    match structure {
                        "Bastion" => "Enter Bastion",
                        "Fortress" => "Enter Fortress",
                        _ => "",
                    }
                } else {
                    ""
                }
            }
            Split::Blind => "First Portal",
            Split::EyeSpy => "Enter Stronghold",
            Split::EndEnter => "Enter End",
        }
        .to_string()
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
}

impl PlayerSplitsData {
    pub fn new() -> Self {
        Self {
            first_structure: 0,
            second_structure: 0,
            blind: 0,
            eye_spy: 0,
            end_enter: 0,
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

#[derive(Clone)]
pub struct PlayerData {
    pub splits: PlayerSplitsData,
    pub last_split: Option<Split>,
    pub last_pace_message: Option<MessageId>,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            splits: PlayerSplitsData::new(),
            last_split: None,
            last_pace_message: None,
        }
    }
}

pub struct GuildData {
    pub name: String,
    pub pace_channel: ChannelId,
    pub players: HashMap<String, PlayerData>,
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
        let mut players: HashMap<String, PlayerData> = HashMap::new();
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
                let mut player_data = PlayerData::new();
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
            players,
            roles,
        })
    }
}
