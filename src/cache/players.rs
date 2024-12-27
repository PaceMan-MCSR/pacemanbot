use std::collections::HashMap;

use super::split::Split;

pub type Players = HashMap<String, PlayerSplitsData>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlayerSplitsData {
    pub enter_fortress: u8,
    pub blind: u8,
    pub eye_spy: u8,
    pub end_enter: u8,
    pub finish: Option<u8>,
}

impl PlayerSplitsData {
    pub fn default() -> Self {
        Self {
            enter_fortress: 0,
            blind: 0,
            eye_spy: 0,
            end_enter: 0,
            finish: None,
        }
    }

    pub fn get(&self, split: &Split) -> Option<u8> {
        match split {
            Split::EnterFortress => Some(self.enter_fortress),
            Split::Blind => Some(self.blind),
            Split::EyeSpy => Some(self.eye_spy),
            Split::EndEnter => Some(self.end_enter),
        }
    }
}
