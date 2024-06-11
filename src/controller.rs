use std::sync::Arc;

use serenity::{client::Context, model::mention::Mentionable};

use crate::{
    consts::{PEARL_EMOJI, ROD_EMOJI, SPECIAL_UNDERSCORE}, guild_types::{CachedGuilds, GuildData, PlayerSplitsData, Split}, response_types::{Event, EventId, EventType, Item, Response, RunInfo, RunType, Structure}, utils::{format_time, get_event_type, millis_to_mins_secs, update_leaderboard}, ArcMux
};


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
                let bastion_ss_check = self
                    .record
                    .event_list
                    .iter()
                    .any(|ctx| ctx.event_id == EventId::RsgEnterFortress);
                let bastion_ss_context_check = self
                    .record
                    .context_event_list
                    .iter()
                    .any(|ctx| ctx.event_id == EventId::RsgObtainBlazeRod);

                if bastion_ss_check && bastion_ss_context_check {
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

    async fn handle_non_pace_event(&self, live_link: String, last_event: &Event, guild_data: &mut GuildData) {
        let player_data = match guild_data.players.get_mut(&self.record.nickname.to_lowercase()) {
            Some(data) => data,
            None => {
                return println!(
                    "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                    self.record.nickname, 
                    guild_data.name
                );
            }
        };

        let runner_name = self.record.nickname.to_owned();
        let (minutes, seconds) = millis_to_mins_secs(last_event.igt as u64);
    
        let finish_minutes = match player_data.finish {
            Some(mins) => mins,
            None => {
                if !guild_data.is_private && minutes >= 10 {
                    return println!(
                        "Skipping guild name: {} because it is not a sub 10 completion and the guild is public.", 
                        guild_data.name
                    );
                }
                // minutes + 1 will always be greater than minutes. 
                // This is done to send finish message always if finish time is not defined.
                minutes + 1
            }, 
        };
        if minutes >= finish_minutes {
            return println!(
                "Skipping guild name: {} because finish time is above the defined amount.",
                guild_data.name,
            )
        }

        let content = format!(
            "## {} - Finish\n{}\t<t:{}:R>",
            format_time(last_event.igt as u64),
            live_link,
            (self.record.last_updated / 1000) as u64,
        );

        match guild_data.pace_channel.send_message(&self.ctx, |m| m.content(content)).await {
            Ok(_) => {
                println!(
                    "Sent pace-ping for user with name: '{}' for split: 'Finish' in guild name: {}.",
                    self.record.nickname, guild_data.name 
                );
            }
            Err(err) => {
                return eprintln!(
                    "Unable to send split: 'Finish' due to: {}",
                    err
                );
            }
        };

        if !guild_data.is_private || guild_data.lb_channel.is_none() {
            return println!(
                "Can't handle non pace event for guild name: {} because it is a public server or does not have a leaderboard channel.", 
                guild_data.name
            );
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

    async fn handle_pace_event(&self, live_link: String, last_event: &Event, guild_data: &mut GuildData) {
        let run_info = 
            match self.get_run_info(last_event) {
                Some(info) => info,
                None => {
                    return eprintln!("Unrecognized event id: {:#?}.", last_event.event_id);
                }
            };

        let player_data = match guild_data.players.get_mut(&self.record.nickname.to_lowercase()) {
            Some(data) => data,
            None => {
                if guild_data.is_private {
                    return println!(
                        "Skipping guild because player name: {} is not in the runners channel for guild name: {}", 
                        self.record.nickname, 
                        guild_data.name
                    );
                }
                let player_data = PlayerSplitsData::default();
                guild_data.players.insert(self.record.nickname.to_owned().to_lowercase(), player_data);
                guild_data.players.get_mut(&self.record.nickname.to_lowercase()).unwrap()
            }
        };
        let split_desc = match run_info.split.desc(&run_info.structure) {
            Some(desc) => desc,
            None => {
                return eprintln!("Unable to get split desc for split: {:#?}", run_info.split);
            }
        };

        let bastionless = if let RunType::Bastionless = run_info.run_type {
            "(Bastionless)"
        } else {
            ""
        };

        let roles_to_ping = guild_data.roles
            .iter()
            .filter(|role| {
                let (split_minutes, split_seconds) = millis_to_mins_secs(last_event.igt as u64);
                if role.guild_role.name.contains("PB") {
                    if !guild_data.is_private {
                        return false;
                    }
                    let pb_minutes = player_data.get(&role.split).unwrap().to_owned();
                    role.split == run_info.split && pb_minutes > split_minutes
                } else if role.guild_role.name.contains("+") {
                    role.split == run_info.split
                        && role.runner.to_lowercase() == self.record.nickname.to_lowercase()
                        && role.minutes >= split_minutes
                        && (role.minutes != split_minutes || role.seconds > split_seconds)
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

        let mut item_data_content = String::new();

        match &self.record.item_data {
            Some(data) => {
                let pearl_count = data.estimated_counts.get(&Item::MinecraftEnderPearl);
                let rod_count = data.estimated_counts.get(&Item::MinecraftBlazeRod);

                if rod_count.is_some() {
                    if item_data_content == "" {
                        item_data_content = format!("{} {}", ROD_EMOJI, rod_count.unwrap());
                    } else {
                        item_data_content = format!("{}  {} {}", item_data_content, ROD_EMOJI, rod_count.unwrap());
                    }
                }
                if pearl_count.is_some() {
                    if item_data_content == "" {
                        item_data_content = format!("{} {}", PEARL_EMOJI, pearl_count.unwrap());
                    } else {
                        item_data_content = format!("{}  {} {}", item_data_content, PEARL_EMOJI, pearl_count.unwrap());
                    }
                }
                if item_data_content != "" {
                    item_data_content = format!("\n{}", item_data_content); 
                }
            },
            None => (),
        }
        
        let content = format!(
            "## {} - {} {}\n{}\t<t:{}:R>{}\n{}",
            format_time(last_event.igt as u64),
            split_desc,
            bastionless,
            live_link,
            (self.record.last_updated / 1000) as u64,
            item_data_content,
            roles_to_ping
                .iter()
                .map(|role| role.guild_role.mention().to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
        
        match guild_data.pace_channel.send_message(&self.ctx, |m| m.content(content.to_owned())).await {
            Ok(mut message) => {
                println!(
                    "Sent pace-ping for user with name: '{}' for split: '{}' in guild name: {}.",
                    self.record.nickname, split_desc, guild_data.name 
                );
                let removable_roles = roles_to_ping.iter().filter(|r| r.runner.as_str() != "").map(|r| r.guild_role.mention()).collect::<Vec<_>>();
                let mut new_content = content.to_owned();
                for role in removable_roles {
                    let replacable_str = format!("{} ", role);
                    new_content = new_content.replace(replacable_str.as_str(), "");
                    let replacable_str = format!("{}", role);
                    new_content = new_content.replace(replacable_str.as_str(), "");
                }
                if new_content == content {
                    return;
                }
                match message.edit(&self.ctx.http, |m| m.content(new_content)).await {
                    Ok(_) => (),
                    Err(err) => {
                        return eprintln!("Unable to edit message due to: {}", err);
                    } 
                };
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
            let live_link = match self.record.user.live_account.to_owned() {
                Some(acc) => format!("[{}](<https://twitch.tv/{}>)", self.record.nickname.replace("_", SPECIAL_UNDERSCORE), acc),
                None => {
                    if !guild_data.is_private {
                        println!(
                            "Skipping guild: '{}' because user with name: '{}' is not live.",
                            guild_data.name, self.record.nickname,
                        );
                        continue;
                    } 
                    format!("Offline - {}", self.record.nickname.replace("_", SPECIAL_UNDERSCORE))
                }
            };
            let event_type = match get_event_type(&last_event) {
                Some(etype) => etype,
                None => {
                    return eprintln!(
                        "Unable to get event type for event: {:#?}. Skipping all guilds.",
                        last_event.event_id,
                    );
                }
            };
            match event_type {
                EventType::NonPaceEvent => {
                    self.handle_non_pace_event(live_link, last_event, guild_data).await;
                }
                EventType::PaceEvent => {
                    self.handle_pace_event(live_link, last_event, guild_data).await;
                }
            }
        }
    }
}
