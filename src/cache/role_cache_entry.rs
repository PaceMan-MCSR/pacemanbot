use std::error::Error;

use serenity::model::guild::Role;

use crate::{
    cache::{PlayerCacheEntry, Split},
    config::Config,
    dispatcher::{millis_to_mins_secs, RunInfo},
    ws::{Event, WSResponse},
};

#[derive(Debug)]
pub struct RoleCacheEntry {
    pub split: Split,
    pub minutes: u8,
    pub seconds: u8,
    pub runner: String,
    pub role: Role,
}

impl RoleCacheEntry {
    pub fn new(role: Role) -> Result<Self, Box<dyn Error>> {
        return Config::parse_role_config_for_role(role);
    }

    pub fn is_pingable(
        &self,
        player_data: &PlayerCacheEntry,
        run_info: &RunInfo,
        last_event: &Event,
        is_private: bool,
        ws_response: &WSResponse,
    ) -> bool {
        let (split_minutes, split_seconds) = millis_to_mins_secs(last_event.igt as u64);
        if self.role.name.contains("PB") {
            if !is_private {
                return false;
            }
            let pb_minutes = player_data.get(&self.split).unwrap().to_owned();
            self.split == run_info.split && pb_minutes > split_minutes
        } else if self.role.name.contains("+") {
            self.split == run_info.split
                && self.runner.to_lowercase() == ws_response.nickname.to_lowercase()
                && self.minutes >= split_minutes
                && (self.minutes != split_minutes || self.seconds > split_seconds)
        } else {
            self.split == run_info.split
                && self.minutes >= split_minutes
                && (self.minutes != split_minutes || self.seconds > split_seconds)
        }
    }
}
