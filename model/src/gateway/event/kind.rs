use serde::{Deserialize, Serialize};

/// The type of an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    #[serde(rename = "GUILD_BAN_ADD")]
    BanAdd,
    #[serde(rename = "GUILD_BAN_REMOVE")]
    BanRemove,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsUpdate,
    ChannelUpdate,
    #[serde(rename = "APPLICATION_COMMAND_PERMISSIONS_UPDATE")]
    CommandPermissionsUpdate,
    GatewayHeartbeat,
    GatewayHeartbeatAck,
    GatewayHello,
    GatewayInvalidateSession,
    GatewayReconnect,
    GiftCodeUpdate,
    GuildCreate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildScheduledEventCreate,
    GuildScheduledEventDelete,
    GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildStickersUpdate,
    GuildUpdate,
    IntegrationCreate,
    IntegrationDelete,
    IntegrationUpdate,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    #[serde(rename = "GUILD_MEMBER_ADD")]
    MemberAdd,
    #[serde(rename = "GUILD_MEMBER_REMOVE")]
    MemberRemove,
    #[serde(rename = "GUILD_MEMBER_UPDATE")]
    MemberUpdate,
    #[serde(rename = "GUILD_MEMBERS_CHUNK")]
    MemberChunk,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessageUpdate,
    PresenceUpdate,
    PresencesReplace,
    #[serde(rename = "MESSAGE_REACTION_ADD")]
    ReactionAdd,
    #[serde(rename = "MESSAGE_REACTION_REMOVE")]
    ReactionRemove,
    #[serde(rename = "MESSAGE_REACTION_REMOVE_ALL")]
    ReactionRemoveAll,
    #[serde(rename = "MESSAGE_REACTION_REMOVE_EMOJI")]
    ReactionRemoveEmoji,
    Ready,
    Resumed,
    #[serde(rename = "GUILD_ROLE_CREATE")]
    RoleCreate,
    #[serde(rename = "GUILD_ROLE_DELETE")]
    RoleDelete,
    #[serde(rename = "GUILD_ROLE_UPDATE")]
    RoleUpdate,
    ShardConnected,
    ShardConnecting,
    ShardDisconnected,
    ShardIdentifying,
    ShardReconnecting,
    ShardPayload,
    ShardResuming,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    ThreadCreate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    ThreadUpdate,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
}

impl EventType {
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Self::BanAdd => Some("GUILD_BAN_ADD"),
            Self::BanRemove => Some("GUILD_BAN_REMOVE"),
            Self::ChannelCreate => Some("CHANNEL_CREATE"),
            Self::ChannelDelete => Some("CHANNEL_DELETE"),
            Self::ChannelPinsUpdate => Some("CHANNEL_PINS_UPDATE"),
            Self::ChannelUpdate => Some("CHANNEL_UPDATE"),
            Self::CommandPermissionsUpdate => Some("APPLICATION_COMMAND_PERMISSIONS_UPDATE"),
            Self::GiftCodeUpdate => Some("GIFT_CODE_UPDATE"),
            Self::GuildCreate => Some("GUILD_CREATE"),
            Self::GuildDelete => Some("GUILD_DELETE"),
            Self::GuildEmojisUpdate => Some("GUILD_EMOJIS_UPDATE"),
            Self::GuildIntegrationsUpdate => Some("GUILD_INTEGRATIONS_UPDATE"),
            Self::GuildScheduledEventCreate => Some("GUILD_SCHEDULED_EVENT_CREATE"),
            Self::GuildScheduledEventDelete => Some("GUILD_SCHEDULED_EVENT_DELETE"),
            Self::GuildScheduledEventUpdate => Some("GUILD_SCHEDULED_EVENT_UPDATE"),
            Self::GuildScheduledEventUserAdd => Some("GUILD_SCHEDULED_EVENT_USER_ADD"),
            Self::GuildScheduledEventUserRemove => Some("GUILD_SCHEDULED_EVENT_USER_REMOVE"),
            Self::GuildStickersUpdate => Some("GUILD_STICKERS_UPDATE"),
            Self::GuildUpdate => Some("GUILD_UPDATE"),
            Self::IntegrationCreate => Some("INTEGRATION_CREATE"),
            Self::IntegrationDelete => Some("INTEGRATION_DELETE"),
            Self::IntegrationUpdate => Some("INTEGRATION_UPDATE"),
            Self::InteractionCreate => Some("INTERACTION_CREATE"),
            Self::InviteCreate => Some("INVITE_CREATE"),
            Self::InviteDelete => Some("INVITE_DELETE"),
            Self::MemberAdd => Some("GUILD_MEMBER_ADD"),
            Self::MemberRemove => Some("GUILD_MEMBER_REMOVE"),
            Self::MemberUpdate => Some("GUILD_MEMBER_UPDATE"),
            Self::MemberChunk => Some("GUILD_MEMBERS_CHUNK"),
            Self::MessageCreate => Some("MESSAGE_CREATE"),
            Self::MessageDelete => Some("MESSAGE_DELETE"),
            Self::MessageDeleteBulk => Some("MESSAGE_DELETE_BULK"),
            Self::MessageUpdate => Some("MESSAGE_UPDATE"),
            Self::PresenceUpdate => Some("PRESENCE_UPDATE"),
            Self::PresencesReplace => Some("PRESENCES_REPLACE"),
            Self::ReactionAdd => Some("MESSAGE_REACTION_ADD"),
            Self::ReactionRemove => Some("MESSAGE_REACTION_REMOVE"),
            Self::ReactionRemoveAll => Some("MESSAGE_REACTION_REMOVE_ALL"),
            Self::ReactionRemoveEmoji => Some("MESSAGE_REACTION_REMOVE_EMOJI"),
            Self::Ready => Some("READY"),
            Self::Resumed => Some("RESUMED"),
            Self::RoleCreate => Some("GUILD_ROLE_CREATE"),
            Self::RoleDelete => Some("GUILD_ROLE_DELETE"),
            Self::RoleUpdate => Some("GUILD_ROLE_UPDATE"),
            Self::StageInstanceCreate => Some("STAGE_INSTANCE_CREATE"),
            Self::StageInstanceDelete => Some("STAGE_INSTANCE_DELETE"),
            Self::StageInstanceUpdate => Some("STAGE_INSTANCE_UPDATE"),
            Self::ThreadCreate => Some("THREAD_CREATE"),
            Self::ThreadDelete => Some("THREAD_DELETE"),
            Self::ThreadListSync => Some("THREAD_LIST_SYNC"),
            Self::ThreadMemberUpdate => Some("THREAD_MEMBER_UPDATE"),
            Self::ThreadMembersUpdate => Some("THREAD_MEMBERS_UPDATE"),
            Self::ThreadUpdate => Some("THREAD_UPDATE"),
            Self::TypingStart => Some("TYPING_START"),
            Self::UnavailableGuild => Some("UNAVAILABLE_GUILD"),
            Self::UserUpdate => Some("USER_UPDATE"),
            Self::VoiceServerUpdate => Some("VOICE_SERVER_UPDATE"),
            Self::VoiceStateUpdate => Some("VOICE_STATE_UPDATE"),
            Self::WebhooksUpdate => Some("WEBHOOKS_UPDATE"),
            Self::GatewayHeartbeat
            | Self::GatewayHeartbeatAck
            | Self::GatewayHello
            | Self::GatewayInvalidateSession
            | Self::GatewayReconnect
            | Self::ShardConnected
            | Self::ShardConnecting
            | Self::ShardDisconnected
            | Self::ShardIdentifying
            | Self::ShardReconnecting
            | Self::ShardPayload
            | Self::ShardResuming => None,
        }
    }
}

impl<'a> TryFrom<&'a str> for EventType {
    type Error = &'a str;

    fn try_from(event_type: &'a str) -> Result<Self, Self::Error> {
        match event_type {
            "GUILD_BAN_ADD" => Ok(Self::BanAdd),
            "GUILD_BAN_REMOVE" => Ok(Self::BanRemove),
            "CHANNEL_CREATE" => Ok(Self::ChannelCreate),
            "CHANNEL_DELETE" => Ok(Self::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => Ok(Self::ChannelPinsUpdate),
            "CHANNEL_UPDATE" => Ok(Self::ChannelUpdate),
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => Ok(Self::CommandPermissionsUpdate),
            "GIFT_CODE_UPDATE" => Ok(Self::GiftCodeUpdate),
            "GUILD_CREATE" => Ok(Self::GuildCreate),
            "GUILD_DELETE" => Ok(Self::GuildDelete),
            "GUILD_EMOJIS_UPDATE" => Ok(Self::GuildEmojisUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => Ok(Self::GuildIntegrationsUpdate),
            "GUILD_SCHEDULED_EVENT_CREATE" => Ok(Self::GuildScheduledEventCreate),
            "GUILD_SCHEDULED_EVENT_DELETE" => Ok(Self::GuildScheduledEventDelete),
            "GUILD_SCHEDULED_EVENT_UPDATE" => Ok(Self::GuildScheduledEventUpdate),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => Ok(Self::GuildScheduledEventUserAdd),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => Ok(Self::GuildScheduledEventUserRemove),
            "GUILD_UPDATE" => Ok(Self::GuildUpdate),
            "INTEGRATION_CREATE" => Ok(Self::IntegrationCreate),
            "INTEGRATION_DELETE" => Ok(Self::IntegrationDelete),
            "INTEGRATION_UPDATE" => Ok(Self::IntegrationUpdate),
            "INTERACTION_CREATE" => Ok(Self::InteractionCreate),
            "INVITE_CREATE" => Ok(Self::InviteCreate),
            "INVITE_DELETE" => Ok(Self::InviteDelete),
            "GUILD_MEMBER_ADD" => Ok(Self::MemberAdd),
            "GUILD_MEMBER_REMOVE" => Ok(Self::MemberRemove),
            "GUILD_MEMBER_UPDATE" => Ok(Self::MemberUpdate),
            "GUILD_MEMBERS_CHUNK" => Ok(Self::MemberChunk),
            "MESSAGE_CREATE" => Ok(Self::MessageCreate),
            "MESSAGE_DELETE" => Ok(Self::MessageDelete),
            "MESSAGE_DELETE_BULK" => Ok(Self::MessageDeleteBulk),
            "MESSAGE_UPDATE" => Ok(Self::MessageUpdate),
            "PRESENCE_UPDATE" => Ok(Self::PresenceUpdate),
            "PRESENCES_REPLACE" => Ok(Self::PresencesReplace),
            "MESSAGE_REACTION_ADD" => Ok(Self::ReactionAdd),
            "MESSAGE_REACTION_REMOVE" => Ok(Self::ReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => Ok(Self::ReactionRemoveAll),
            "MESSAGE_REACTION_REMOVE_EMOJI" => Ok(Self::ReactionRemoveEmoji),
            "READY" => Ok(Self::Ready),
            "RESUMED" => Ok(Self::Resumed),
            "GUILD_ROLE_CREATE" => Ok(Self::RoleCreate),
            "GUILD_ROLE_DELETE" => Ok(Self::RoleDelete),
            "GUILD_ROLE_UPDATE" => Ok(Self::RoleUpdate),
            "STAGE_INSTANCE_CREATE" => Ok(Self::StageInstanceCreate),
            "STAGE_INSTANCE_DELETE" => Ok(Self::StageInstanceDelete),
            "STAGE_INSTANCE_UPDATE" => Ok(Self::StageInstanceUpdate),
            "THREAD_CREATE" => Ok(Self::ThreadCreate),
            "THREAD_DELETE" => Ok(Self::ThreadDelete),
            "THREAD_LIST_SYNC" => Ok(Self::ThreadListSync),
            "THREAD_MEMBER_UPDATE" => Ok(Self::ThreadMemberUpdate),
            "THREAD_MEMBERS_UPDATE" => Ok(Self::ThreadMembersUpdate),
            "THREAD_UPDATE" => Ok(Self::ThreadUpdate),
            "TYPING_START" => Ok(Self::TypingStart),
            "UNAVAILABLE_GUILD" => Ok(Self::UnavailableGuild),
            "USER_UPDATE" => Ok(Self::UserUpdate),
            "VOICE_SERVER_UPDATE" => Ok(Self::VoiceServerUpdate),
            "VOICE_STATE_UPDATE" => Ok(Self::VoiceStateUpdate),
            "WEBHOOKS_UPDATE" => Ok(Self::WebhooksUpdate),
            _ => Err(event_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EventType;
    use serde_test::Token;

    fn assert_variant(kind: EventType, name: &'static str) {
        serde_test::assert_tokens(
            &kind,
            &[Token::UnitVariant {
                name: "EventType",
                variant: name,
            }],
        );
    }

    #[test]
    fn test_variants() {
        assert_variant(EventType::BanAdd, "GUILD_BAN_ADD");
        assert_variant(EventType::BanRemove, "GUILD_BAN_REMOVE");
        assert_variant(EventType::ChannelCreate, "CHANNEL_CREATE");
        assert_variant(EventType::ChannelDelete, "CHANNEL_DELETE");
        assert_variant(EventType::ChannelPinsUpdate, "CHANNEL_PINS_UPDATE");
        assert_variant(EventType::ChannelUpdate, "CHANNEL_UPDATE");
        assert_variant(
            EventType::CommandPermissionsUpdate,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE",
        );
        assert_variant(EventType::GatewayHeartbeat, "GATEWAY_HEARTBEAT");
        assert_variant(EventType::GatewayHeartbeatAck, "GATEWAY_HEARTBEAT_ACK");
        assert_variant(EventType::GatewayHello, "GATEWAY_HELLO");
        assert_variant(
            EventType::GatewayInvalidateSession,
            "GATEWAY_INVALIDATE_SESSION",
        );
        assert_variant(EventType::GatewayReconnect, "GATEWAY_RECONNECT");
        assert_variant(EventType::GiftCodeUpdate, "GIFT_CODE_UPDATE");
        assert_variant(EventType::GuildCreate, "GUILD_CREATE");
        assert_variant(EventType::GuildDelete, "GUILD_DELETE");
        assert_variant(EventType::GuildEmojisUpdate, "GUILD_EMOJIS_UPDATE");
        assert_variant(
            EventType::GuildIntegrationsUpdate,
            "GUILD_INTEGRATIONS_UPDATE",
        );
        assert_variant(
            EventType::GuildScheduledEventCreate,
            "GUILD_SCHEDULED_EVENT_CREATE",
        );
        assert_variant(
            EventType::GuildScheduledEventDelete,
            "GUILD_SCHEDULED_EVENT_DELETE",
        );
        assert_variant(
            EventType::GuildScheduledEventUpdate,
            "GUILD_SCHEDULED_EVENT_UPDATE",
        );
        assert_variant(
            EventType::GuildScheduledEventUserAdd,
            "GUILD_SCHEDULED_EVENT_USER_ADD",
        );
        assert_variant(
            EventType::GuildScheduledEventUserRemove,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE",
        );
        assert_variant(EventType::GuildUpdate, "GUILD_UPDATE");
        assert_variant(EventType::IntegrationCreate, "INTEGRATION_CREATE");
        assert_variant(EventType::IntegrationDelete, "INTEGRATION_DELETE");
        assert_variant(EventType::IntegrationUpdate, "INTEGRATION_UPDATE");
        assert_variant(EventType::InteractionCreate, "INTERACTION_CREATE");
        assert_variant(EventType::InviteCreate, "INVITE_CREATE");
        assert_variant(EventType::InviteDelete, "INVITE_DELETE");
        assert_variant(EventType::MemberAdd, "GUILD_MEMBER_ADD");
        assert_variant(EventType::MemberRemove, "GUILD_MEMBER_REMOVE");
        assert_variant(EventType::MemberUpdate, "GUILD_MEMBER_UPDATE");
        assert_variant(EventType::MemberChunk, "GUILD_MEMBERS_CHUNK");
        assert_variant(EventType::MessageCreate, "MESSAGE_CREATE");
        assert_variant(EventType::MessageDelete, "MESSAGE_DELETE");
        assert_variant(EventType::MessageDeleteBulk, "MESSAGE_DELETE_BULK");
        assert_variant(EventType::MessageUpdate, "MESSAGE_UPDATE");
        assert_variant(EventType::PresenceUpdate, "PRESENCE_UPDATE");
        assert_variant(EventType::PresencesReplace, "PRESENCES_REPLACE");
        assert_variant(EventType::ReactionAdd, "MESSAGE_REACTION_ADD");
        assert_variant(EventType::ReactionRemove, "MESSAGE_REACTION_REMOVE");
        assert_variant(EventType::ReactionRemoveAll, "MESSAGE_REACTION_REMOVE_ALL");
        assert_variant(
            EventType::ReactionRemoveEmoji,
            "MESSAGE_REACTION_REMOVE_EMOJI",
        );
        assert_variant(EventType::Ready, "READY");
        assert_variant(EventType::Resumed, "RESUMED");
        assert_variant(EventType::RoleCreate, "GUILD_ROLE_CREATE");
        assert_variant(EventType::RoleDelete, "GUILD_ROLE_DELETE");
        assert_variant(EventType::RoleUpdate, "GUILD_ROLE_UPDATE");
        assert_variant(EventType::ShardConnected, "SHARD_CONNECTED");
        assert_variant(EventType::ShardConnecting, "SHARD_CONNECTING");
        assert_variant(EventType::ShardDisconnected, "SHARD_DISCONNECTED");
        assert_variant(EventType::ShardIdentifying, "SHARD_IDENTIFYING");
        assert_variant(EventType::ShardPayload, "SHARD_PAYLOAD");
        assert_variant(EventType::ShardReconnecting, "SHARD_RECONNECTING");
        assert_variant(EventType::ShardResuming, "SHARD_RESUMING");
        assert_variant(EventType::StageInstanceCreate, "STAGE_INSTANCE_CREATE");
        assert_variant(EventType::StageInstanceDelete, "STAGE_INSTANCE_DELETE");
        assert_variant(EventType::StageInstanceUpdate, "STAGE_INSTANCE_UPDATE");
        assert_variant(EventType::ThreadCreate, "THREAD_CREATE");
        assert_variant(EventType::ThreadDelete, "THREAD_DELETE");
        assert_variant(EventType::ThreadListSync, "THREAD_LIST_SYNC");
        assert_variant(EventType::ThreadMemberUpdate, "THREAD_MEMBER_UPDATE");
        assert_variant(EventType::ThreadMembersUpdate, "THREAD_MEMBERS_UPDATE");
        assert_variant(EventType::ThreadUpdate, "THREAD_UPDATE");
        assert_variant(EventType::TypingStart, "TYPING_START");
        assert_variant(EventType::UnavailableGuild, "UNAVAILABLE_GUILD");
        assert_variant(EventType::UserUpdate, "USER_UPDATE");
        assert_variant(EventType::VoiceServerUpdate, "VOICE_SERVER_UPDATE");
        assert_variant(EventType::VoiceStateUpdate, "VOICE_STATE_UPDATE");
        assert_variant(EventType::WebhooksUpdate, "WEBHOOKS_UPDATE");
    }
}
