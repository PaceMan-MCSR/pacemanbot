use std::collections::HashMap;

use super::split::Split;

pub type Players = HashMap<String, PlayerSplitsData>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlayerSplitsData {
    pub tower_start: u8,
    pub end_enter: u8,
    pub finish: Option<u8>,
}

impl PlayerSplitsData {
    pub fn default() -> Self {
        Self {
            tower_start: 0,
            end_enter: 0,
            finish: None,
        }
    }

    pub fn get(&self, split: &Split) -> Option<u8> {
        match split {
            Split::TowerStart => Some(self.tower_start),
            Split::EndEnter => Some(self.end_enter),
        }
    }
}
