use crate::{
    cache::split::Split,
    ws::response::{Advancement, AdvancementId, Response},
};

use super::run_info::RunInfo;

pub fn get_run_info(response: &Response, last_advancement: &Advancement) -> Option<RunInfo> {
    let num_advancements = response.completed.len();
    let thunder_check = response.context.thunder.len() >= 1;
    let phantom_check = response.context.phantoms.len() >= 1;
    let shells_check = response.context.shells >= 1;
    let skulls_check = response.items.skulls >= 1;
    let gold_blocks_check = response.items.gold_blocks >= 1;
    let debris_check = response.items.ancient_debris >= 1;
    let adventuring_time_check = response
        .completed
        .iter()
        .any(|adv| adv.event_id == AdvancementId::AdventureAdventuringTime);
    match last_advancement.event_id {
        AdvancementId::AdventureAdventuringTime => {
            if num_advancements < 30 {
                return None;
            }
            if !(thunder_check
                && shells_check
                && phantom_check
                && skulls_check
                && gold_blocks_check)
            {
                return None;
            }
            Some(RunInfo {
                split: Split::AdventuringTime,
            })
        }
        AdvancementId::NetherCreateFullBeacon => {
            if num_advancements < 50 {
                return None;
            }
            if !(thunder_check && adventuring_time_check && debris_check && phantom_check) {
                return None;
            }
            Some(RunInfo {
                split: Split::Beaconator,
            })
        }
        AdvancementId::NetherAllEffects => {
            if num_advancements < 50 {
                return None;
            }
            if !(thunder_check && adventuring_time_check && debris_check && phantom_check) {
                return None;
            }
            Some(RunInfo {
                split: Split::HDWGH,
            })
        }
        _ => None,
    }
}
