use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum AdvancementId {
    #[serde(rename = "nether/obtain_crying_obsidian")]
    NetherObtainCryingObsidian,
    #[serde(rename = "nether/distract_piglin")]
    NetherDistractPiglin,
    #[serde(rename = "story/obtain_armor")]
    StoryObtainArmor,
    #[serde(rename = "adventure/very_very_frightening")]
    AdventureVeryVeryFrightening,
    #[serde(rename = "story/lava_bucket")]
    StoryLavaBucket,
    #[serde(rename = "end/kill_dragon")]
    EndKillDragon,
    #[serde(rename = "nether/all_potions")]
    NetherAllPotions,
    #[serde(rename = "husbandry/tame_an_animal")]
    HusbandryTameAnAnimal,
    #[serde(rename = "nether/create_beacon")]
    NetherCreateBeacon,
    #[serde(rename = "story/deflect_arrow")]
    StoryDeflectArrow,
    #[serde(rename = "story/iron_tools")]
    StoryIronTools,
    #[serde(rename = "nether/brew_potion")]
    NetherBrewPotion,
    #[serde(rename = "end/dragon_egg")]
    EndDragonEgg,
    #[serde(rename = "husbandry/fishy_business")]
    HusbandryFishyBusiness,
    #[serde(rename = "nether/explore_nether")]
    NetherExploreNether,
    #[serde(rename = "nether/ride_strider")]
    NetherRideStrider,
    #[serde(rename = "adventure/sniper_duel")]
    AdventureSniperDuel,
    #[serde(rename = "nether/root")]
    NetherRoot,
    #[serde(rename = "end/levitate")]
    EndLevitate,
    #[serde(rename = "nether/all_effects")] // HDWGH
    NetherAllEffects,
    #[serde(rename = "adventure/bullseye")]
    AdventureBullseye,
    #[serde(rename = "nether/get_wither_skull")]
    NetherGetWitherSkull,
    #[serde(rename = "husbandry/bred_all_animals")]
    HusbandryBredAllAnimals,
    #[serde(rename = "story/mine_stone")]
    StoryMineStone,
    #[serde(rename = "adventure/two_birds_one_arrow")]
    AdventureTwoBirdsOneArrow,
    #[serde(rename = "story/enter_the_nether")]
    StoryEnterTheNether,
    #[serde(rename = "adventure/whos_the_pillager_now")]
    AdventureWhosThePillagerNow,
    #[serde(rename = "story/upgrade_tools")]
    StoryUpgradeTools,
    #[serde(rename = "husbandry/tactical_fishing")]
    HusbandryTacticalFishing,
    #[serde(rename = "story/cure_zombie_villager")]
    StoryCureZombieVillager,
    #[serde(rename = "end/find_end_city")]
    EndFindEndCity,
    #[serde(rename = "story/form_obsidian")]
    StoryFormObsidian,
    #[serde(rename = "end/enter_end_gateway")]
    EndEnterEndGateway,
    #[serde(rename = "nether/obtain_blaze_rod")]
    NetherObtainBlazeRod,
    #[serde(rename = "nether/loot_bastion")]
    NetherLootBastion,
    #[serde(rename = "adventure/shoot_arrow")]
    AdventureShootArrow,
    #[serde(rename = "husbandry/silk_touch_nest")]
    HusbandrySilkTouchNest,
    #[serde(rename = "adventure/arbalistic")]
    AdventureArbalistic,
    #[serde(rename = "end/respawn_dragon")] // Finish
    EndRespawnDragon,
    #[serde(rename = "story/smelt_iron")]
    StorySmeltIron,
    #[serde(rename = "nether/charge_respawn_anchor")]
    NetherChargeRespawnAnchor,
    #[serde(rename = "story/shiny_gear")]
    StoryShinyGear,
    #[serde(rename = "end/elytra")]
    EndElytra,
    #[serde(rename = "adventure/summon_iron_golem")]
    AdventureSummonIronGolem,
    #[serde(rename = "nether/return_to_sender")]
    NetherReturnToSender,
    #[serde(rename = "adventure/sleep_in_bed")]
    AdventureSleepInBed,
    #[serde(rename = "end/dragon_breath")]
    EndDragonBreath,
    #[serde(rename = "adventure/root")]
    AdventureRoot,
    #[serde(rename = "adventure/kill_all_mobs")]
    AdventureKillAllMobs,
    #[serde(rename = "story/enchant_item")]
    StoryEnchantItem,
    #[serde(rename = "adventure/voluntary_exile")]
    AdventureVoluntaryExile,
    #[serde(rename = "story/follow_ender_eye")]
    StoryFollowEnderEye,
    #[serde(rename = "end/root")]
    EndRoot,
    #[serde(rename = "husbandry/obtain_netherite_hoe")]
    HusbandryObtainNetheriteHoe,
    #[serde(rename = "adventure/totem_of_undying")]
    AdventureTotemOfUndying,
    #[serde(rename = "adventure/kill_a_mob")]
    AdventureKillAMob,
    #[serde(rename = "adventure/adventuring_time")] // AdventuringTime
    AdventureAdventuringTime,
    #[serde(rename = "husbandry/plant_seed")]
    HusbandryPlantSeed,
    #[serde(rename = "nether/find_bastion")]
    NetherFindBastion,
    #[serde(rename = "adventure/hero_of_the_village")]
    AdventureHeroOfTheVillage,
    #[serde(rename = "nether/obtain_ancient_debris")]
    NetherObtainAncientDebris,
    #[serde(rename = "nether/create_full_beacon")] // Beaconator
    NetherCreateFullBeacon,
    #[serde(rename = "nether/summon_wither")]
    NetherSummonWither,
    #[serde(rename = "husbandry/balanced_diet")]
    HusbandryBalancedDiet,
    #[serde(rename = "nether/fast_travel")]
    NetherFastTravel,
    #[serde(rename = "husbandry/root")]
    HusbandryRoot,
    #[serde(rename = "nether/use_lodestone")]
    NetherUseLodestone,
    #[serde(rename = "husbandry/safely_harvest_honey")]
    HusbandrySafelyHarvestHoney,
    #[serde(rename = "adventure/trade")]
    AdventureTrade,
    #[serde(rename = "nether/uneasy_alliance")]
    NetherUneasyAlliance,
    #[serde(rename = "story/mine_diamond")]
    StoryMineDiamond,
    #[serde(rename = "nether/find_fortress")]
    NetherFindFortress,
    #[serde(rename = "adventure/throw_trident")]
    AdventureThrowTrident,
    #[serde(rename = "story/root")]
    StoryRoot,
    #[serde(rename = "adventure/honey_block_slide")]
    AdventureHoneyBlockSlide,
    #[serde(rename = "adventure/ol_betsy")]
    AdventureOlBetsy,
    #[serde(rename = "nether/netherite_armor")]
    NetherNetheriteArmor,
    #[serde(rename = "story/enter_the_end")]
    StoryEnterTheEnd,
    #[serde(rename = "husbandry/breed_an_animal")]
    HusbandryBreedAnAnimal,
    #[serde(rename = "husbandry/complete_catalogue")]
    HusbandryCompleteCgtalogue,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Advancement {
    pub event_id: AdvancementId,
    pub rta: i64,
    pub igt: i64,
}
