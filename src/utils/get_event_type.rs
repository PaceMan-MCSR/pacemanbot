use crate::ws::response::{Event, EventId, EventType};

pub fn get_event_type(last_event: &Event) -> Option<EventType> {
    match last_event.event_id {
        EventId::RsgTowerStart | EventId::RsgEnterEnd => Some(EventType::PaceEvent),
        EventId::RsgCredits => Some(EventType::NonPaceEvent),
        _ => None,
    }
}
