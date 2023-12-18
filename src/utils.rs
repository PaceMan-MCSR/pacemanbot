use std::collections::HashMap;

use regex::Regex;
use serenity::{
    model::prelude::{Role, RoleId},
    prelude::Context,
};

use crate::types::{MojangResponse, Response, ResponseError};

pub async fn remove_roles_starting_with(
    ctx: &Context,
    guild_id: &serenity::model::prelude::GuildId,
    member: &mut serenity::model::prelude::Member,
    role_prefix: &str,
) {
    // Remove roles starting with role_prefix
    let guild_roles = guild_id.roles(&ctx.http).await.unwrap();
    for role_id in member.roles.clone() {
        let role = guild_roles.get(&role_id).unwrap().clone();
        if role.name.starts_with(role_prefix) {
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

pub fn sort_guildroles_based_on_split(roles: &HashMap<RoleId, Role>) -> Vec<Role> {
    let roles = roles
        .iter()
        .map(|(_, role)| role.clone())
        .filter(|role| role.name.starts_with("*"))
        .collect::<Vec<_>>();

    // Sort roles by role name, using extract_split_from_role_name to extract the split and time.
    // Sort first by split in this order : SecondStructure, Blind, EyeSpy, EndEnter
    // Then sort by minutes, then seconds
    let mut roles = roles
        .iter()
        .map(|role| {
            let (character, minutes, seconds) = extract_split_from_role_name(&role.name);
            (role, character, minutes, seconds)
        })
        .collect::<Vec<_>>();
    roles.sort_by(|a, b| {
        let (_, a_split, a_minutes, a_seconds) = a;
        let (_, b_split, b_minutes, b_seconds) = b;
        let order_a = get_split_order_number(&a_split);
        let order_b = get_split_order_number(&b_split);

        order_a
            .cmp(&order_b)
            .then_with(|| a_minutes.cmp(&b_minutes))
            .then_with(|| a_seconds.cmp(&b_seconds))
    });
    roles
        .iter()
        .map(|(role, _, _, _)| role.to_owned())
        .cloned()
        .collect::<Vec<Role>>()
}

fn get_split_order_number(split: &str) -> usize {
    match split {
        "FS" => 0,
        "SS" => 1,
        "B" => 2,
        "E" => 3,
        "EE" => 4,
        _ => 5,
    }
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

pub async fn get_username_for_uuid(uuid: &str) -> Result<String, ResponseError> {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );
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
    let res: MojangResponse = match serde_json::from_str(result.as_str()) {
        Ok(res) => res,
        Err(err) => return Err(ResponseError::new(err)),
    };
    Ok(res.name)
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
