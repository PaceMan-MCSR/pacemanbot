use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum EventId {
    #[serde(rename = "common.open_to_lan")]
    CommonOpenToLan,
    #[serde(rename = "common.multiplayer")]
    CommonMultiplayer,
    #[serde(rename = "common.enable_cheats")]
    CommonEnableCheats,
    #[serde(rename = "common.view_seed")]
    CommonViewSeed,
    #[serde(rename = "common.leave_world")]
    CommonLeaveWorld,
    #[serde(rename = "common.rejoin_world")]
    CommonRejoinWorld,
    #[serde(rename = "common.old_world")]
    CommondOldWorld,

    #[serde(rename = "rsg.enter_nether")]
    RsgEnterNether,
    #[serde(rename = "rsg.enter_bastion")]
    RsgEnterBastion,
    #[serde(rename = "rsg.enter_fortress")]
    RsgEnterFortress,
    #[serde(rename = "rsg.first_portal")]
    RsgFirstPortal,
    #[serde(rename = "rsg.second_portal")]
    RsgSecondPortal,
    #[serde(rename = "rsg.enter_stronghold")]
    RsgEnterStronghold,
    #[serde(rename = "rsg.enter_end")]
    RsgEnterEnd,
    #[serde(rename = "rsg.credits")]
    RsgCredits,

    #[serde(rename = "rsg.obtain_iron_ingot")]
    RsgObtainIronIngot,
    #[serde(rename = "rsg.obtain_iron_pickaxe")]
    RsgObtainIronPickaxe,
    #[serde(rename = "rsg.obtain_lava_bucket")]
    RsgObtainLavaBucket,
    #[serde(rename = "rsg.obtain_gold_block")]
    RsgObtainGoldBlock,
    #[serde(rename = "rsg.distract_piglin")]
    RsgDistractPiglin,
    #[serde(rename = "rsg.loot_bastion")]
    RsgLootBastion,
    #[serde(rename = "rsg.obtain_crying_obsidian")]
    RsgObtainCryingObsidian,
    #[serde(rename = "rsg.obtain_obsidian")]
    RsgObtainObsidian,
    #[serde(rename = "rsg.obtain_blaze_rod")]
    RsgObtainBlazeRod,
    #[serde(rename = "rsg.tower_start")]
    RsgTowerStart,
    #[serde(rename = "rsg.kill_dragon")]
    RsgKillDragon,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: EventId,
    pub rta: i64,
    pub igt: i64,
}

#[derive(Debug)]
pub enum EventType {
    NonPaceEvent,
    PaceEvent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: String,
    pub live_account: Option<String>,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    #[serde(rename = "minecraft:ender_pearl")]
    MinecraftEnderPearl,
    #[serde(rename = "minecraft:obsidian")]
    MinecraftObsidian,
    #[serde(rename = "minecraft:blaze_rod")]
    MinecraftBlazeRod,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub estimated_counts: HashMap<Item, u32>,
    pub _usages: Option<HashMap<Item, u32>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
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
