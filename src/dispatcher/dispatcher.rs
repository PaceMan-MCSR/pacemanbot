use serenity::builder::CreateEmbedAuthor;

use crate::{utils::get_event_type::get_event_type, ws::response::EventType, Result};

use super::{
    consts::SPECIAL_UNDERSCORE, non_pace_event::handle_non_pace_event,
    pace_event::handle_pace_event, Dispatcher,
};

impl Dispatcher {
    pub async fn dispatch(&self) -> Result<()> {
        let game_version = self.response.game_version.to_owned();
        if game_version.is_some() && game_version.unwrap() != "1.7.10" {
            println!("Skipping record because it was not of 1.7.10.");
            return Ok(());
        }
        let last_event = match self.response.event_list.last() {
            Some(evt) => evt,
            None => {
                return Err(format!(
                    "DispatcherError: get last event from events list for response: {:#?}",
                    self.response
                )
                .into())
            }
        };
        let mut locked_guild_cache = self.cache_manager.lock().await;
        for (_, guild_data) in locked_guild_cache.cache.iter_mut() {
            let live_link = match self.response.user.live_account.to_owned() {
                Some(acc) => format!("https://twitch.tv/{}", acc),
                None => {
                    if !guild_data.is_private {
                        println!(
                            "Skipping guild: '{}' because user with name: '{}' is not live.",
                            guild_data.name, self.response.nickname,
                        );
                        continue;
                    }
                    String::new()
                }
            };

            let mc_head_url = format!("https://api.mineatar.io/face/{}", self.response.user.uuid);
            let author_name = self.response.nickname.replace("_", SPECIAL_UNDERSCORE);
            let mut author = CreateEmbedAuthor::default();
            author.icon_url(mc_head_url);
            author.name(author_name);
            if live_link != String::new() {
                author.url(live_link.clone());
            }

            let event_type = match get_event_type(&last_event) {
                Some(etype) => etype,
                None => {
                    return Err(format!(
                        "DispatcherError: get event type for event: {:#?}. Skipping all guilds.",
                        last_event.event_id,
                    )
                    .into());
                }
            };
            match event_type {
                EventType::NonPaceEvent => {
                    handle_non_pace_event(
                        self.ctx.clone(),
                        &self.response,
                        live_link,
                        author,
                        last_event,
                        guild_data,
                    )
                    .await;
                }
                EventType::PaceEvent => {
                    handle_pace_event(
                        self.ctx.clone(),
                        &self.response,
                        live_link,
                        author,
                        last_event,
                        guild_data,
                    )
                    .await;
                }
            }
        }
        Ok(())
    }
}
