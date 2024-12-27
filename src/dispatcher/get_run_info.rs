use crate::{cache::split::Split, ws::response::Event};

use super::run_info::RunInfo;

pub fn get_run_info(last_event: &Event) -> Option<RunInfo> {
    match last_event.event_id {
        _ => {
            let split = Split::from_event_id(&last_event.event_id)?;
            Some(RunInfo { split })
        }
    }
}
