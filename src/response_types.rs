use serde::Deserialize;

use crate::guild_types::Split;

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
    #[serde(rename = "rsg.kill_dragon")]
    RsgKillDragon,
}

#[derive(Debug)]
pub enum EventType {
    CommonEvent,
    NonPaceEvent,
    PaceEvent,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: EventId,
    pub rta: i64,
    pub igt: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: String,
    pub live_account: Option<String>,
}

#[derive(Deserialize, Debug)]
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

pub enum Structure {
    Bastion,
    Fortress,
}

pub enum RunType {
    Bastionless,
    Modern,
}

pub struct RunInfo {
    pub split: Split,
    pub structure: Option<Structure>,
    pub run_type: RunType,
}
