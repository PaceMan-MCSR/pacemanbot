use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

use serenity::{
    builder::CreateEmbedAuthor, client::Context, futures::lock::Mutex, model::id::ChannelId,
    prelude::Mentionable,
};
use tokio::time::sleep;

use crate::{
    cache::{Cache, GuildCacheEntry, PlayerCacheEntry, RoleCacheEntry, EDIT_MESSAGE_DELAY},
    config::PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL,
    dispatcher::{
        format_time, millis_to_mins_secs, mins_secs_to_millis, EventType, RunInfo, RunType,
        CREDITS_EMOJI, LIVE_INDICATOR, MC_HEAD_URL_PREFIX, OFFLINE_EMOJI, OFFLINE_INDICATOR,
        SPECIAL_UNDERSCORE, STATS_URL_PREFIX, TWITCH_EMOJI, TWITCH_LINK_PREFIX,
    },
    log::Log,
    ws::{Event, ItemData, WSResponse},
};

pub struct Dispatcher {
    pub ctx: Arc<Context>,
    pub log: Arc<Log>,
    pub cache: Arc<Mutex<Cache>>,
    pub partial_author: CreateEmbedAuthor,
    pub ws_response: WSResponse,
    pub stats_link: String,
}

impl Dispatcher {
    pub fn new(
        ctx: Arc<Context>,
        log: Arc<Log>,
        cache: Arc<Mutex<Cache>>,
        ws_response: WSResponse,
    ) -> Self {
        let stats_link = format!("{}{}", STATS_URL_PREFIX, ws_response.world_id);
        let mc_head_url = format!("{}{}", MC_HEAD_URL_PREFIX, ws_response.user.uuid);
        let author_name = ws_response.nickname.replace("_", SPECIAL_UNDERSCORE);
        let mut partial_author = CreateEmbedAuthor::default();
        partial_author.icon_url(mc_head_url);
        partial_author.name(author_name);
        Self {
            ctx,
            log,
            cache,
            ws_response,
            stats_link,
            partial_author,
        }
    }
    pub async fn dispatch(&self) -> Result<(), Box<dyn Error>> {
        let game_version = self.ws_response.game_version.to_owned();
        if game_version.is_some() && game_version.unwrap() != "1.16.1" {
            self.log
                .warn("Skipping record because it was not of 1.16.1.");
            return Ok(());
        }
        let last_event = match self.ws_response.event_list.last() {
            Some(evt) => evt,
            None => {
                return Err(format!(
                    "failed to get last event from events list of size: {}",
                    self.ws_response.event_list.len()
                )
                .into())
            }
        };
        let mut locked_cache = self.cache.lock().await;
        for (guild_id, guild_cache_entry) in locked_cache.entries.iter_mut() {
            let live_link = match self.ws_response.user.live_account.to_owned() {
                Some(live_account) => format!("{}{}", TWITCH_LINK_PREFIX, live_account),
                None => {
                    if !match GuildCacheEntry::is_private(
                        guild_cache_entry.name.to_string(),
                        self.ctx.clone(),
                        guild_id,
                    ) {
                        Ok(is_private) => is_private,
                        Err(err) => {
                            self.log.warn(err.to_string().as_str());
                            continue;
                        }
                    } {
                        self.log.warn(
                            format!(
                                "Skipping guild: '{}' because user with name: '{}' is not live.",
                                guild_cache_entry.name, self.ws_response.nickname,
                            )
                            .as_str(),
                        );
                        continue;
                    }
                    "".to_string()
                }
            };
            let mut author = self.partial_author.clone();
            if live_link.is_empty() {
                author.url(live_link.clone());
            }

            let event_type = EventType::from(last_event);
            let is_private = match GuildCacheEntry::is_private(
                guild_cache_entry.name.to_string(),
                self.ctx.clone(),
                guild_id,
            ) {
                Ok(is_private) => is_private,
                Err(err) => return Err(err.into()),
            };
            let has_player_ign = guild_cache_entry
                .player_whitelist
                .iter()
                .any(|p| p.0 == &self.ws_response.nickname.to_lowercase());
            let has_player_uuid = guild_cache_entry
                .player_whitelist
                .iter()
                .any(|p| p.0 == &self.ws_response.user.uuid);
            if !has_player_ign && !has_player_uuid {
                if is_private {
                    self.log.warn(format!(
                        "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                        self.ws_response.nickname,
                        guild_cache_entry.name
                    ).as_str());
                    continue;
                }
                let player_data = PlayerCacheEntry::default();
                guild_cache_entry.player_whitelist.insert(
                    self.ws_response.nickname.to_owned().to_lowercase(),
                    player_data,
                );
            }
            match event_type {
                EventType::Unknown => {
                    self.log.warn(
                        format!(
                            "Unknown event type: {:#?}. Skipping all guilds.",
                            last_event.event_id
                        )
                        .as_str(),
                    );
                    return Ok(());
                }
                EventType::NonPaceEvent => {
                    match self
                        .handle_non_pace_event(
                            live_link,
                            author,
                            last_event,
                            guild_cache_entry,
                            is_private,
                            has_player_uuid,
                        )
                        .await
                    {
                        Ok(_) => (),
                        Err(err) => self.log.warn(err.to_string().as_str()),
                    };
                }
                EventType::PaceEvent => {
                    match self
                        .handle_pace_event(
                            live_link,
                            author,
                            last_event,
                            guild_cache_entry,
                            is_private,
                            has_player_uuid,
                        )
                        .await
                    {
                        Ok(_) => (),
                        Err(err) => self.log.warn(err.to_string().as_str()),
                    };
                }
            }
        }
        Ok(())
    }

    pub async fn handle_pace_event(
        &self,
        live_link: String,
        author: CreateEmbedAuthor,
        last_event: &Event,
        guild_cache_entry: &mut GuildCacheEntry,
        is_private: bool,
        has_player_uuid: bool,
    ) -> Result<(), Box<dyn Error>> {
        let event_list: Vec<Event> = self.ws_response.event_list.iter().cloned().collect();
        let context_event_list: Vec<Event> = self
            .ws_response
            .context_event_list
            .iter()
            .cloned()
            .collect();
        let item_data = self.ws_response.item_data.clone();
        let run_info = match RunInfo::from_last_event(last_event, event_list, context_event_list) {
            Some(info) => info,
            None => {
                return Err(format!("unrecognized event id: {:#?}.", last_event.event_id).into());
            }
        };
        let player_data = if has_player_uuid {
            guild_cache_entry
                .player_whitelist
                .get_mut(&self.ws_response.user.uuid)
                .unwrap()
        } else {
            guild_cache_entry
                .player_whitelist
                .get_mut(&self.ws_response.nickname.to_lowercase())
                .unwrap()
        };
        let split_desc = match run_info.split.desc(&run_info.structure) {
            Some(desc) => desc,
            None => {
                return Err(
                    format!("failed to get split desc for split: {:#?}", run_info.split).into(),
                );
            }
        };
        let split_emoji = match run_info.split.get_emoji(&run_info.structure) {
            Some(emoji) => emoji,
            None => {
                return Err(
                    format!("failed to get split emoji for split: {:#?}", run_info.split).into(),
                );
            }
        };
        let roles_to_ping = guild_cache_entry
            .roles
            .iter()
            .filter(|role_cache_entry| {
                role_cache_entry.is_pingable(
                    player_data,
                    &run_info,
                    last_event,
                    is_private,
                    &self.ws_response,
                )
            })
            .collect::<Vec<_>>();
        if roles_to_ping.is_empty() {
            self.log.warn(
                format!(
                    "Skipping split: '{}' because there are no roles to ping in guild name: {}.",
                    split_desc, guild_cache_entry.name
                )
                .as_str(),
            );
            return Ok(());
        }
        let live_indicator = if self.ws_response.user.live_account.is_some() {
            LIVE_INDICATOR
        } else {
            OFFLINE_INDICATOR
        };
        let items_msg = ItemData::to_formatted_message(item_data, &run_info);
        let metadata = format!(
            "{} {} - {} {}",
            live_indicator,
            format_time(last_event.igt as u64),
            split_desc,
            self.ws_response.nickname.replace("_", SPECIAL_UNDERSCORE)
        );
        let message_content = format!(
            "{}\n-# {}",
            metadata.clone(),
            roles_to_ping
                .iter()
                .map(|role_cache_entry| role_cache_entry.role.mention().to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
        let pace_msg = format!(
            "{}  {} - {}",
            split_emoji,
            format_time(last_event.igt as u64),
            split_desc,
        );
        match self
            .send_message_in_pace_channel(
                &guild_cache_entry.pace_channel,
                guild_cache_entry.name.to_string(),
                author,
                pace_msg,
                items_msg,
                live_link,
                Some(run_info),
                message_content,
                metadata,
                roles_to_ping,
                split_desc.to_string(),
                true,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                self.log.error(
                    format!("failed to send split: '{}' due to: {}", split_desc, err).as_str(),
                );
                return Err(err.into());
            }
        }
    }

    pub async fn handle_non_pace_event(
        &self,
        live_link: String,
        author: CreateEmbedAuthor,
        last_event: &Event,
        guild_cache_entry: &mut GuildCacheEntry,
        is_private: bool,
        has_player_uuid: bool,
    ) -> Result<(), Box<dyn Error>> {
        let player_data = if has_player_uuid {
            guild_cache_entry
                .player_whitelist
                .get_mut(&self.ws_response.user.uuid)
                .unwrap()
        } else {
            guild_cache_entry
                .player_whitelist
                .get_mut(&self.ws_response.nickname.to_lowercase())
                .unwrap()
        };

        let runner_name = self.ws_response.nickname.to_owned();
        let (minutes, seconds) = millis_to_mins_secs(last_event.igt as u64);
        let finish_minutes = match player_data.finish {
            Some(mins) => mins,
            None => {
                if !is_private && minutes >= 10 {
                    self.log.warn(format!(
                        "Skipping guild name: {} because it is not a sub 10 completion and the guild is public.", 
                        guild_cache_entry.name
                    ).as_str());
                    return Ok(());
                }
                // `minutes` + 1 will always be greater than minutes.
                // This is done to send finish message always if finish time is not defined.
                minutes + 1
            }
        };
        if minutes >= finish_minutes {
            self.log.warn(
                format!(
                    "Skipping guild name: {} because finish time is above the defined amount.",
                    guild_cache_entry.name,
                )
                .as_str(),
            );
            return Ok(());
        }

        let finish_msg = format!(
            "{}  {} - Finish",
            CREDITS_EMOJI,
            format_time(last_event.igt as u64),
        );

        let mut items_msg = String::new();
        ItemData::format_item_count(&mut items_msg, "0", "0".to_string());

        match self
            .send_message_in_pace_channel(
                &guild_cache_entry.pace_channel,
                guild_cache_entry.name.to_string(),
                author,
                finish_msg,
                items_msg,
                live_link,
                None,
                String::new(),
                String::new(),
                Vec::new(),
                "Finish".to_string(),
                false,
            )
            .await
        {
            Ok(_) => (),
            Err(err) => {
                self.log
                    .error(format!("failed to send split: 'Finish' due to: {}", err).as_str());
                return Err(err.into());
            }
        };

        if !is_private || guild_cache_entry.lb_channel.is_none() {
            self.log.warn(format!(
                "Can't handle non pace event for guild name: {} because it is either a public server or does not have a leaderboard channel.", 
                guild_cache_entry.name
            ).as_str());
            return Ok(());
        }

        match self.update_leaderboard(
            guild_cache_entry.lb_channel.unwrap(),
            runner_name.to_owned().replace("_", SPECIAL_UNDERSCORE),
            (minutes, seconds),
        )
        .await
        {
            Ok(_) => {
                self.log.info(format!(
                    "Updated leaderboard in #{} for guild name: {}, runner name: {} with time: {}.",
                    PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL,
                    guild_cache_entry.name,
                    runner_name,
                    format_time(last_event.igt as u64),
                ).as_str());
                Ok(())
            },
            Err(err) => {
                Err(format!(
                    "HandleNonPaceEvent: update leaderboard in guild name: {} for runner name: {} due to: {}",
                    guild_cache_entry.name,
                    self.ws_response.nickname.to_owned(),
                    err
                ).into())
            }
        }
    }

    pub async fn update_leaderboard(
        &self,
        leaderboard_channel: ChannelId,
        nickname: String,
        time: (u8, u8),
    ) -> Result<(), Box<dyn Error>> {
        let messages = leaderboard_channel
            .messages(&self.ctx, |m| m.limit(100))
            .await?;
        if messages.is_empty() {
            let leaderboard_content = format!(
                "## Runner Leaderboard\n\n`{}`\t\t{}",
                format_time(mins_secs_to_millis(time)),
                nickname
            );
            leaderboard_channel
                .send_message(&self.ctx.http, |m| m.content(leaderboard_content))
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
                    return Err("failed to parse leaderboard message.".into());
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
                .edit_message(&self.ctx, first_message_id, |m| {
                    m.content(leaderboard_content)
                })
                .await?;
        }
        Ok(())
    }

    pub async fn send_message_in_pace_channel(
        &self,
        pace_channel: &ChannelId,
        guild_name: String,
        author: CreateEmbedAuthor,
        pace_msg: String,
        items_msg: String,
        live_link: String,
        run_info: Option<RunInfo>,
        message_content: String,
        metadata: String,
        roles_to_ping: Vec<&RoleCacheEntry>,
        split_desc: String,
        is_pace_event: bool,
    ) -> Result<(), Box<dyn Error>> {
        let run_info = run_info.unwrap_or(RunInfo::default());
        match pace_channel
            .send_message(&self.ctx.clone(), |m| {
                m.embed(|e| {
                    e.set_author(author.clone());
                    e.field(pace_msg.clone(), "", false);
                    if live_link.is_empty() {
                        e.field(format!("{} {}", TWITCH_EMOJI, live_link.clone()), "", false);
                    } else {
                        e.field(format!("{}  Offline", OFFLINE_EMOJI), "", false);
                    }
                    e.field("Splits", format!("[Link]({})", self.stats_link), true);
                    e.field(
                        "Time",
                        format!("<t:{}:R>", (self.ws_response.last_updated / 1000) as u64),
                        true,
                    );
                    e.field("Items", items_msg.clone(), true);
                    if is_pace_event && RunType::Bastionless == run_info.run_type {
                        e.field("Bastionless", "Yes", true);
                    }
                    e
                })
                .content(message_content.to_owned())
            })
            .await
        {
            Ok(mut message) => {
                if is_pace_event {
                    let ctx_clone = self.ctx.clone();
                    let ping_content_clone = message_content.to_owned();
                    let metadata_clone = metadata.clone();
                    let roles_to_ping_clone = roles_to_ping
                        .into_iter()
                        .map(|r| (r.runner.clone(), r.role.mention().to_string()))
                        .collect::<Vec<_>>();
                    let author_clone = author.clone();
                    let pace_content_clone = pace_msg.clone();
                    let live_link_clone = live_link.clone();
                    let stats_link_clone = self.stats_link.clone();
                    let item_data_content_clone = items_msg.clone();
                    let live_account_clone = self.ws_response.user.live_account.is_some();
                    let last_updated = self.ws_response.last_updated;
                    let run_type_clone = run_info.run_type.clone();
                    let log_clone = self.log.clone();

                    tokio::spawn(async move {
                        sleep(Duration::from_secs(EDIT_MESSAGE_DELAY)).await;
                        let removable_roles = roles_to_ping_clone
                            .iter()
                            .filter(|(runner, _)| runner.as_str() != "")
                            .map(|(_, mention)| mention)
                            .collect::<Vec<_>>();
                        let mut new_content = ping_content_clone;
                        for role in removable_roles {
                            let replaceable_str = format!("{} ", role);
                            new_content = new_content.replace(replaceable_str.as_str(), "");
                        }
                        let content_removed_metadata =
                            new_content.replace(format!("{}\n", metadata_clone).as_str(), "");
                        match message
                            .edit(&ctx_clone.http, |m| {
                                m.embed(|e| {
                                    e.set_author(author_clone);
                                    e.field(pace_content_clone, "", false);
                                    if live_account_clone {
                                        e.field(
                                            format!("{} {}", TWITCH_EMOJI, live_link_clone),
                                            "",
                                            false,
                                        );
                                    } else {
                                        e.field(format!("{}  Offline", OFFLINE_EMOJI), "", false);
                                    }
                                    e.field(
                                        "Splits",
                                        format!("[Link]({})", stats_link_clone),
                                        true,
                                    );
                                    e.field(
                                        "Time",
                                        format!("<t:{}:R>", (last_updated / 1000) as u64),
                                        true,
                                    );
                                    e.field("Items", item_data_content_clone, true);
                                    if let RunType::Bastionless = run_type_clone {
                                        e.field("Bastionless", "Yes", true);
                                    }
                                    e
                                })
                                .content(content_removed_metadata)
                            })
                            .await
                        {
                            Ok(_) => (),
                            Err(err) => {
                                log_clone.error(
                                    format!("failed to edit message due to: {}", err).as_str(),
                                );
                            }
                        };
                    });
                }
                self.log.info(
                    format!(
                    "Sent pace-ping for user with name: '{}' for split: '{}' in guild name: {}.",
                    self.ws_response.nickname, split_desc, guild_name,
                )
                    .as_str(),
                );
                Ok(())
            }
            Err(err) => {
                Err(format!("failed to send split: '{}' due to: {}", split_desc, err).into())
            }
        }
    }
}
