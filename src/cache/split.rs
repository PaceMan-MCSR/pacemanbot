use crate::{
    dispatcher::consts::{END_EMOJI, PORTAL_EMOJI},
    ws::response::EventId,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Split {
    TowerStart,
    EndEnter,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "T" => Some(Split::TowerStart),
            "EE" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_event_id(event_id: &EventId) -> Option<Split> {
        match event_id {
            EventId::RsgTowerStart => Some(Split::TowerStart),
            EventId::RsgEnterEnd => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "tower_start" => Some(Split::TowerStart),
            "end_enter" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn desc(&self) -> String {
        match self {
            Split::TowerStart => "Tower Start",
            Split::EndEnter => "Enter End",
        }
        .to_string()
    }

    pub fn get_emoji(&self) -> Option<String> {
        Some(
            match self {
                Split::TowerStart => PORTAL_EMOJI,
                Split::EndEnter => END_EMOJI,
            }
            .to_string(),
        )
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::TowerStart => "T",
            Split::EndEnter => "EE",
        }
        .to_string()
    }
}
