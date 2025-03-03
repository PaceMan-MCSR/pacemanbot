use crate::{
    cache::split::Split,
    ws::response::{Event, EventId, Item, Response},
};

use super::run_info::RunInfo;

pub fn get_run_info(response: &Response, last_event: &Event) -> Option<RunInfo> {
    let item_data = match &response.item_data {
        Some(data) => data,
        None => return None,
    };

    let crafted = match &item_data.crafted {
        Some(crafted) => crafted,
        None => return None,
    };

    let pearls = match crafted.get(&Item::MinecraftEnderPearl) {
        Some(pearls) => pearls,
        None => return None,
    };

    if pearls < &10 {
        return None;
    }

    match last_event.event_id {
        EventId::RsgEnterStronghold => {
            let validity_check = response
                .event_list
                .iter()
                .any(|e| e.event_id == EventId::RsgFirstPortal);
            if !validity_check {
                return None;
            }

            Some(RunInfo {
                split: Split::EyeSpy,
            })
        }
        _ => {
            let split = Split::from_event_id(&last_event.event_id)?;
            Some(RunInfo { split })
        }
    }
}
