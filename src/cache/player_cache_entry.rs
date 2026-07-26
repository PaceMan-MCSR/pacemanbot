use crate::cache::Split;

#[derive(Clone)]
pub struct PlayerCacheEntry {
    pub enter_nether: u8,
    pub enter_fortress: u8,
    pub blind: u8,
    pub eye_spy: u8,
    pub end_enter: u8,
    pub finish: Option<u8>,
}

impl Default for PlayerCacheEntry {
    fn default() -> Self {
        Self {
            enter_nether: 0,
            enter_fortress: 0,
            blind: 0,
            eye_spy: 0,
            end_enter: 0,
            finish: None,
        }
    }
}

impl PlayerCacheEntry {
    pub fn get(&self, split: &Split) -> Option<u8> {
        match split {
            Split::EnterNether => Some(self.enter_nether),
            Split::EnterFortress => Some(self.enter_fortress),
            Split::Blind => Some(self.blind),
            Split::EyeSpy => Some(self.eye_spy),
            Split::EndEnter => Some(self.end_enter),
        }
    }
}
