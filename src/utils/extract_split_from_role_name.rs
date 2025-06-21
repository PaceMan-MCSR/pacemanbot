use regex::Regex;

use crate::{
    cache::{consts::ROLE_PREFIX, split::Split},
    Result,
};

pub fn extract_split_from_role_name(role_name: &str) -> Result<(Split, u8, u8)> {
    let role_name = role_name.replace(ROLE_PREFIX, "");
    let role_name = role_name.replace(" ", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)")?;
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
    let hours = match caps.get(2) {
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
    let minutes = match caps.get(3) {
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
    let split = Split::from_str(character.as_str()).unwrap();
    Ok((split, hours, minutes))
}
