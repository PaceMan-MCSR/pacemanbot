use std::sync::Arc;

use serenity::{client::Context, model::mention::Mentionable};

use crate::{
    guild_types::{CachedGuilds, GuildData, Split, PlayerData},
    response_types::{Event, EventId, EventType, Response, RunInfo, Structure, RunType},
    ArcMux, utils::{get_time, update_leaderboard, format_time},
};

const SPECIAL_UNDERSCORE: &'static str = "ˍ";

pub struct Controller {
    pub ctx: Arc<Context>,
    pub record: Response,
    pub guild_cache: ArcMux<CachedGuilds>,
}

impl Controller {
    pub fn new(ctx: Arc<Context>, record: Response, guild_cache: ArcMux<CachedGuilds>) -> Self {
        Self {
            ctx,
            record,
            guild_cache,
        }
    }

    fn get_run_info(&self, last_event: &Event) -> Option<RunInfo> {
        match last_event.event_id {
            EventId::RsgEnterBastion => {
                let mut split = Split::FirstStructure;
                if self
                    .record
                    .context_event_list
                    .iter()
                    .any(|ctx| ctx.event_id == EventId::RsgObtainBlazeRod)
                {
                    split = Split::SecondStructure;
                }
                Some(RunInfo {
                    split,
                    structure: Some(Structure::Bastion),
                    run_type: RunType::Modern,
                })
            }
            EventId::RsgEnterFortress => {
                let mut split = Split::FirstStructure;
                let fort_ss_check = self
                    .record
                    .event_list
                    .iter()
                    .filter(|evt| evt != &last_event)
                    .any(|evt| evt.event_id == EventId::RsgEnterBastion);

                let mut fort_ss_context_check = false;
                let mut context_hits = 0;
                for ctx in self.record.context_event_list.iter() {
                    let context_check = ctx.event_id == EventId::RsgObtainCryingObsidian
                        || ctx.event_id == EventId::RsgObtainObsidian
                        || ctx.event_id == EventId::RsgLootBastion;
                    if context_check {
                        context_hits += 1;
                    }
                }
                if context_hits >= 2 {
                    fort_ss_context_check = true;
                }

                if fort_ss_check && fort_ss_context_check {
                    split = Split::SecondStructure;
                }
                Some(RunInfo {
                    split,
                    structure: Some(Structure::Fortress),
                    run_type: RunType::Modern,
                })
            }
            EventId::RsgFirstPortal => {
                let mut run_type = RunType::Modern;
                if self
                    .record
                    .event_list
                    .iter()
                    .all(|evt| evt.event_id != EventId::RsgEnterBastion)
                {
                    run_type = RunType::Bastionless;
                }
                Some(RunInfo {
                    split: Split::Blind,
                    structure: None,
                    run_type,
                })
            }
            _ => {
                let split = Split::from_event_id(&last_event.event_id)?;
                Some(RunInfo {
                    split,
                    structure: None,
                    run_type: RunType::Modern,
                })
            }
        }
    }

    fn get_event_type(&self, last_event: &Event) -> Option<EventType> {
        match last_event.event_id {
            EventId::CommonEnableCheats
            | EventId::CommonMultiplayer
            | EventId::CommonLeaveWorld
            | EventId::CommonOpenToLan
            | EventId::CommonViewSeed => Some(EventType::CommonEvent),
            EventId::RsgEnterBastion
            | EventId::RsgEnterFortress
            | EventId::RsgFirstPortal
            | EventId::RsgEnterStronghold
            | EventId::RsgEnterEnd => Some(EventType::PaceEvent),
            EventId::RsgCredits => Some(EventType::NonPaceEvent),
            _ => None,
        }
    }

    async fn update_cache(&self) {
        for guild_id in self.ctx.clone().cache.guilds() {
            let mut locked_guild_cache = self.guild_cache.lock().await;
            if let Some(cache) = locked_guild_cache.get(&guild_id) {
                let mut guild_data = match GuildData::new(&self.ctx, guild_id).await {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("{}", err);
                        locked_guild_cache.remove(&guild_id);
                        continue;
                    }
                };
                if !guild_data.is_private {
                    guild_data.players = cache.players.clone();
                }
                locked_guild_cache.insert(guild_id, guild_data);
            } else {
                let guild_data = match GuildData::new(&self.ctx, guild_id).await {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
                locked_guild_cache.insert(guild_id, guild_data);
            }
        }
    }


    async fn handle_common_event(&self, guild_data: &mut GuildData) {
        let player_data = match guild_data.players.get_mut(&self.record.nickname) {
            Some(data) => data,
            None => {
                if guild_data.is_private {
                    return println!(
                        "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                        self.record.nickname, 
                        guild_data.name
                    );
                }
                let player_data = PlayerData::default();
                guild_data.players.insert(self.record.nickname.to_owned(), player_data);
                guild_data.players.get_mut(&self.record.nickname).unwrap()
            }
        };
        let message_id = match player_data.last_pace_message {
            Some(id) => id,
            None => {
                return eprintln!("No last pace message to edit for reset.");
            }
        };
        let mut message = match self.ctx.cache.message(guild_data.pace_channel, message_id)
        {
            Some(message) => message,
            None => {
                return eprintln!(
                    "Unable to construct message from message id: {}",
                    message_id
                );
            }
        };
        let prev_content = message.content.split('\n').collect::<Vec<&str>>();
        let first_line = match prev_content.first() {
            Some(line) => line,
            None => {
                return eprintln!(
                    "Unable to get first line from message with id: {}",
                    message_id
                );
            }
        };
        let new_first_line = format!("{} (Reset)", first_line);
        let other_lines = prev_content
            .iter()
            .filter(|l| l.to_owned() != first_line)
            .map(|l| l.to_owned())
            .collect::<Vec<&str>>()
            .join("\n");
        let new_content = format!("{}\n{}", new_first_line, other_lines);
        match message
            .edit(&self.ctx.http, |m| m.content(new_content))
            .await
        {
            Ok(_) => (),
            Err(err) => eprintln!(
                "Unable to edit message with id: {} due to: {}",
                message_id, err
            ),
        };
    }

    async fn handle_non_pace_event(&self, last_event: &Event, guild_data: &mut GuildData) {
        let runner_name = self.record.nickname.to_owned();
        let (minutes, seconds) = get_time(last_event.igt as u64);
        if !guild_data.is_private {
            return;
        }
        match update_leaderboard(&self.ctx, guild_data.lb_channel.unwrap(), runner_name.to_owned(), (minutes, seconds))
            .await
        {
            Ok(_) => {
                println!(
                    "Updated leaderboard in #pacemanbot-runner-leaderboard for guild name: {}, runner name: {} with time: {}.", 
                    guild_data.name, 
                    runner_name, 
                    format_time(last_event.igt as u64),
                );
            }
            Err(err) => {
                eprintln!(
                    "Unable to update leaderboard in guild name: {} for runner name: {} due to: {}",
                    guild_data.name,
                    self.record.nickname.to_owned(), 
                    err
                );
            }
        };
    }

    async fn handle_pace_event(&self, last_event: &Event, guild_data: &mut GuildData) {
        let run_info = match self.get_run_info(last_event) {
                        Some(info) => info,
                        None => {
                            return eprintln!("Unrecognized event id: {:#?}, skipping all guilds.", last_event.event_id);
                        }
                    };

        let player_data = match guild_data.players.get_mut(&self.record.nickname) {
            Some(data) => data,
            None => {
                if guild_data.is_private {
                    return println!(
                        "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                        self.record.nickname, 
                        guild_data.name
                    );
                }
                let player_data = PlayerData::default();
                guild_data.players.insert(self.record.nickname.to_owned(), player_data);
                guild_data.players.get_mut(&self.record.nickname).unwrap()
            }
        };
        let split_desc = match run_info.split.desc(run_info.structure) {
            Some(desc) => desc,
            None => {
                return eprintln!("Unable to get split desc for split: {:#?}", run_info.split);
            }
        };

        let player_splits = player_data.splits;

        let bastionless = if let RunType::Bastionless = run_info.run_type {
            "(Bastionless)"
        } else {
            ""
        };

        let roles_to_ping = guild_data.roles
            .iter()
            .filter(|role| {
                let (split_minutes, split_seconds) = get_time(last_event.igt as u64);
                if role.guild_role.name.contains("PB") {
                    if !guild_data.is_private {
                        return false;
                    }
                    let pb_minutes = player_splits.get(&role.split).unwrap().to_owned();
                    role.split == run_info.split && pb_minutes > split_minutes
                } else {
                    role.split == run_info.split
                        && role.minutes >= split_minutes
                        && (role.minutes != split_minutes || role.seconds > split_seconds)
                }
            })
            .collect::<Vec<_>>();

        if roles_to_ping.is_empty() {
            return println!(
                "Skipping split: '{}' because there are no roles to ping in guild name: {}.",
                split_desc, guild_data.name 
            );
        }

        let live_link = match self.record.user.live_account.to_owned() {
            Some(acc) => format!("[{}](<https://twitch.tv/{}>)", self.record.nickname.replace("_", SPECIAL_UNDERSCORE), acc),
            None => {
                if !guild_data.is_private {
                    return println!(
                        "Skipping split: '{}' because user with name: '{}' is not live.",
                        split_desc, self.record.nickname,
                    );
                } else {
                    format!("Offline - {}", self.record.nickname.replace("_", SPECIAL_UNDERSCORE))
                }
            }
        };


        let content = format!(
            "## {} - {} {}\n{}\t<t:{}:R>\n{}",
            format_time(last_event.igt as u64),
            split_desc,
            bastionless,
            live_link,
            (self.record.last_updated / 1000) as u64,
            roles_to_ping
                .iter()
                .map(|role| role.guild_role.mention().to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );

        player_data.last_pace_message = match guild_data.pace_channel
            .send_message(&self.ctx, |m| m.content(content))
            .await
        {
            Ok(message) => {
                println!(
                    "Sent pace-ping for user with name: '{}' for split: '{}' in guild name: {}.",
                    self.record.nickname, split_desc, guild_data.name 
                );
                player_data.last_split = Some(run_info.split);
                Some(message.id)
            }
            Err(err) => {
                return eprintln!(
                    "Unable to send split: '{}' with roles: {:?} due to: {}",
                    split_desc, roles_to_ping, err
                );
            }
        };
    }

    pub async fn start(&self) {
        self.update_cache().await;
        let last_event = match self.record.event_list.last() {
            Some(evt) => evt,
            None => {
                return eprintln!(
                    "Unable to get last event from events list for record: {:#?}",
                    self.record
                )
            }
        };
        let mut locked_guild_cache = self.guild_cache.lock().await;
        for (_, guild_data) in locked_guild_cache.iter_mut() {
            let event_type = match self.get_event_type(&last_event) {
                Some(etype) => etype,
                None => {
                    eprintln!(
                        "Unable to get event type for event: {:#?}",
                        last_event.event_id
                    );
                    continue;
                }
            };
            match event_type {
                EventType::CommonEvent  => {
                    self.handle_common_event(guild_data).await;
                }
                EventType::NonPaceEvent => {
                    self.handle_non_pace_event(last_event, guild_data).await;
                }
                EventType::PaceEvent => {
                    self.handle_pace_event(last_event, guild_data).await;
                }
            }
        }
    }
}