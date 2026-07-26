use crate::{cache::Split, ws::Event};

pub struct RunInfo {
    pub split: Split,
}

impl Default for RunInfo {
    fn default() -> Self {
        Self {
            split: Split::TowerStart,
        }
    }
}

impl RunInfo {
    pub fn from_last_event(last_event: &Event) -> Option<Self> {
        match last_event.event_id {
            _ => {
                let split = Split::from_event_id(&last_event.event_id)?;
                Some(RunInfo { split })
            }
        }
    }
}
