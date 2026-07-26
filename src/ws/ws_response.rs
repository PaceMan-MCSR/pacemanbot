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
    pub _estimated_counts: HashMap<Item, u32>,
    pub _usages: Option<HashMap<Item, u32>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResponse {
    pub game_version: Option<String>,
    pub _world_id: String,
    pub event_list: Vec<Event>,
    pub _context_event_list: Vec<Event>,
    pub user: User,
    pub _is_cheated: bool,
    pub _is_hidden: bool,
    pub last_updated: i64,
    pub _item_data: Option<ItemData>,
    pub nickname: String,
}
