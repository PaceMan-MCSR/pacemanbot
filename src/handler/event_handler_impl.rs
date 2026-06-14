use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        event::MessageUpdateEvent,
        guild::{Guild, Role, UnavailableGuild},
        id::{ChannelId, GuildId, MessageId, RoleId},
        prelude::{GuildChannel, Interaction, Message, Ready},
    },
};

use crate::handler::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.handle_interaction_create(&ctx, interaction).await
    }

    async fn guild_role_delete(
        &self,
        ctx: Context,
        guild_id: GuildId,
        _removed_role_id: RoleId,
        removed_role_data_if_available: Option<Role>,
    ) {
        let role = match removed_role_data_if_available {
            Some(role) => role,
            None => {
                return self
                    .log
                    .error("Failed to delete role: no role data available.");
            }
        };
        self.handle_guild_role_events(&ctx, role, guild_id).await;
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        let guild_id = new.guild_id;
        self.handle_guild_role_events(&ctx, new, guild_id).await;
    }

    async fn channel_create(&self, ctx: Context, channel: &GuildChannel) {
        let guild_id = channel.guild_id;
        self.handle_channel_events(&ctx, channel, guild_id).await;
    }

    async fn channel_delete(&self, ctx: Context, channel: &GuildChannel) {
        let guild_id = channel.guild_id;
        self.handle_channel_events(&ctx, channel, guild_id).await;
    }

    async fn message_update(
        &self,
        ctx: Context,
        _old_if_available: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        let channel_id = event.channel_id;
        let guild_id = match event.guild_id {
            Some(id) => id,
            None => {
                return self.log.error(
                    format!("Failed to update message: get guild id for update message event.")
                        .as_str(),
                );
            }
        };
        self.handle_message_events(&ctx, channel_id, guild_id).await;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        _deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        let guild_id = match guild_id {
            Some(id) => id,
            None => {
                return self.log.error(
                    format!("Failed to delete message: get guild id for delete message event.")
                        .as_str(),
                );
            }
        };
        self.handle_message_events(&ctx, channel_id, guild_id).await;
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let channel_id = new_message.channel_id;
        let guild_id = match new_message.guild_id {
            Some(id) => id,
            None => return self.log.error("Failed to get guild id for message event."),
        };
        self.handle_message_events(&ctx, channel_id, guild_id).await;
    }

    async fn guild_delete(
        &self,
        _ctx: Context,
        _incomplete: UnavailableGuild,
        full: Option<Guild>,
    ) {
        let guild_id = match full {
            Some(guild) => guild.id,
            None => {
                return self.log.error(
                    format!("Failed to delete guild: get guild id for deleted guild.").as_str(),
                )
            }
        };
        self.handle_guild_delete(guild_id).await
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        self.handle_guild_create(&ctx, guild.id).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        self.handle_ready(ctx, ready).await;
    }
}
