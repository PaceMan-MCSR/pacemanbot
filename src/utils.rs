use std::collections::HashMap;

use regex::Regex;
use serenity::{
    builder::{CreateSelectMenuOption, CreateSelectMenuOptions},
    model::{
        id::{ChannelId, GuildId, RoleId},
        prelude::Role,
    },
    prelude::Context,
};
use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{handshake::client::generate_key, http::request},
    MaybeTlsStream, WebSocketStream,
};

use crate::{
    consts::{
        ROLE_COLOR, WS_CONNECTION_HEADER, WS_FALLBACK_HOST, WS_FALLBACK_URL, WS_SEC_VERSION_HEADER,
        WS_UPGRADE_HEADER,
    },
    guild_types::{PlayerSplitsData, Players, Split},
    response_types::{Event, EventId, EventType},
};

pub async fn remove_roles_starting_with(
    ctx: &Context,
    guild_id: &serenity::model::prelude::GuildId,
    member: &mut serenity::model::prelude::Member,
    role_prefix: &str,
    skip_pb_roles: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let guild_roles = guild_id.roles(&ctx.http).await?;
    for role_id in member.roles.clone() {
        let role = guild_roles.get(&role_id).unwrap().clone();
        if role.name.starts_with(role_prefix) {
            if skip_pb_roles && role.name.contains("PB") {
                continue;
            }
            member.remove_role(&ctx.http, role_id).await?;
        }
    }
    Ok(())
}

pub fn extract_split_from_role_name(
    role_name: &str,
) -> Result<(Split, u8, u8), Box<dyn std::error::Error>> {
    let role_name = role_name.replace("*", "");
    let role_name = role_name.replace(" ", "");
    let re = Regex::new(r"([a-zA-Z]+)(\d+)\:(\d+)")?;
    let caps = match re.captures(&role_name) {
        Some(caps) => caps,
        None => {
            return Err(format!("Unable to capture regex for role name: '{}'.", role_name).into())
        }
    };
    let character = match caps.get(1) {
        Some(capture) => capture,
        None => {
            return Err(format!(
                "Unable to get first regex capture for role name: '{}'.",
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
                "Unable to get second regex capture for role name: '{}'.",
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
                "Unable to get third regex capture for role name: '{}'.",
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
    let role_name = role_name.replace("*", "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("PB", "");
    Split::from_str(role_name.as_str())
}

pub fn extract_name_and_splits_from_line(
    line: &str,
) -> Result<(String, PlayerSplitsData), Box<dyn std::error::Error>> {
    let line = line.trim();
    let line = line.replace(" ", "");
    let line_splits = line.split(':').collect::<Vec<&str>>();
    if line_splits.len() != 2 {
        return Err(format!("Unable to parse line contents: '{}'.", line).into());
    }
    let (player_name, splits_string) = (line_splits[0], line_splits[1]);
    let splits = splits_string.split('/').collect::<Vec<&str>>();
    if splits.len() != 5 && splits.len() != 6 {
        return Err(format!("Unable to parse line contents: '{}'.", line).into());
    }
    let mut idx = 0;
    let mut split_data = PlayerSplitsData::default();
    for split in splits {
        let split_u8 = match split.parse::<u8>() {
            Ok(split) => split,
            Err(err) => {
                return Err(format!("Unable to parse to u8 due to: {}", err).into());
            }
        };
        match idx {
            0 => split_data.first_structure = split_u8,
            1 => split_data.second_structure = split_u8,
            2 => split_data.blind = split_u8,
            3 => split_data.eye_spy = split_u8,
            4 => split_data.end_enter = split_u8,
            5 => split_data.finish = Some(split_u8),
            _ => (),
        };
        idx += 1;
    }
    Ok((player_name.to_string(), split_data))
}

pub fn millis_to_mins_secs(milliseconds: u64) -> (u8, u8) {
    let seconds_total = milliseconds / 1000;
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    (minutes as u8, seconds as u8)
}

pub fn mins_secs_to_millis(time: (u8, u8)) -> u64 {
    let (minutes, seconds) = (time.0 as u64, time.1 as u64);
    minutes * 60000 + seconds * 1000
}

pub fn format_time(milliseconds: u64) -> String {
    let seconds_total = milliseconds / 1000;
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    format!("{}:{:02}", minutes, seconds)
}

pub async fn get_response_stream_from_api(
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
    let url = match env::var("WS_URL") {
        Ok(url) => url,
        Err(err) => {
            eprintln!("{}", err);
            WS_FALLBACK_URL.to_string()
        }
    };
    let host = match env::var("WS_HOST") {
        Ok(host) => host,
        Err(err) => {
            eprintln!("{}", err);
            WS_FALLBACK_HOST.to_string()
        }
    };
    let auth_key: String = match env::var("API_AUTH_KEY") {
        Ok(key) => key,
        Err(err) => return Err(format!("API_AUTH_KEY not found in env: {}", err)),
    };
    let request = request::Request::builder()
        .uri(url)
        .header("auth", auth_key.to_owned())
        .header("sec-websocket-key", generate_key())
        .header("host", host)
        .header("upgrade", WS_UPGRADE_HEADER)
        .header("connection", WS_CONNECTION_HEADER)
        .header("sec-websocket-version", WS_SEC_VERSION_HEADER)
        .body(())
        .unwrap();
    let (response_stream, _) = match tokio_tungstenite::connect_async(request).await {
        Ok(stream_tuple) => stream_tuple,
        Err(err) => return Err(format!("WS Connection error: {}", err)),
    };
    Ok(response_stream)
}

pub async fn update_leaderboard(
    ctx: &Context,
    leaderboard_channel: ChannelId,
    nickname: String,
    time: (u8, u8),
) -> Result<(), Box<dyn std::error::Error>> {
    let messages = leaderboard_channel.messages(&ctx, |m| m.limit(100)).await?;
    if messages.is_empty() {
        let leaderboard_content = format!(
            "## Runner Leaderboard\n\n`{}`\t\t{}",
            format_time(mins_secs_to_millis(time)),
            nickname
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
            if splits.len() != 2 {
                return Err("Unable to parse leaderboard message.".into());
            }
            let player_name = splits[1];
            let time = splits[0].replace("`", "");
            let time_splits = time
                .split(':')
                .map(|sp| sp.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let (minutes, seconds) = (time_splits[0], time_splits[1]);
            let time_millis: u64 = mins_secs_to_millis((minutes, seconds));
            player_names_with_time.insert(player_name.to_owned(), time_millis);
        }
        let current_finish_time = mins_secs_to_millis(time);
        if player_names_with_time.get(&nickname).is_some() {
            let time = player_names_with_time.get(&nickname).unwrap();
            if time > &current_finish_time {
                player_names_with_time.insert(nickname.to_owned(), current_finish_time);
            }
        } else {
            player_names_with_time.insert(nickname, mins_secs_to_millis(time));
        }
        let mut entry_vector: Vec<(&String, &u64)> = player_names_with_time
            .iter()
            .collect::<Vec<(&String, &u64)>>();
        entry_vector.sort_by(|a, b| a.1.cmp(b.1));
        let mut updated_contents: Vec<String> = vec![];
        for entry in entry_vector {
            let name = entry.0;
            let time = format_time(entry.1.to_owned());
            updated_contents.push(format!("`{}`\t\t{}", time, name));
        }
        let leaderboard_content =
            format!("## Runner Leaderboard\n\n{}", updated_contents.join("\n"));
        leaderboard_channel
            .edit_message(&ctx, first_message_id, |m| m.content(leaderboard_content))
            .await?;
    }
    Ok(())
}

pub fn get_new_config_contents(players: Players) -> String {
    let mut new_config = String::new();
    for (name, splits) in players {
        let finish_config = if splits.finish.is_some() {
            format!("/{}", splits.finish.unwrap())
        } else {
            "".to_string()
        };
        let line = format!(
            "{}:{}/{}/{}/{}/{}{}",
            name,
            splits.first_structure,
            splits.second_structure,
            splits.blind,
            splits.eye_spy,
            splits.end_enter,
            finish_config
        );
        new_config = format!("{}\n{}", new_config, line);
    }
    new_config
}

pub fn create_select_option<'a>(
    o: &'a mut CreateSelectMenuOptions,
    roles: &Vec<&Role>,
    target_split: Split,
) -> Result<&'a mut CreateSelectMenuOptions, Box<dyn std::error::Error>> {
    for role in roles {
        if role.name.contains("PB") {
            let split = match extract_split_from_pb_role_name(&role.name) {
                Some(split) => split,
                None => {
                    return Err(
                        format!("Unable to extract split from pb role name: {}", role.name).into(),
                    )
                }
            };
            if split == target_split {
                o.add_option(
                    CreateSelectMenuOption::default()
                        .label(format!("PB Pace {}", target_split.alt_desc()))
                        .value(role.id.to_string())
                        .to_owned(),
                );
            }
        } else {
            let (split, minutes, seconds) = extract_split_from_role_name(&role.name)?;
            if split == target_split {
                o.add_option(
                    CreateSelectMenuOption::default()
                        .label(format!(
                            "Sub {}:{:02} {}",
                            minutes,
                            seconds,
                            target_split.alt_desc()
                        ))
                        .value(role.id.to_string())
                        .to_owned(),
                );
            }
        }
    }
    Ok(o)
}

pub fn get_event_type(last_event: &Event) -> Option<EventType> {
    match last_event.event_id {
        EventId::RsgEnterBastion
        | EventId::RsgEnterFortress
        | EventId::RsgFirstPortal
        | EventId::RsgEnterStronghold
        | EventId::RsgEnterEnd => Some(EventType::PaceEvent),
        EventId::RsgCredits => Some(EventType::NonPaceEvent),
        _ => None,
    }
}

pub async fn create_guild_role(
    ctx: &Context,
    guild: &GuildId,
    roles: &HashMap<RoleId, Role>,
    role_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !roles
        .iter()
        .any(|(_, role)| role.name == role_name.to_string())
    {
        guild
            .create_role(ctx, |r| r.name(role_name).colour(ROLE_COLOR.into()))
            .await?;
    }
    Ok(())
}
