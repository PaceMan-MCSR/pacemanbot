use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    cache::{Split, Structure},
    dispatcher::{RunInfo, PEARL_EMOJI, ROD_EMOJI},
    ws::{EventId, Item},
};

#[derive(Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: EventId,
    pub rta: i64,
    pub igt: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: String,
    pub live_account: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub estimated_counts: HashMap<Item, u32>,
    pub _usages: Option<HashMap<Item, u32>>,
}

impl ItemData {
    pub fn format_item_count(source: &mut String, emoji: &str, item_count: String) {
        *source = format!("{} {} {}", source, emoji, item_count);
    }

    pub fn to_formatted_message(item_data: Option<ItemData>, run_info: &RunInfo) -> String {
        let pearl_count;
        let mut rod_count;
        match item_data {
            Some(item_data) => {
                pearl_count = item_data
                    .estimated_counts
                    .get(&Item::MinecraftEnderPearl)
                    .unwrap_or(&0)
                    .to_string();
                rod_count = item_data
                    .estimated_counts
                    .get(&Item::MinecraftBlazeRod)
                    .unwrap_or(&0)
                    .to_string();
                if let Some(Structure::Bastion) = run_info.structure {
                    if rod_count == "0".to_string() && run_info.split == Split::SecondStructure {
                        rod_count = "1+".to_string();
                    }
                }
            }
            None => {
                pearl_count = "0".to_string();
                rod_count = "0".to_string();
            }
        }
        let mut msg = String::new();
        ItemData::format_item_count(&mut msg, ROD_EMOJI, rod_count);
        ItemData::format_item_count(&mut msg, PEARL_EMOJI, pearl_count);
        msg
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResponse {
    pub game_version: Option<String>,
    pub world_id: String,
    pub event_list: Vec<Event>,
    pub context_event_list: Vec<Event>,
    pub user: User,
    pub _is_cheated: bool,
    pub _is_hidden: bool,
    pub last_updated: i64,
    pub item_data: Option<ItemData>,
    pub nickname: String,
}
