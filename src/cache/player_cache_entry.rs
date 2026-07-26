use crate::cache::Split;

#[derive(Clone)]
pub struct PlayerCacheEntry {
    pub adventuring_time: u32,
    pub beaconator: u32,
    pub hdwgh: u32,
    pub finish: Option<u32>,
}

impl Default for PlayerCacheEntry {
    fn default() -> Self {
        Self {
            adventuring_time: 0,
            beaconator: 0,
            hdwgh: 0,
            finish: None,
        }
    }
}

impl PlayerCacheEntry {
    pub fn get(&self, split: &Split) -> Option<u32> {
        match split {
            Split::AdventuringTime => Some(self.adventuring_time),
            Split::Beaconator => Some(self.beaconator),
            Split::HDWGH => Some(self.hdwgh),
            _ => self.finish,
        }
    }
}
