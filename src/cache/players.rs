use std::collections::HashMap;

use super::split::Split;

pub type Players = HashMap<String, PlayerSplitsData>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PlayerSplitsData {
    pub adventuring_time: u32,
    pub beaconator: u32,
    pub hdwgh: u32,
    pub finish: Option<u32>,
}

impl PlayerSplitsData {
    pub fn default() -> Self {
        Self {
            adventuring_time: 0,
            beaconator: 0,
            hdwgh: 0,
            finish: None,
        }
    }

    pub fn get(&self, split: &Split) -> Option<u32> {
        match split {
            Split::AdventuringTime => Some(self.adventuring_time),
            Split::Beaconator => Some(self.beaconator),
            Split::HDWGH => Some(self.hdwgh),
            _ => self.finish,
        }
    }
}
