use std::error::Error;

use serenity::model::guild::Role;

use crate::{
    cache::{PlayerCacheEntry, Split},
    config::Config,
    dispatcher::{millis_to_hrs_mins, RunInfo},
    ws::{Advancement, WSResponse},
};

#[derive(Debug)]
pub struct RoleCacheEntry {
    pub split: Split,
    pub hours: u8,
    pub minutes: u8,
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
        last_advancement: &Advancement,
        is_private: bool,
        ws_response: &WSResponse,
    ) -> bool {
        let (split_hours, split_minutes) = millis_to_hrs_mins(last_advancement.igt as u64);
        if self.role.name.contains("PB") {
            if !is_private {
                return false;
            }
            let pb_minutes = player_data.get(&self.split).unwrap().to_owned();
            let pb_hours = (pb_minutes / 60) as u8;
            let pb_minutes = (pb_minutes % 60) as u8;
            self.split == run_info.split && pb_hours > split_hours && pb_minutes > split_minutes
        } else if self.role.name.contains("+") {
            self.split == run_info.split
                && self.runner.to_lowercase() == ws_response.nickname.to_lowercase()
                && self.hours >= split_hours
                && (self.hours != split_hours || self.minutes > split_minutes)
        } else {
            self.split == run_info.split
                && self.hours >= split_hours
                && (self.hours != split_hours || self.minutes > split_minutes)
        }
    }
}
