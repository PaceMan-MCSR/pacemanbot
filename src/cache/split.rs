use crate::{
    dispatcher::consts::{END_EMOJI, FORT_EMOJI, PORTAL_EMOJI, SH_EMOJI},
    ws::response::EventId,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Split {
    EnterFortress,
    Blind,
    EyeSpy,
    EndEnter,
}

impl Split {
    pub fn from_str(split: &str) -> Option<Split> {
        match split {
            "F" => Some(Split::EnterFortress),
            "B" => Some(Split::Blind),
            "E" => Some(Split::EyeSpy),
            "EE" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_event_id(event_id: &EventId) -> Option<Split> {
        match event_id {
            EventId::RsgFirstPortal => Some(Split::Blind),
            EventId::RsgEnterStronghold => Some(Split::EyeSpy),
            EventId::RsgEnterEnd => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn from_command_param(param: &str) -> Option<Split> {
        match param {
            "enter_fortress" => Some(Split::EnterFortress),
            "blind" => Some(Split::Blind),
            "eye_spy" => Some(Split::EyeSpy),
            "end_enter" => Some(Split::EndEnter),
            _ => None,
        }
    }

    pub fn desc(&self) -> String {
        match self {
            Split::EnterFortress => "Enter Fortress",
            Split::Blind => "First Portal",
            Split::EyeSpy => "Enter Stronghold",
            Split::EndEnter => "Enter End",
        }
        .to_string()
    }

    pub fn get_emoji(&self) -> Option<String> {
        Some(
            match self {
                Split::EnterFortress => FORT_EMOJI,
                Split::Blind => PORTAL_EMOJI,
                Split::EyeSpy => SH_EMOJI,
                Split::EndEnter => END_EMOJI,
            }
            .to_string(),
        )
    }

    pub fn to_str(&self) -> String {
        match self {
            Split::EnterFortress => "F",
            Split::Blind => "B",
            Split::EyeSpy => "E",
            Split::EndEnter => "EE",
        }
        .to_string()
    }
}
