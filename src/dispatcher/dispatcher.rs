use serenity::builder::CreateEmbedAuthor;

use crate::{utils::get_event_type::get_event_type, ws::response::EventType, Result};

use super::{
    consts::SPECIAL_UNDERSCORE, non_pace_event::handle_non_pace_event,
    pace_event::handle_pace_event, Dispatcher,
};

impl Dispatcher {
    pub async fn dispatch(&self) -> Result<()> {
        let last_advancement = match self.response.completed.last() {
            Some(evt) => evt,
            None => {
                return Err(format!(
                    "DispatcherError: get last advancement from completed list for response: {:#?}",
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

            let stats_link = format!("https://paceman.gg/stats/run/{}", self.response.world_id);
            let mc_head_url = format!("https://api.mineatar.io/face/{}", self.response.user.uuid);
            let author_name = self.response.nickname.replace("_", SPECIAL_UNDERSCORE);
            let mut author = CreateEmbedAuthor::default();
            author.icon_url(mc_head_url);
            author.name(author_name);
            if live_link != String::new() {
                author.url(live_link.clone());
            }

            let event_type = match get_event_type(&last_advancement, self.response.completed.len())
            {
                Some(etype) => etype,
                None => {
                    return Err(format!(
                        "DispatcherError: get event type for event: {:#?}. Skipping all guilds.",
                        last_advancement.event_id,
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
                        stats_link,
                        author,
                        last_advancement,
                        guild_data,
                    )
                    .await;
                }
                EventType::PaceEvent => {
                    handle_pace_event(
                        self.ctx.clone(),
                        &self.response,
                        live_link,
                        stats_link,
                        author,
                        last_advancement,
                        guild_data,
                    )
                    .await;
                }
            }
        }
        Ok(())
    }
}
