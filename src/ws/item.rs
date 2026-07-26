use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Item {
    #[serde(rename = "minecraft:ender_pearl")]
    MinecraftEnderPearl,
    #[serde(rename = "minecraft:obsidian")]
    MinecraftObsidian,
    #[serde(rename = "minecraft:blaze_rod")]
    MinecraftBlazeRod,
}
