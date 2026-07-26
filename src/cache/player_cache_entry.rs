use crate::cache::Split;

#[derive(Clone)]
pub struct PlayerCacheEntry {
    pub tower_start: u8,
    pub end_enter: u8,
    pub finish: Option<u8>,
}

impl Default for PlayerCacheEntry {
    fn default() -> Self {
        Self {
            tower_start: 0,
            end_enter: 0,
            finish: None,
        }
    }
}

impl PlayerCacheEntry {
    pub fn get(&self, split: &Split) -> Option<u8> {
        match split {
            Split::TowerStart => Some(self.tower_start),
            Split::EndEnter => Some(self.end_enter),
        }
    }
}
