use crate::{
    dispatcher::TOTAL_ADVANCEMENTS_116,
    ws::{Advancement, AdvancementId},
};

pub enum EventType {
    NonPaceEvent,
    PaceEvent,
    Unknown,
}

impl EventType {
    pub fn from_advancement(last_advancement: &Advancement, completed: usize) -> Self {
        match last_advancement.event_id {
            AdvancementId::AdventureAdventuringTime
            | AdvancementId::NetherAllEffects
            | AdvancementId::NetherCreateFullBeacon => Self::PaceEvent,
            _ => {
                if completed == TOTAL_ADVANCEMENTS_116 {
                    EventType::NonPaceEvent
                } else {
                    EventType::Unknown
                }
            }
        }
    }
}
