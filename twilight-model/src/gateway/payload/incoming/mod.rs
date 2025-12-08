//! Payloads that are incoming from the Discord Gateway API.
//!
//! These are sent by the Discord Gateway API to shards. The only payload
//! required for the operation of a shard is [`Ready`]; all other payloads are
//! for users to consume in their own operations based on the [`Intents`] they
//! have set
//!
//! Refer to [Discord Docs / Gateway Events][1] for Discord's documentation
//! about incoming and outgoing events.
//!
//! [`Intents`]: crate::gateway::Intents
//! [1]: https://discord.com/developers/docs/topics/gateway#commands-and-events-gateway-events

pub mod invite_create;
pub mod reaction_remove_emoji;

mod auto_moderation_action_execution;
mod auto_moderation_rule_create;
mod auto_moderation_rule_delete;
mod auto_moderation_rule_update;
mod ban_add;
mod ban_remove;
mod channel_create;
mod channel_delete;
mod channel_pins_update;
mod channel_update;
mod command_permissions_update;
mod entitlement_create;
mod entitlement_delete;
mod entitlement_update;
mod guild_audit_log_entry_create;
mod guild_create;
mod guild_delete;
mod guild_emojis_update;
mod guild_integrations_update;
mod guild_scheduled_event_create;
mod guild_scheduled_event_delete;
mod guild_scheduled_event_update;
mod guild_scheduled_event_user_add;
mod guild_scheduled_event_user_remove;
mod guild_stickers_update;
mod guild_update;
mod hello;
mod integration_create;
mod integration_delete;
mod integration_update;
mod interaction_create;
mod invite_delete;
mod member_add;
mod member_chunk;
mod member_remove;
mod member_update;
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_poll_vote_add;
mod message_poll_vote_remove;
mod message_update;
mod presence_update;
pub mod rate_limited;
mod reaction_add;
mod reaction_remove;
mod reaction_remove_all;
mod ready;
mod role_create;
mod role_delete;
mod role_update;
mod stage_instance_create;
mod stage_instance_delete;
mod stage_instance_update;
mod thread_create;
mod thread_delete;
mod thread_list_sync;
mod thread_member_update;
mod thread_members_update;
mod thread_update;
mod typing_start;
mod unavailable_guild;
mod user_update;
mod voice_server_update;
mod voice_state_update;
mod webhooks_update;

pub use self::{
    auto_moderation_action_execution::AutoModerationActionExecution,
    auto_moderation_rule_create::AutoModerationRuleCreate,
    auto_moderation_rule_delete::AutoModerationRuleDelete,
    auto_moderation_rule_update::AutoModerationRuleUpdate, ban_add::BanAdd, ban_remove::BanRemove,
    channel_create::ChannelCreate, channel_delete::ChannelDelete,
    channel_pins_update::ChannelPinsUpdate, channel_update::ChannelUpdate,
    command_permissions_update::CommandPermissionsUpdate, entitlement_create::EntitlementCreate,
    entitlement_delete::EntitlementDelete, entitlement_update::EntitlementUpdate,
    guild_audit_log_entry_create::GuildAuditLogEntryCreate, guild_create::GuildCreate,
    guild_delete::GuildDelete, guild_emojis_update::GuildEmojisUpdate,
    guild_integrations_update::GuildIntegrationsUpdate,
    guild_scheduled_event_create::GuildScheduledEventCreate,
    guild_scheduled_event_delete::GuildScheduledEventDelete,
    guild_scheduled_event_update::GuildScheduledEventUpdate,
    guild_scheduled_event_user_add::GuildScheduledEventUserAdd,
    guild_scheduled_event_user_remove::GuildScheduledEventUserRemove,
    guild_stickers_update::GuildStickersUpdate, guild_update::GuildUpdate, hello::Hello,
    integration_create::IntegrationCreate, integration_delete::IntegrationDelete,
    integration_update::IntegrationUpdate, interaction_create::InteractionCreate,
    invite_create::InviteCreate, invite_delete::InviteDelete, member_add::MemberAdd,
    member_chunk::MemberChunk, member_remove::MemberRemove, member_update::MemberUpdate,
    message_create::MessageCreate, message_delete::MessageDelete,
    message_delete_bulk::MessageDeleteBulk, message_poll_vote_add::MessagePollVoteAdd,
    message_poll_vote_remove::MessagePollVoteRemove, message_update::MessageUpdate,
    presence_update::PresenceUpdate, rate_limited::RateLimited, reaction_add::ReactionAdd,
    reaction_remove::ReactionRemove, reaction_remove_all::ReactionRemoveAll,
    reaction_remove_emoji::ReactionRemoveEmoji, ready::Ready, role_create::RoleCreate,
    role_delete::RoleDelete, role_update::RoleUpdate, stage_instance_create::StageInstanceCreate,
    stage_instance_delete::StageInstanceDelete, stage_instance_update::StageInstanceUpdate,
    thread_create::ThreadCreate, thread_delete::ThreadDelete, thread_list_sync::ThreadListSync,
    thread_member_update::ThreadMemberUpdate, thread_members_update::ThreadMembersUpdate,
    thread_update::ThreadUpdate, typing_start::TypingStart, unavailable_guild::UnavailableGuild,
    user_update::UserUpdate, voice_server_update::VoiceServerUpdate,
    voice_state_update::VoiceStateUpdate, webhooks_update::WebhooksUpdate,
};
