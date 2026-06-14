use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
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
    #[serde(rename = "rsg.loot_monument")]
    RsgLootMonument,
    #[serde(rename = "rsg.tower_start")]
    RsgTowerStart,
    #[serde(rename = "rsg.trade")]
    RsgTrade,
    #[serde(rename = "rsg.killed_blaze")]
    RsgKilledBlaze,
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
