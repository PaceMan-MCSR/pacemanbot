use std::collections::HashMap;

use serde::Deserialize;

use crate::ws::{EventId, Item};

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
    pub usages: Option<HashMap<Item, u32>>,
    pub crafted: Option<HashMap<Item, u32>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResponse {
    pub game_version: Option<String>,
    pub world_id: String,
    pub event_list: Vec<Event>,
    pub context_event_list: Vec<Event>,
    pub user: User,
    pub is_cheated: bool,
    pub is_hidden: bool,
    pub last_updated: i64,
    pub item_data: Option<ItemData>,
    pub nickname: String,
}
