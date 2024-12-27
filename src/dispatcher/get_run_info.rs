use crate::{
    cache::split::Split,
    ws::response::{Event, EventId},
};

use super::run_info::RunInfo;

pub fn get_run_info(last_event: &Event) -> Option<RunInfo> {
    match last_event.event_id {
        EventId::RsgEnterFortress => Some(RunInfo {
            split: Split::EnterFortress,
        }),
        EventId::RsgFirstPortal => Some(RunInfo {
            split: Split::Blind,
        }),
        _ => {
            let split = Split::from_event_id(&last_event.event_id)?;
            Some(RunInfo { split })
        }
    }
}
