use regex::Regex;

use crate::{cache::split::Split, Result};

pub fn extract_splits_and_name_from_role_name(role_name: &str) -> Result<(Split, u8, u8, String)> {
    let role_name = role_name.replace("*115", "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("+", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)([a-zA-Z_]+)")?;
    let caps = match re.captures(&role_name) {
        Some(caps) => caps,
        None => {
            return Err(format!(
                "ExtractError: capture regex for role name: '{}'.",
                role_name
            )
            .into())
        }
    };
    let character = match caps.get(1) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "ExtractError: get first regex capture for role name: '{}'.",
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
                "ExtractError: get second regex capture for role name: '{}'.",
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
                "ExtractError: get third regex capture for role name: '{}'.",
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
                "ExtractError: get fourth regex capture for role name: '{}'.",
                role_name
            )
            .into())
        }
    }
    .as_str()
    .to_string();
    Ok((split, minutes, seconds, name))
}
