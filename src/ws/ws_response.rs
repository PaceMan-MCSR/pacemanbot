use serde::Deserialize;

use crate::ws::{Advancement, EventId};

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

#[derive(Deserialize, Debug)]
pub struct Items {
    pub has_enchanted_golden_apple: bool,
    pub skulls: u64,
    pub gold_blocks: u64,
    pub ancient_debris: u64,
}

#[derive(Deserialize, Debug)]
pub struct Context {
    pub shells: u64,
    pub _mesa: Vec<u64>,
    pub _snowy: Vec<u64>,
    pub _jungle: Vec<u64>,
    pub _mushroom: Vec<u64>,
    pub phantoms: Vec<u64>,
    pub thunder: Vec<u64>,
    pub _endgame: Vec<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Criterias {
    pub _biomes: Vec<String>,
    pub _monsters_killed: Vec<String>,
    pub _animals_bred: Vec<String>,
    pub _cats_tamed: Vec<String>,
    pub _food_eaten: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSResponse {
    pub completed: Vec<Advancement>,
    pub event_list: Vec<Event>,
    pub context: Context,
    pub _current_time: u64,
    pub user: User,
    pub world_id: String,
    pub _is_cheated: bool,
    pub _is_hidden: bool,
    pub last_updated: i64,
    pub nickname: String,
    pub _criterias: Criterias,
    pub items: Items,
}
