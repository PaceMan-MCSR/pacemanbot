use std::collections::HashMap;

use regex::Regex;
use serenity::{model::prelude::GuildId, prelude::Context};

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

pub async fn update_leaderboard(
    ctx: &Context,
    guild_id: &GuildId,
    nickname: String,
    time: (u8, u8),
) -> Result<(), Box<dyn std::error::Error>> {
    let channels = match ctx.cache.guild_channels(guild_id) {
        Some(channels) => channels,
        None => return Err("Unable to get channels.".into()),
    };
    let leaderboard_channel = match channels
        .iter()
        .find(|c| c.name == "pacemanbot-runner-leaderboard")
    {
        Some(channel) => channel,
        None => return Err("No channel with name: 'pacemanbot-runner-leaderboard'.".into()),
    };
    let messages = leaderboard_channel.messages(&ctx, |m| m.limit(1)).await?;
    if messages.is_empty() {
        let leaderboard_content = format!(
            "## Runner Leaderboard\n\n{}\t\t`{}:{}`",
            nickname, time.0, time.1,
        );
        leaderboard_channel
            .send_message(&ctx.http, |m| m.content(leaderboard_content))
            .await?;
    } else {
        let first_message_id = messages.last().unwrap().id;
        let first_message = messages.last().unwrap().content.to_owned();
        let leaderboard_lines = first_message
            .split("\n")
            .filter(|l| l != &"## Runner Leaderboard" && l != &"")
            .collect::<Vec<&str>>();
        let mut player_names_with_time: HashMap<String, u64> = HashMap::new();
        for l in leaderboard_lines {
            let splits = l.split("\t\t").collect::<Vec<&str>>();
            let player_name = splits[0];
            let time = splits[1].replace("`", "");
            let time_splits = time
                .split(':')
                .map(|sp| sp.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let (minutes, seconds) = (time_splits[0], time_splits[1]);
            let time_millis: u64 = minutes as u64 * 60000 + seconds as u64 * 1000;
            player_names_with_time.insert(player_name.to_owned(), time_millis);
        }
        let current_finish_time = time.0 as u64 * 60000 + time.1 as u64 * 1000;
        if player_names_with_time.get(&nickname).is_some() {
            let time = player_names_with_time.get(&nickname).unwrap();
            if time > &current_finish_time {
                player_names_with_time.insert(nickname.to_owned(), current_finish_time);
            }
        } else {
            player_names_with_time.insert(nickname, time.0 as u64 * 60000 + time.1 as u64 * 1000);
        }
        let mut entry_vector: Vec<(&String, &u64)> = player_names_with_time
            .iter()
            .collect::<Vec<(&String, &u64)>>();
        entry_vector.sort_by(|a, b| a.1.cmp(b.1));
        let mut updated_contents: Vec<String> = vec![];
        for entry in entry_vector {
            let name = entry.0;
            let time = format_time(entry.1.to_owned());
            updated_contents.push(format!("{}\t\t`{}`", name, time));
        }
        let leaderboard_content =
            format!("## Runner Leaderboard\n\n{}", updated_contents.join("\n"));
        leaderboard_channel
            .edit_message(&ctx, first_message_id, |m| m.content(leaderboard_content))
            .await?;
    }
    Ok(())
}
