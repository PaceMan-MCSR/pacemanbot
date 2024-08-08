use crate::{
    cache::split::{Split, Structure},
    ws::response::{Event, EventId, Response},
};

use super::run_info::{RunInfo, RunType};

pub fn get_run_info(response: &Response, last_event: &Event) -> Option<RunInfo> {
    match last_event.event_id {
        EventId::RsgEnterBastion => {
            let mut split = Split::FirstStructure;
            let bastion_ss_check = response
                .event_list
                .iter()
                .any(|ctx| ctx.event_id == EventId::RsgEnterFortress);
            let bastion_ss_context_check = response
                .context_event_list
                .iter()
                .any(|ctx| ctx.event_id == EventId::RsgObtainBlazeRod);

            if bastion_ss_check && bastion_ss_context_check {
                split = Split::SecondStructure;
            }
            Some(RunInfo {
                split,
                structure: Some(Structure::Bastion),
                run_type: RunType::Modern,
            })
        }
        EventId::RsgEnterFortress => {
            let mut split = Split::FirstStructure;
            let fort_ss_check = response
                .event_list
                .iter()
                .filter(|evt| evt != &last_event)
                .any(|evt| evt.event_id == EventId::RsgEnterBastion);

            let mut fort_ss_context_check = false;
            let mut context_hits = 0;
            for ctx in response.context_event_list.iter() {
                let context_check = ctx.event_id == EventId::RsgObtainCryingObsidian
                    || ctx.event_id == EventId::RsgObtainObsidian
                    || ctx.event_id == EventId::RsgLootBastion;
                if context_check {
                    context_hits += 1;
                }
            }
            if context_hits >= 2 {
                fort_ss_context_check = true;
            }

            if fort_ss_check && fort_ss_context_check {
                split = Split::SecondStructure;
            }
            Some(RunInfo {
                split,
                structure: Some(Structure::Fortress),
                run_type: RunType::Modern,
            })
        }
        EventId::RsgFirstPortal => {
            let mut run_type = RunType::Modern;
            if response
                .event_list
                .iter()
                .all(|evt| evt.event_id != EventId::RsgEnterBastion)
            {
                run_type = RunType::Bastionless;
            }
            Some(RunInfo {
                split: Split::Blind,
                structure: None,
                run_type,
            })
        }
        _ => {
            let split = Split::from_event_id(&last_event.event_id)?;
            Some(RunInfo {
                split,
                structure: None,
                run_type: RunType::Modern,
            })
        }
    }
}
