use std::collections::HashMap;

use regex::Regex;
use serenity::{
    model::prelude::{Role, RoleId},
    prelude::Context,
};

use crate::types::Response;

pub async fn remove_roles_starting_with(
    ctx: &Context,
    guild_id: &serenity::model::prelude::GuildId,
    mut member: serenity::model::prelude::Member,
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
    let role_name = role_name.replace("PMB", "");
    let role_name = role_name.replace("Sub", "");
    let role_name = role_name.replace(" ", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)").unwrap();
    let caps = re.captures(&role_name).unwrap();
    let character = caps.get(1).unwrap().as_str().to_string();
    let minutes = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
    let seconds = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
    (character, minutes, seconds)
}

pub fn sort_guildroles_based_on_split(roles: &HashMap<RoleId, Role>) -> Vec<Role> {
    let roles = roles
        .iter()
        .map(|(_, role)| role.clone())
        .filter(|role| role.name.starts_with("PMB"))
        .collect::<Vec<_>>();

    // Sort roles by role name, using extract_split_from_role_name to extract the split and time.
    // Sort first by split in this order : FirstStructure, SecondStructure, Blind, EyeSpy
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
        .map(|(role, _, _, _)| role.clone())
        .cloned()
        .collect::<Vec<Role>>()
}

fn get_split_order_number(split: &str) -> usize {
    match split {
        "FirstStructure" => 0,
        "SecondStructure" => 1,
        "Blind" => 2,
        "EyeSpy" => 3,
        _ => 4,
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

pub async fn get_response_from_api() -> Vec<Response> {
    let url = "https://paceman.gg/api/ars/liveruns";
    let url = reqwest::Url::parse(&*url).ok().unwrap();
    let result = match match reqwest::get(url).await {
        Ok(res) => res,
        Err(err) => panic!("Error getting from url: {}", err),
    }
    .text()
    .await
    {
        Ok(text) => text,
        Err(err) => panic!("Unable to get text: {}", err),
    };
    let res: Vec<Response> = match serde_json::from_str(result.as_str()) {
        Ok(res) => res,
        Err(err) => panic!("Unable to convert to response: {}", err),
    };
    res
}
