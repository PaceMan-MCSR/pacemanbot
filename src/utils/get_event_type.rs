use crate::ws::{
    consts::TOTAL_ADVANCEMENTS_116,
    response::{Advancement, AdvancementId, EventType},
};

pub fn get_event_type(last_advancement: &Advancement, completed: usize) -> Option<EventType> {
    match last_advancement.event_id {
        AdvancementId::AdventureAdventuringTime
        | AdvancementId::NetherAllEffects
        | AdvancementId::NetherCreateFullBeacon => Some(EventType::PaceEvent),
        _ => {
            if completed == TOTAL_ADVANCEMENTS_116 {
                Some(EventType::NonPaceEvent)
            } else {
                None
            }
        }
    }
}
