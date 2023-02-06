use super::{super::payload::incoming::*, Event, EventConversionError, EventType};
use serde::{
    de::{Deserialize, DeserializeSeed, Deserializer, Error as DeError, IgnoredAny},
    Serialize,
};

/// A dispatch event, containing information about a created guild, a member
/// added, etc.
///
/// You can deserialize into a `DispatchEvent` via
/// [`DispatchEventWithTypeDeserializer`].
// **NOTE**: When adding a variant, be sure to add it to the DeserializeSeed
// implementation.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DispatchEvent {
    AutoModerationActionExecution(AutoModerationActionExecution),
    AutoModerationRuleCreate(Box<AutoModerationRuleCreate>),
    AutoModerationRuleDelete(Box<AutoModerationRuleDelete>),
    AutoModerationRuleUpdate(Box<AutoModerationRuleUpdate>),
    BanAdd(BanAdd),
    BanRemove(BanRemove),
    ChannelCreate(Box<ChannelCreate>),
    ChannelDelete(Box<ChannelDelete>),
    ChannelPinsUpdate(ChannelPinsUpdate),
    ChannelUpdate(Box<ChannelUpdate>),
    CommandPermissionsUpdate(CommandPermissionsUpdate),
    GiftCodeUpdate,
    GuildAuditLogEntryCreate(Box<GuildAuditLogEntryCreate>),
    GuildCreate(Box<GuildCreate>),
    GuildDelete(GuildDelete),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildScheduledEventCreate(Box<GuildScheduledEventCreate>),
    GuildScheduledEventDelete(Box<GuildScheduledEventDelete>),
    GuildScheduledEventUpdate(Box<GuildScheduledEventUpdate>),
    GuildScheduledEventUserAdd(GuildScheduledEventUserAdd),
    GuildScheduledEventUserRemove(GuildScheduledEventUserRemove),
    GuildStickersUpdate(GuildStickersUpdate),
    GuildUpdate(Box<GuildUpdate>),
    IntegrationCreate(Box<IntegrationCreate>),
    IntegrationDelete(IntegrationDelete),
    IntegrationUpdate(Box<IntegrationUpdate>),
    InteractionCreate(Box<InteractionCreate>),
    InviteCreate(Box<InviteCreate>),
    InviteDelete(InviteDelete),
    MemberAdd(Box<MemberAdd>),
    MemberRemove(MemberRemove),
    MemberUpdate(Box<MemberUpdate>),
    MemberChunk(MemberChunk),
    MessageCreate(Box<MessageCreate>),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageUpdate(Box<MessageUpdate>),
    PresenceUpdate(Box<PresenceUpdate>),
    PresencesReplace,
    ReactionAdd(Box<ReactionAdd>),
    ReactionRemove(Box<ReactionRemove>),
    ReactionRemoveAll(ReactionRemoveAll),
    ReactionRemoveEmoji(ReactionRemoveEmoji),
    Ready(Box<Ready>),
    Resumed,
    RoleCreate(RoleCreate),
    RoleDelete(RoleDelete),
    RoleUpdate(RoleUpdate),
    StageInstanceCreate(StageInstanceCreate),
    StageInstanceDelete(StageInstanceDelete),
    StageInstanceUpdate(StageInstanceUpdate),
    ThreadCreate(Box<ThreadCreate>),
    ThreadDelete(ThreadDelete),
    ThreadListSync(ThreadListSync),
    ThreadMemberUpdate(Box<ThreadMemberUpdate>),
    ThreadMembersUpdate(ThreadMembersUpdate),
    ThreadUpdate(Box<ThreadUpdate>),
    TypingStart(Box<TypingStart>),
    UnavailableGuild(UnavailableGuild),
    UserUpdate(UserUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    WebhooksUpdate(WebhooksUpdate),
}

impl DispatchEvent {
    /// Returns the type of event that this event is.
    pub const fn kind(&self) -> EventType {
        match self {
            Self::AutoModerationActionExecution(_) => EventType::AUTO_MODERATION_ACTION_EXECUTION,
            Self::AutoModerationRuleCreate(_) => EventType::AUTO_MODERATION_RULE_CREATE,
            Self::AutoModerationRuleDelete(_) => EventType::AUTO_MODERATION_RULE_DELETE,
            Self::AutoModerationRuleUpdate(_) => EventType::AUTO_MODERATION_RULE_UPDATE,
            Self::BanAdd(_) => EventType::BAN_ADD,
            Self::BanRemove(_) => EventType::BAN_REMOVE,
            Self::ChannelCreate(_) => EventType::CHANNEL_CREATE,
            Self::ChannelDelete(_) => EventType::CHANNEL_DELETE,
            Self::ChannelPinsUpdate(_) => EventType::CHANNEL_PINS_UPDATE,
            Self::ChannelUpdate(_) => EventType::CHANNEL_UPDATE,
            Self::CommandPermissionsUpdate(_) => EventType::COMMAND_PERMISSIONS_UPDATE,
            Self::GiftCodeUpdate => EventType::GIFT_CODE_UPDATE,
            Self::GuildAuditLogEntryCreate(_) => EventType::GUILD_AUDIT_LOG_ENTRY_CREATE,
            Self::GuildCreate(_) => EventType::GUILD_CREATE,
            Self::GuildDelete(_) => EventType::GUILD_DELETE,
            Self::GuildEmojisUpdate(_) => EventType::GUILD_EMOJIS_UPDATE,
            Self::GuildIntegrationsUpdate(_) => EventType::GUILD_INTEGRATIONS_UPDATE,
            Self::GuildScheduledEventCreate(_) => EventType::GUILD_SCHEDULED_EVENT_CREATE,
            Self::GuildScheduledEventDelete(_) => EventType::GUILD_SCHEDULED_EVENT_DELETE,
            Self::GuildScheduledEventUpdate(_) => EventType::GUILD_SCHEDULED_EVENT_UPDATE,
            Self::GuildScheduledEventUserAdd(_) => EventType::GUILD_SCHEDULED_EVENT_USER_ADD,
            Self::GuildScheduledEventUserRemove(_) => EventType::GUILD_SCHEDULED_EVENT_USER_REMOVE,
            Self::GuildStickersUpdate(_) => EventType::GUILD_STICKERS_UPDATE,
            Self::GuildUpdate(_) => EventType::GUILD_UPDATE,
            Self::IntegrationCreate(_) => EventType::INTEGRATION_CREATE,
            Self::IntegrationDelete(_) => EventType::INTEGRATION_DELETE,
            Self::IntegrationUpdate(_) => EventType::INTEGRATION_UPDATE,
            Self::InteractionCreate(_) => EventType::INTERACTION_CREATE,
            Self::InviteCreate(_) => EventType::INVITE_CREATE,
            Self::InviteDelete(_) => EventType::INVITE_DELETE,
            Self::MemberAdd(_) => EventType::MEMBER_ADD,
            Self::MemberRemove(_) => EventType::MEMBER_REMOVE,
            Self::MemberUpdate(_) => EventType::MEMBER_UPDATE,
            Self::MemberChunk(_) => EventType::MEMBER_CHUNK,
            Self::MessageCreate(_) => EventType::MESSAGE_CREATE,
            Self::MessageDelete(_) => EventType::MESSAGE_DELETE,
            Self::MessageDeleteBulk(_) => EventType::MESSAGE_DELETE_BULK,
            Self::MessageUpdate(_) => EventType::MESSAGE_UPDATE,
            Self::PresenceUpdate(_) => EventType::PRESENCE_UPDATE,
            Self::PresencesReplace => EventType::PRESENCES_REPLACE,
            Self::ReactionAdd(_) => EventType::REACTION_ADD,
            Self::ReactionRemove(_) => EventType::REACTION_REMOVE,
            Self::ReactionRemoveAll(_) => EventType::REACTION_REMOVE_ALL,
            Self::ReactionRemoveEmoji(_) => EventType::REACTION_REMOVE_EMOJI,
            Self::Ready(_) => EventType::READY,
            Self::Resumed => EventType::RESUMED,
            Self::RoleCreate(_) => EventType::ROLE_CREATE,
            Self::RoleDelete(_) => EventType::ROLE_DELETE,
            Self::RoleUpdate(_) => EventType::ROLE_UPDATE,
            Self::StageInstanceCreate(_) => EventType::STAGE_INSTANCE_CREATE,
            Self::StageInstanceDelete(_) => EventType::STAGE_INSTANCE_DELETE,
            Self::StageInstanceUpdate(_) => EventType::STAGE_INSTANCE_UPDATE,
            Self::ThreadCreate(_) => EventType::THREAD_CREATE,
            Self::ThreadDelete(_) => EventType::THREAD_DELETE,
            Self::ThreadListSync(_) => EventType::THREAD_LIST_SYNC,
            Self::ThreadMemberUpdate(_) => EventType::THREAD_MEMBER_UPDATE,
            Self::ThreadMembersUpdate(_) => EventType::THREAD_MEMBERS_UPDATE,
            Self::ThreadUpdate(_) => EventType::THREAD_UPDATE,
            Self::TypingStart(_) => EventType::TYPING_START,
            Self::UnavailableGuild(_) => EventType::UNAVAILABLE_GUILD,
            Self::UserUpdate(_) => EventType::USER_UPDATE,
            Self::VoiceServerUpdate(_) => EventType::VOICE_SERVER_UPDATE,
            Self::VoiceStateUpdate(_) => EventType::VOICE_STATE_UPDATE,
            Self::WebhooksUpdate(_) => EventType::WEBHOOKS_UPDATE,
        }
    }
}

impl TryFrom<Event> for DispatchEvent {
    type Error = EventConversionError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        Ok(match event {
            Event::AutoModerationActionExecution(v) => Self::AutoModerationActionExecution(v),
            Event::AutoModerationRuleCreate(v) => Self::AutoModerationRuleCreate(v),
            Event::AutoModerationRuleDelete(v) => Self::AutoModerationRuleDelete(v),
            Event::AutoModerationRuleUpdate(v) => Self::AutoModerationRuleUpdate(v),
            Event::BanAdd(v) => Self::BanAdd(v),
            Event::BanRemove(v) => Self::BanRemove(v),
            Event::ChannelCreate(v) => Self::ChannelCreate(v),
            Event::ChannelDelete(v) => Self::ChannelDelete(v),
            Event::ChannelPinsUpdate(v) => Self::ChannelPinsUpdate(v),
            Event::ChannelUpdate(v) => Self::ChannelUpdate(v),
            Event::CommandPermissionsUpdate(v) => Self::CommandPermissionsUpdate(v),
            Event::GiftCodeUpdate => Self::GiftCodeUpdate,
            Event::GuildAuditLogEntryCreate(v) => Self::GuildAuditLogEntryCreate(v),
            Event::GuildCreate(v) => Self::GuildCreate(v),
            Event::GuildDelete(v) => Self::GuildDelete(v),
            Event::GuildEmojisUpdate(v) => Self::GuildEmojisUpdate(v),
            Event::GuildIntegrationsUpdate(v) => Self::GuildIntegrationsUpdate(v),
            Event::GuildScheduledEventCreate(v) => Self::GuildScheduledEventCreate(v),
            Event::GuildScheduledEventDelete(v) => Self::GuildScheduledEventDelete(v),
            Event::GuildScheduledEventUpdate(v) => Self::GuildScheduledEventUpdate(v),
            Event::GuildScheduledEventUserAdd(v) => Self::GuildScheduledEventUserAdd(v),
            Event::GuildScheduledEventUserRemove(v) => Self::GuildScheduledEventUserRemove(v),
            Event::GuildUpdate(v) => Self::GuildUpdate(v),
            Event::IntegrationCreate(v) => Self::IntegrationCreate(v),
            Event::IntegrationDelete(v) => Self::IntegrationDelete(v),
            Event::IntegrationUpdate(v) => Self::IntegrationUpdate(v),
            Event::InteractionCreate(v) => Self::InteractionCreate(v),
            Event::InviteCreate(v) => Self::InviteCreate(v),
            Event::InviteDelete(v) => Self::InviteDelete(v),
            Event::MemberAdd(v) => Self::MemberAdd(v),
            Event::MemberRemove(v) => Self::MemberRemove(v),
            Event::MemberUpdate(v) => Self::MemberUpdate(v),
            Event::MemberChunk(v) => Self::MemberChunk(v),
            Event::MessageCreate(v) => Self::MessageCreate(v),
            Event::MessageDelete(v) => Self::MessageDelete(v),
            Event::MessageDeleteBulk(v) => Self::MessageDeleteBulk(v),
            Event::MessageUpdate(v) => Self::MessageUpdate(v),
            Event::PresenceUpdate(v) => Self::PresenceUpdate(v),
            Event::PresencesReplace => Self::PresencesReplace,
            Event::ReactionAdd(v) => Self::ReactionAdd(v),
            Event::ReactionRemove(v) => Self::ReactionRemove(v),
            Event::ReactionRemoveAll(v) => Self::ReactionRemoveAll(v),
            Event::ReactionRemoveEmoji(v) => Self::ReactionRemoveEmoji(v),
            Event::Ready(v) => Self::Ready(v),
            Event::Resumed => Self::Resumed,
            Event::RoleCreate(v) => Self::RoleCreate(v),
            Event::RoleDelete(v) => Self::RoleDelete(v),
            Event::RoleUpdate(v) => Self::RoleUpdate(v),
            Event::StageInstanceCreate(v) => Self::StageInstanceCreate(v),
            Event::StageInstanceDelete(v) => Self::StageInstanceDelete(v),
            Event::StageInstanceUpdate(v) => Self::StageInstanceUpdate(v),
            Event::ThreadCreate(v) => Self::ThreadCreate(v),
            Event::ThreadDelete(v) => Self::ThreadDelete(v),
            Event::ThreadListSync(v) => Self::ThreadListSync(v),
            Event::ThreadMemberUpdate(v) => Self::ThreadMemberUpdate(v),
            Event::ThreadMembersUpdate(v) => Self::ThreadMembersUpdate(v),
            Event::ThreadUpdate(v) => Self::ThreadUpdate(v),
            Event::TypingStart(v) => Self::TypingStart(v),
            Event::UnavailableGuild(v) => Self::UnavailableGuild(v),
            Event::UserUpdate(v) => Self::UserUpdate(v),
            Event::VoiceServerUpdate(v) => Self::VoiceServerUpdate(v),
            Event::VoiceStateUpdate(v) => Self::VoiceStateUpdate(v),
            Event::WebhooksUpdate(v) => Self::WebhooksUpdate(v),

            _ => return Err(EventConversionError::new(event)),
        })
    }
}

/// Deserialize into a [`DispatchEvent`] by knowing its event name.
///
/// An event name is something like `"CHANNEL_CREATE"` or `"GUILD_MEMBER_ADD"`.
#[derive(PartialEq, Eq)]
pub struct DispatchEventWithTypeDeserializer<'a>(&'a str);

impl<'a> DispatchEventWithTypeDeserializer<'a> {
    /// Create a new deserializer.
    pub const fn new(event_name: &'a str) -> Self {
        Self(event_name)
    }
}

impl<'de, 'a> DeserializeSeed<'de> for DispatchEventWithTypeDeserializer<'a> {
    type Value = DispatchEvent;

    #[allow(clippy::too_many_lines)]
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(match self.0 {
            "AUTO_MODERATION_ACTION_EXECUTION" => DispatchEvent::AutoModerationActionExecution(
                AutoModerationActionExecution::deserialize(deserializer)?,
            ),
            "AUTO_MODERATION_RULE_CREATE" => DispatchEvent::AutoModerationRuleCreate(Box::new(
                AutoModerationRuleCreate::deserialize(deserializer)?,
            )),
            "AUTO_MODERATION_RULE_DELETE" => DispatchEvent::AutoModerationRuleDelete(Box::new(
                AutoModerationRuleDelete::deserialize(deserializer)?,
            )),
            "AUTO_MODERATION_RULE_UPDATE" => DispatchEvent::AutoModerationRuleUpdate(Box::new(
                AutoModerationRuleUpdate::deserialize(deserializer)?,
            )),
            "CHANNEL_CREATE" => {
                DispatchEvent::ChannelCreate(Box::new(ChannelCreate::deserialize(deserializer)?))
            }
            "CHANNEL_DELETE" => {
                DispatchEvent::ChannelDelete(Box::new(ChannelDelete::deserialize(deserializer)?))
            }
            "CHANNEL_PINS_UPDATE" => {
                DispatchEvent::ChannelPinsUpdate(ChannelPinsUpdate::deserialize(deserializer)?)
            }
            "CHANNEL_UPDATE" => {
                DispatchEvent::ChannelUpdate(Box::new(ChannelUpdate::deserialize(deserializer)?))
            }
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => DispatchEvent::CommandPermissionsUpdate(
                CommandPermissionsUpdate::deserialize(deserializer)?,
            ),
            "GIFT_CODE_UPDATE" => {
                deserializer.deserialize_ignored_any(IgnoredAny)?;

                DispatchEvent::GiftCodeUpdate
            }
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => DispatchEvent::GuildAuditLogEntryCreate(Box::new(
                GuildAuditLogEntryCreate::deserialize(deserializer)?,
            )),
            "GUILD_BAN_ADD" => DispatchEvent::BanAdd(BanAdd::deserialize(deserializer)?),
            "GUILD_BAN_REMOVE" => DispatchEvent::BanRemove(BanRemove::deserialize(deserializer)?),
            "GUILD_CREATE" => {
                DispatchEvent::GuildCreate(Box::new(GuildCreate::deserialize(deserializer)?))
            }
            "GUILD_DELETE" => DispatchEvent::GuildDelete(GuildDelete::deserialize(deserializer)?),
            "GUILD_EMOJIS_UPDATE" => {
                DispatchEvent::GuildEmojisUpdate(GuildEmojisUpdate::deserialize(deserializer)?)
            }
            "GUILD_INTEGRATIONS_UPDATE" => DispatchEvent::GuildIntegrationsUpdate(
                GuildIntegrationsUpdate::deserialize(deserializer)?,
            ),
            "GUILD_SCHEDULED_EVENT_CREATE" => DispatchEvent::GuildScheduledEventCreate(Box::new(
                GuildScheduledEventCreate::deserialize(deserializer)?,
            )),
            "GUILD_SCHEDULED_EVENT_DELETE" => DispatchEvent::GuildScheduledEventDelete(Box::new(
                GuildScheduledEventDelete::deserialize(deserializer)?,
            )),
            "GUILD_SCHEDULED_EVENT_UPDATE" => DispatchEvent::GuildScheduledEventUpdate(Box::new(
                GuildScheduledEventUpdate::deserialize(deserializer)?,
            )),
            "GUILD_SCHEDULED_EVENT_USER_ADD" => DispatchEvent::GuildScheduledEventUserAdd(
                GuildScheduledEventUserAdd::deserialize(deserializer)?,
            ),
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => DispatchEvent::GuildScheduledEventUserRemove(
                GuildScheduledEventUserRemove::deserialize(deserializer)?,
            ),
            "GUILD_MEMBERS_CHUNK" => {
                DispatchEvent::MemberChunk(MemberChunk::deserialize(deserializer)?)
            }
            "GUILD_MEMBER_ADD" => {
                DispatchEvent::MemberAdd(Box::new(MemberAdd::deserialize(deserializer)?))
            }
            "GUILD_MEMBER_REMOVE" => {
                DispatchEvent::MemberRemove(MemberRemove::deserialize(deserializer)?)
            }
            "GUILD_MEMBER_UPDATE" => {
                DispatchEvent::MemberUpdate(Box::new(MemberUpdate::deserialize(deserializer)?))
            }
            "GUILD_ROLE_CREATE" => {
                DispatchEvent::RoleCreate(RoleCreate::deserialize(deserializer)?)
            }
            "GUILD_ROLE_DELETE" => {
                DispatchEvent::RoleDelete(RoleDelete::deserialize(deserializer)?)
            }
            "GUILD_ROLE_UPDATE" => {
                DispatchEvent::RoleUpdate(RoleUpdate::deserialize(deserializer)?)
            }
            "GUILD_STICKERS_UPDATE" => {
                DispatchEvent::GuildStickersUpdate(GuildStickersUpdate::deserialize(deserializer)?)
            }
            "GUILD_UPDATE" => {
                DispatchEvent::GuildUpdate(Box::new(GuildUpdate::deserialize(deserializer)?))
            }
            "INTEGRATION_CREATE" => DispatchEvent::IntegrationCreate(Box::new(
                IntegrationCreate::deserialize(deserializer)?,
            )),
            "INTEGRATION_DELETE" => {
                DispatchEvent::IntegrationDelete(IntegrationDelete::deserialize(deserializer)?)
            }
            "INTEGRATION_UPDATE" => DispatchEvent::IntegrationUpdate(Box::new(
                IntegrationUpdate::deserialize(deserializer)?,
            )),
            "INTERACTION_CREATE" => DispatchEvent::InteractionCreate(Box::new(
                InteractionCreate::deserialize(deserializer)?,
            )),
            "INVITE_CREATE" => {
                DispatchEvent::InviteCreate(Box::new(InviteCreate::deserialize(deserializer)?))
            }
            "INVITE_DELETE" => {
                DispatchEvent::InviteDelete(InviteDelete::deserialize(deserializer)?)
            }
            "MESSAGE_CREATE" => {
                DispatchEvent::MessageCreate(Box::new(MessageCreate::deserialize(deserializer)?))
            }
            "MESSAGE_DELETE" => {
                DispatchEvent::MessageDelete(MessageDelete::deserialize(deserializer)?)
            }
            "MESSAGE_DELETE_BULK" => {
                DispatchEvent::MessageDeleteBulk(MessageDeleteBulk::deserialize(deserializer)?)
            }
            "MESSAGE_REACTION_ADD" => {
                DispatchEvent::ReactionAdd(Box::new(ReactionAdd::deserialize(deserializer)?))
            }
            "MESSAGE_REACTION_REMOVE" => {
                DispatchEvent::ReactionRemove(Box::new(ReactionRemove::deserialize(deserializer)?))
            }
            "MESSAGE_REACTION_REMOVE_EMOJI" => {
                DispatchEvent::ReactionRemoveEmoji(ReactionRemoveEmoji::deserialize(deserializer)?)
            }
            "MESSAGE_REACTION_REMOVE_ALL" => {
                DispatchEvent::ReactionRemoveAll(ReactionRemoveAll::deserialize(deserializer)?)
            }
            "MESSAGE_UPDATE" => {
                DispatchEvent::MessageUpdate(Box::new(MessageUpdate::deserialize(deserializer)?))
            }
            "PRESENCE_UPDATE" => {
                DispatchEvent::PresenceUpdate(Box::new(PresenceUpdate::deserialize(deserializer)?))
            }
            "PRESENCES_REPLACE" => {
                deserializer.deserialize_ignored_any(IgnoredAny)?;

                DispatchEvent::PresencesReplace
            }
            "READY" => DispatchEvent::Ready(Box::new(Ready::deserialize(deserializer)?)),
            "RESUMED" => {
                deserializer.deserialize_ignored_any(IgnoredAny)?;

                DispatchEvent::Resumed
            }
            "STAGE_INSTANCE_CREATE" => {
                DispatchEvent::StageInstanceCreate(StageInstanceCreate::deserialize(deserializer)?)
            }
            "STAGE_INSTANCE_DELETE" => {
                DispatchEvent::StageInstanceDelete(StageInstanceDelete::deserialize(deserializer)?)
            }
            "STAGE_INSTANCE_UPDATE" => {
                DispatchEvent::StageInstanceUpdate(StageInstanceUpdate::deserialize(deserializer)?)
            }
            "THREAD_CREATE" => {
                DispatchEvent::ThreadCreate(Box::new(ThreadCreate::deserialize(deserializer)?))
            }
            "THREAD_DELETE" => {
                DispatchEvent::ThreadDelete(ThreadDelete::deserialize(deserializer)?)
            }
            "THREAD_LIST_SYNC" => {
                DispatchEvent::ThreadListSync(ThreadListSync::deserialize(deserializer)?)
            }
            "THREAD_MEMBER_UPDATE" => DispatchEvent::ThreadMemberUpdate(Box::new(
                ThreadMemberUpdate::deserialize(deserializer)?,
            )),
            "THREAD_MEMBERS_UPDATE" => {
                DispatchEvent::ThreadMembersUpdate(ThreadMembersUpdate::deserialize(deserializer)?)
            }
            "THREAD_UPDATE" => {
                DispatchEvent::ThreadUpdate(Box::new(ThreadUpdate::deserialize(deserializer)?))
            }
            "TYPING_START" => {
                DispatchEvent::TypingStart(Box::new(TypingStart::deserialize(deserializer)?))
            }
            "USER_UPDATE" => DispatchEvent::UserUpdate(UserUpdate::deserialize(deserializer)?),
            "VOICE_SERVER_UPDATE" => {
                DispatchEvent::VoiceServerUpdate(VoiceServerUpdate::deserialize(deserializer)?)
            }
            "VOICE_STATE_UPDATE" => DispatchEvent::VoiceStateUpdate(Box::new(
                VoiceStateUpdate::deserialize(deserializer)?,
            )),
            "WEBHOOKS_UPDATE" => {
                DispatchEvent::WebhooksUpdate(WebhooksUpdate::deserialize(deserializer)?)
            }
            other => return Err(DeError::unknown_variant(other, &[])),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{DispatchEvent, DispatchEventWithTypeDeserializer};
    use serde::de::DeserializeSeed;
    use serde_json::Deserializer;

    #[test]
    fn gift_code_update() {
        // Input will be ignored so long as it's valid JSON.
        let input = r#"{
            "a": "b"
        }"#;

        let deserializer = DispatchEventWithTypeDeserializer::new("GIFT_CODE_UPDATE");
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert_eq!(event, DispatchEvent::GiftCodeUpdate);
    }
}
