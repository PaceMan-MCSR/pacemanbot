use std::sync::Arc;

use serenity::{
    client::Context,
    futures::lock::Mutex,
    model::{
        guild::Role,
        id::{ChannelId, GuildId},
        prelude::{Activity, GuildChannel, Interaction, Ready},
        user::OnlineStatus,
    },
};

use crate::{
    cache::Cache,
    command::get_default_commands,
    config::{
        Config, PACEMANBOT_CHANNEL, PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL,
        PACEMANBOT_RUNNER_NAMES_CHANNEL, ROLE_PREFIX, ROLE_PREFIX_115, ROLE_PREFIX_17,
        ROLE_PREFIX_AA,
    },
    interaction::{handle_application_command_interaction, handle_message_component_interaction},
    log::Log,
    ws::WS,
};

pub struct Handler {
    pub log: Arc<Log>,
    pub cache: Arc<Mutex<Cache>>,
    pub ws: Arc<WS>,
}

impl Handler {
    pub async fn handle_interaction_create(&self, ctx: &Context, interaction: Interaction) {
        let mut interaction_error: Option<String> = None;
        if let Some(command) = interaction.as_application_command() {
            match command.defer_ephemeral(&ctx.http).await {
                Ok(_) => (),
                Err(err) => {
                    self.log.error(
                        format!(
                            "Failed to defer_ephemeral on application command interaction: {}",
                            err
                        )
                        .as_str(),
                    );
                }
            };
            match handle_application_command_interaction(ctx, command).await {
                Ok(_) => (),
                Err(err) => {
                    let content =
                        format!("Failed to handle application command interaction: {}", err);
                    interaction_error = Some(content.to_string());
                }
            };
            if let Some(application_command_error) = interaction_error.as_ref() {
                match command
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(application_command_error.to_string())
                    })
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        self.log
                            .error(format!("Failed to edit application command: {}", err).as_str());
                    }
                };
                return self.log.error(application_command_error.as_str());
            }
        }
        if let Some(message_component) = interaction.as_message_component() {
            match message_component.defer_ephemeral(&ctx).await {
                Ok(_) => (),
                Err(err) => {
                    return self.log.error(
                        format!(
                            "Failed to defer_ephemeral on message_component failed: {}",
                            err
                        )
                        .as_str(),
                    );
                }
            };
            match handle_message_component_interaction(ctx, message_component).await {
                Ok(_) => (),
                Err(err) => {
                    let content =
                        format!("Failed to handle message component interaction: {}", err);
                    interaction_error = Some(content.to_string());
                }
            };
            if let Some(message_component_error) = interaction_error.as_ref() {
                match message_component
                    .edit_original_interaction_response(&ctx.http, |m| {
                        m.content(message_component_error.to_string())
                    })
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        self.log
                            .error(format!("Failed to edit message component: {}", err).as_str());
                    }
                };
                return self.log.error(message_component_error.as_str());
            }
        }
    }

    pub async fn handle_guild_role_events(&self, ctx: &Context, new: Role, guild_id: GuildId) {
        if !new.name.starts_with(ROLE_PREFIX)
            || new.name.starts_with(ROLE_PREFIX_115)
            || new.name.starts_with(ROLE_PREFIX_17)
            || new.name.starts_with(ROLE_PREFIX_AA)
        {
            return self.log.info(
                format!(
                    "Skipping role create event because it is not something that concerns the bot."
                )
                .as_str(),
            );
        }
        self.update_cache(ctx, guild_id).await;
    }

    pub async fn handle_channel_events(
        &self,
        ctx: &Context,
        channel: &GuildChannel,
        guild_id: GuildId,
    ) {
        match channel.name.as_str() {
            PACEMANBOT_RUNNER_NAMES_CHANNEL
            | PACEMANBOT_CHANNEL
            | PACEMANBOT_RUNNER_LEADERBOARD_CHANNEL => {
                self.update_cache(ctx, guild_id).await;
            }
            _ => {
                return self.log.info(
                    format!(
                        "Skipping channel event because it is not something that concerns the bot."
                    )
                    .as_str(),
                )
            }
        }
    }

    pub async fn handle_message_events(
        &self,
        ctx: &Context,
        channel_id: ChannelId,
        guild_id: GuildId,
    ) {
        let name = match channel_id.name(&ctx.cache).await {
            Some(name) => name,
            None => {
                return self.log.error(
                    format!(
                        "Error while getting guild name for channel id: {}.",
                        channel_id
                    )
                    .as_str(),
                );
            }
        };
        if name != PACEMANBOT_RUNNER_NAMES_CHANNEL {
            return self.log.info(
                format!(
                    "Skipping message delete because it was not sent in #{}.",
                    PACEMANBOT_RUNNER_NAMES_CHANNEL,
                )
                .as_str(),
            );
        }
        self.update_cache(ctx, guild_id).await;
    }

    pub async fn handle_guild_delete(&self, guild_id: GuildId) {
        let mut locked_cache = self.cache.lock().await;
        match locked_cache.entries.remove(&guild_id) {
            Some(_) => self
                .log
                .info(format!("Removed guild from cache: {}", guild_id).as_str()),
            None => self
                .log
                .info(format!("Failed to remove guild from cache: {}", guild_id).as_str()),
        };
    }

    pub async fn handle_guild_create(&self, ctx: &Context, guild_id: GuildId) {
        match GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            let default_commands = get_default_commands();
            for command in default_commands {
                commands.create_application_command(move |c| {
                    c.name(command.name()).description(command.description());
                    command.create_options(c)
                });
            }
            commands
        })
        .await
        {
            Ok(_) => (),
            Err(err) => self
                .log
                .error(format!("Error creating command: {}", err).as_str()),
        }
        ctx.set_presence(Some(Activity::watching("paceman.gg")), OnlineStatus::Online)
            .await;
        self.update_cache(ctx, guild_id).await;
    }

    pub async fn handle_ready(&self, ctx: Context, ready: Ready) {
        self.log
            .info(format!("{} is connected!", ready.user.name).as_str());
        let ws = self.ws.clone();
        let log = self.log.clone();
        let cache = self.cache.clone();
        let ctx = Arc::new(ctx);
        tokio::spawn(async move {
            ws.start_event_loop(ctx, log, cache).await;
        });
    }

    pub async fn update_cache(&self, ctx: &Context, guild_id: GuildId) {
        let mut locked_cache = self.cache.lock().await;
        match Config::parse_config_for_guild(ctx, guild_id).await {
            Ok(guild_cache_entry) => locked_cache.entries.insert(guild_id, guild_cache_entry),
            Err(err) => {
                return self
                    .log
                    .error(format!("Failed to parse config for guild: {}", err).as_str())
            }
        };
    }
}
