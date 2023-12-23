use regex::Regex;
use serenity::prelude::Context;

use crate::types::{Response, ResponseError};

pub async fn remove_roles_starting_with(
    ctx: &Context,
    guild_id: &serenity::model::prelude::GuildId,
    member: &mut serenity::model::prelude::Member,
    role_prefix: &str,
    skip_pb_roles: bool,
) {
    // Remove roles starting with role_prefix
    let guild_roles = guild_id.roles(&ctx.http).await.unwrap();
    for role_id in member.roles.clone() {
        let role = guild_roles.get(&role_id).unwrap().clone();
        if role.name.starts_with(role_prefix) {
            if skip_pb_roles && role.name.contains("PB") {
                continue;
            }
            member.remove_role(&ctx.http, role_id).await.unwrap();
        }
    }
}

pub fn extract_split_from_role_name(role_name: &str) -> (String, u8, u8) {
    let role_name = role_name.replace("*", "");
    let role_name = role_name.replace(" ", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)").unwrap();
    let caps = re.captures(&role_name).unwrap();
    let character = caps.get(1).unwrap().as_str().to_string();
    let minutes = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
    let seconds = caps.get(3).unwrap().as_str().parse::<u8>().unwrap() * 10;
    (character, minutes, seconds)
}

pub fn extract_split_from_pb_role_name(role_name: &str) -> String {
    let role_name = role_name.replace("*", "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("PB", "");
    role_name
}

pub fn extract_name_and_splits_from_line(
    line: &str,
) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    let line = line.trim();
    let line = line.replace(" ", "");
    let line_splits = line.split(':').collect::<Vec<&str>>();
    if line_splits.len() != 2 {
        return Err(format!("Unable to parse line contents: '{}'.", line).into());
    }
    let (player_name, splits_string) = (line_splits[0], line_splits[1]);
    let splits = splits_string.split('/').collect::<Vec<&str>>();
    if splits.len() != 5 {
        return Err(format!("Unable to parse line contents: '{}'.", line).into());
    }
    let splits = splits
        .iter()
        .map(|split| split.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    Ok((player_name.to_string(), splits))
}

pub fn get_time(milliseconds: u64) -> (u8, u8) {
    let seconds_total = milliseconds / 1000;
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    (minutes as u8, seconds as u8)
}
pub fn format_time(milliseconds: u64) -> String {
    let seconds_total = milliseconds / 1000;
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    format!("{}:{:02}", minutes, seconds)
}

pub async fn get_response_from_api() -> Result<Vec<Response>, ResponseError> {
    let url = "https://paceman.gg/api/ars/liveruns";
    let url = reqwest::Url::parse(&*url).ok().unwrap();
    let result = match match reqwest::get(url).await {
        Ok(res) => res,
        Err(err) => return Err(ResponseError::new(err)),
    }
    .text()
    .await
    {
        Ok(text) => text,
        Err(err) => return Err(ResponseError::new(err)),
    };
    let res: Vec<Response> = match serde_json::from_str(result.as_str()) {
        Ok(res) => res,
        Err(err) => return Err(ResponseError::new(err)),
    };
    Ok(res)
}

pub fn event_id_to_split(event_id: &str) -> Option<&str> {
    match event_id {
        "rsg.enter_bastion" => Some("Ba"),
        "rsg.enter_fortress" => Some("F"),
        "rsg.first_portal" => Some("B"),
        "rsg.enter_stronghold" => Some("E"),
        "rsg.enter_end" => Some("EE"),
        _ => None,
    }
}

pub fn split_to_desc(split: &str) -> Option<&str> {
    match split {
        "Ba" => Some("Enter Bastion"),
        "F" => Some("Enter Fortress"),
        "B" => Some("First Portal"),
        "E" => Some("Enter Stronghold"),
        "EE" => Some("Enter End"),
        _ => None,
    }
}
