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
    let minutes = match caps.get(2) {
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
    let seconds = match caps.get(3) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get third regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .parse::<u8>()?
        * 10;
    let split = Split::from_str(character.as_str()).unwrap();
    Ok((split, minutes, seconds))
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
    let minutes = match caps.get(2) {
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
    let seconds = match caps.get(3) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "failed to get third regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .parse::<u8>()?
        * 10;
    let split = Split::from_str(character.as_str()).unwrap();
    let name = match caps.get(4) {
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
    Ok((split, minutes, seconds, name))
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
    let (player_name_or_uuid, splits_string) = (line_splits[0], line_splits[1]);
    let splits = splits_string.split('/').collect::<Vec<&str>>();
    if splits.len() != 2 && splits.len() != 3 {
        return Err(format!("failed to parse line contents: '{}'.", line).into());
    }
    let mut idx = 0;
    let mut split_data = PlayerCacheEntry::default();
    for split in splits {
        let split_u8 = match split.parse::<u8>() {
            Ok(split) => split,
            Err(err) => {
                return Err(format!("failed to parse to u8 due to: {}", err).into());
            }
        };
        match idx {
            0 => split_data.tower_start = split_u8,
            1 => split_data.end_enter = split_u8,
            2 => split_data.finish = Some(split_u8),
            _ => (),
        };
        idx += 1;
    }
    Ok((player_name_or_uuid.to_string(), split_data))
}
