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

use crate::eprintln;

use super::{
    channel_events::handle_channel_events, guild_create::handle_guild_create,
    guild_delete::handle_guild_delete, guild_role_events::handle_guild_role_events,
    interaction_create::handle_interaction_create, message_events::handle_message_events,
    ready::handle_ready, Handler,
};

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        handle_interaction_create(&ctx, interaction).await
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
                return eprintln!("GuildRoleDeleteError: No Role Data available.");
            }
        };
        handle_guild_role_events(&ctx, role, guild_id, self.cache_manager.clone()).await;
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        let guild_id = new.guild_id;
        handle_guild_role_events(&ctx, new, guild_id, self.cache_manager.clone()).await;
    }

    async fn channel_create(&self, ctx: Context, channel: &GuildChannel) {
        let guild_id = channel.guild_id;
        handle_channel_events(&ctx, channel, guild_id, self.cache_manager.clone()).await;
    }

    async fn channel_delete(&self, ctx: Context, channel: &GuildChannel) {
        let guild_id = channel.guild_id;
        handle_channel_events(&ctx, channel, guild_id, self.cache_manager.clone()).await;
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
                return eprintln!("MessageUpdateError: get guild id for update message event.");
            }
        };
        handle_message_events(&ctx, channel_id, guild_id, self.cache_manager.clone()).await;
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
                return eprintln!("MessageDeleteError: get guild id for delete message event.");
            }
        };
        handle_message_events(&ctx, channel_id, guild_id, self.cache_manager.clone()).await;
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let channel_id = new_message.channel_id;
        let guild_id = match new_message.guild_id {
            Some(id) => id,
            None => return eprintln!("MessageError: get guild id for message event."),
        };
        handle_message_events(&ctx, channel_id, guild_id, self.cache_manager.clone()).await;
    }

    async fn guild_delete(
        &self,
        _ctx: Context,
        _incomplete: UnavailableGuild,
        full: Option<Guild>,
    ) {
        let guild_id = match full {
            Some(guild) => guild.id,
            None => return eprintln!("GuildDeleteError: get guild id for deleted guild."),
        };
        handle_guild_delete(self.cache_manager.clone(), guild_id).await
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        handle_guild_create(&ctx, guild.id, self.cache_manager.clone()).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        handle_ready(ctx, ready, self.cache_manager.clone()).await;
    }
}
