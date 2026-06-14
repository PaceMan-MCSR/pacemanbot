use crate::ws::{Event, EventId};

pub enum EventType {
    NonPaceEvent,
    PaceEvent,
    Unknown,
}

impl From<&Event> for EventType {
    fn from(value: &Event) -> Self {
        match value.event_id {
            EventId::RsgEnterBastion
            | EventId::RsgEnterFortress
            | EventId::RsgFirstPortal
            | EventId::RsgEnterStronghold
            | EventId::RsgEnterEnd => EventType::PaceEvent,
            EventId::RsgCredits => EventType::NonPaceEvent,
            _ => EventType::Unknown,
        }
    }
}
