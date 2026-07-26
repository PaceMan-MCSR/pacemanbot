use std::error::Error;

use regex::Regex;

use crate::{
    cache::{PlayerCacheEntry, Split},
    config::ROLE_PREFIX,
};

pub fn extract_split_from_role_name(role_name: &str) -> Result<(Split, u8, u8), Box<dyn Error>> {
    let role_name = role_name.replace(ROLE_PREFIX, "");
    let role_name = role_name.replace(" ", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)")?;
    let caps = match re.captures(&role_name) {
        Some(caps) => caps,
        None => {
            return Err(format!("failed to capture regex for role name: '{}'.", role_name).into())
        }
    };
    let character = match caps.get(1) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get first regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .to_string();
    let hours = match caps.get(2) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get second regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .parse::<u8>()?;
    let minutes = match caps.get(3) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get second regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .parse::<u8>()?;
    let split = Split::from_str(character.as_str()).unwrap();
    Ok((split, hours, minutes))
}

pub fn extract_split_from_pb_role_name(role_name: &str) -> Option<Split> {
    let role_name = role_name.replace(ROLE_PREFIX, "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("PB", "");
    Split::from_str(role_name.as_str())
}

pub fn extract_splits_and_name_from_role_name(
    role_name: &str,
) -> Result<(Split, u8, u8, String), Box<dyn Error>> {
    let role_name = role_name.replace(ROLE_PREFIX, "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("+", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)([a-zA-Z_]+)")?;
    let caps = match re.captures(&role_name) {
        Some(caps) => caps,
        None => {
            return Err(format!("failed to capture regex for role name: '{}'.", role_name).into())
        }
    };
    let character = match caps.get(1) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get first regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .to_string();
    let hours = match caps.get(2) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get second regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .parse::<u8>()?;
    let minutes = match caps.get(3) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get second regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .parse::<u8>()?;
    let split = Split::from_str(character.as_str()).unwrap();
    let name = match caps.get(5) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get fourth regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .to_string();
    Ok((split, hours, minutes, name))
}

pub fn extract_name_or_uuid_and_splits_from_config_line(
    line: &str,
) -> Result<(String, PlayerCacheEntry), Box<dyn Error>> {
    let line = line.trim();
    let line = line.replace(" ", "");
    let line_splits = line.split(':').collect::<Vec<&str>>();
    if line_splits.len() != 2 {
        return Err(format!("failed to parse line contents: '{}'.", line).into());
    }
    let (player_name, splits_string) = (line_splits[0], line_splits[1]);
    let splits = splits_string.split('/').collect::<Vec<&str>>();
    if splits.len() != 3 && splits.len() != 4 {
        return Err(format!("failed to parse line contents: '{}'.", line).into());
    }
    let mut idx = 0;
    let mut split_data = PlayerCacheEntry::default();
    for split in splits {
        let time_splits = split.split(';').collect::<Vec<&str>>();
        if time_splits.len() != 2 {
            return Err(format!("failed to parse time: '{}'.", split).into());
        }
        let (split_hours_u32, split_minutes_u32) = (
            match time_splits[0].parse::<u32>() {
                Ok(split) => split,
                Err(err) => {
                    return Err(format!("failed to parse to u8 due to: {}", err).into());
                }
            },
            match time_splits[1].parse::<u32>() {
                Ok(split) => split,
                Err(err) => {
                    return Err(format!("failed to parse to u8 due to: {}", err).into());
                }
            },
        );
        let split_u32 = split_hours_u32 * 60 + split_minutes_u32;
        match idx {
            0 => split_data.adventuring_time = split_u32,
            1 => split_data.beaconator = split_u32,
            2 => split_data.hdwgh = split_u32,
            3 => split_data.finish = Some(split_u32),
            _ => (),
        };
        idx += 1;
    }
    Ok((player_name.to_string(), split_data))
}
