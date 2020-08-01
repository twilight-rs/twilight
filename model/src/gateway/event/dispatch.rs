use super::{super::payload::*, Event, EventConversionError, EventType};
use serde::{
    de::{Deserialize, DeserializeSeed, Deserializer, Error as DeError, IgnoredAny},
    Serialize,
};
use std::convert::TryFrom;

/// A dispatch event, containing information about a created guild, a member
/// added, etc.
///
/// You can deserialize into a `DispatchEvent` via
/// [`DispatchEventWithTypeDeserializer`].
///
/// [`DispatchEventWithTypeDeserializer`]: struct.DispatchEventWithTypeDeserializer.html
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum DispatchEvent {
    BanAdd(BanAdd),
    BanRemove(BanRemove),
    ChannelCreate(ChannelCreate),
    ChannelDelete(ChannelDelete),
    ChannelPinsUpdate(ChannelPinsUpdate),
    ChannelUpdate(ChannelUpdate),
    GiftCodeUpdate,
    GuildCreate(Box<GuildCreate>),
    GuildDelete(Box<GuildDelete>),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildUpdate(Box<GuildUpdate>),
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
    TypingStart(Box<TypingStart>),
    UnavailableGuild(UnavailableGuild),
    UserUpdate(UserUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    WebhooksUpdate(WebhooksUpdate),
}

impl DispatchEvent {
    /// Returns the type of event that this event is.
    pub fn kind(&self) -> EventType {
        match self {
            Self::BanAdd(_) => EventType::BanAdd,
            Self::BanRemove(_) => EventType::BanRemove,
            Self::ChannelCreate(_) => EventType::ChannelCreate,
            Self::ChannelDelete(_) => EventType::ChannelDelete,
            Self::ChannelPinsUpdate(_) => EventType::ChannelPinsUpdate,
            Self::ChannelUpdate(_) => EventType::ChannelUpdate,
            Self::GiftCodeUpdate => EventType::GiftCodeUpdate,
            Self::GuildCreate(_) => EventType::GuildCreate,
            Self::GuildDelete(_) => EventType::GuildDelete,
            Self::GuildEmojisUpdate(_) => EventType::GuildEmojisUpdate,
            Self::GuildIntegrationsUpdate(_) => EventType::GuildIntegrationsUpdate,
            Self::GuildUpdate(_) => EventType::GuildUpdate,
            Self::InviteCreate(_) => EventType::InviteCreate,
            Self::InviteDelete(_) => EventType::InviteDelete,
            Self::MemberAdd(_) => EventType::MemberAdd,
            Self::MemberRemove(_) => EventType::MemberRemove,
            Self::MemberUpdate(_) => EventType::MemberUpdate,
            Self::MemberChunk(_) => EventType::MemberChunk,
            Self::MessageCreate(_) => EventType::MessageCreate,
            Self::MessageDelete(_) => EventType::MessageDelete,
            Self::MessageDeleteBulk(_) => EventType::MessageDeleteBulk,
            Self::MessageUpdate(_) => EventType::MessageUpdate,
            Self::PresenceUpdate(_) => EventType::PresenceUpdate,
            Self::PresencesReplace => EventType::PresencesReplace,
            Self::ReactionAdd(_) => EventType::ReactionAdd,
            Self::ReactionRemove(_) => EventType::ReactionRemove,
            Self::ReactionRemoveAll(_) => EventType::ReactionRemoveAll,
            Self::ReactionRemoveEmoji(_) => EventType::ReactionRemoveEmoji,
            Self::Ready(_) => EventType::Ready,
            Self::Resumed => EventType::Resumed,
            Self::RoleCreate(_) => EventType::RoleCreate,
            Self::RoleDelete(_) => EventType::RoleDelete,
            Self::RoleUpdate(_) => EventType::RoleUpdate,
            Self::TypingStart(_) => EventType::TypingStart,
            Self::UnavailableGuild(_) => EventType::UnavailableGuild,
            Self::UserUpdate(_) => EventType::UserUpdate,
            Self::VoiceServerUpdate(_) => EventType::VoiceServerUpdate,
            Self::VoiceStateUpdate(_) => EventType::VoiceStateUpdate,
            Self::WebhooksUpdate(_) => EventType::WebhooksUpdate,
        }
    }
}

impl TryFrom<Event> for DispatchEvent {
    type Error = EventConversionError<Event>;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::BanAdd(v) => Ok(Self::BanAdd(v)),
            Event::BanRemove(v) => Ok(Self::BanRemove(v)),
            Event::ChannelCreate(v) => Ok(Self::ChannelCreate(v)),
            Event::ChannelDelete(v) => Ok(Self::ChannelDelete(v)),
            Event::ChannelPinsUpdate(v) => Ok(Self::ChannelPinsUpdate(v)),
            Event::ChannelUpdate(v) => Ok(Self::ChannelUpdate(v)),
            Event::GiftCodeUpdate => Ok(Self::GiftCodeUpdate),
            Event::GuildCreate(v) => Ok(Self::GuildCreate(v)),
            Event::GuildDelete(v) => Ok(Self::GuildDelete(v)),
            Event::GuildEmojisUpdate(v) => Ok(Self::GuildEmojisUpdate(v)),
            Event::GuildIntegrationsUpdate(v) => Ok(Self::GuildIntegrationsUpdate(v)),
            Event::GuildUpdate(v) => Ok(Self::GuildUpdate(v)),
            Event::InviteCreate(v) => Ok(Self::InviteCreate(v)),
            Event::InviteDelete(v) => Ok(Self::InviteDelete(v)),
            Event::MemberAdd(v) => Ok(Self::MemberAdd(v)),
            Event::MemberRemove(v) => Ok(Self::MemberRemove(v)),
            Event::MemberUpdate(v) => Ok(Self::MemberUpdate(v)),
            Event::MemberChunk(v) => Ok(Self::MemberChunk(v)),
            Event::MessageCreate(v) => Ok(Self::MessageCreate(v)),
            Event::MessageDelete(v) => Ok(Self::MessageDelete(v)),
            Event::MessageDeleteBulk(v) => Ok(Self::MessageDeleteBulk(v)),
            Event::MessageUpdate(v) => Ok(Self::MessageUpdate(v)),
            Event::PresenceUpdate(v) => Ok(Self::PresenceUpdate(v)),
            Event::PresencesReplace => Ok(Self::PresencesReplace),
            Event::ReactionAdd(v) => Ok(Self::ReactionAdd(v)),
            Event::ReactionRemove(v) => Ok(Self::ReactionRemove(v)),
            Event::ReactionRemoveAll(v) => Ok(Self::ReactionRemoveAll(v)),
            Event::ReactionRemoveEmoji(v) => Ok(Self::ReactionRemoveEmoji(v)),
            Event::Ready(v) => Ok(Self::Ready(v)),
            Event::Resumed => Ok(Self::Resumed),
            Event::RoleCreate(v) => Ok(Self::RoleCreate(v)),
            Event::RoleDelete(v) => Ok(Self::RoleDelete(v)),
            Event::RoleUpdate(v) => Ok(Self::RoleUpdate(v)),
            Event::TypingStart(v) => Ok(Self::TypingStart(v)),
            Event::UnavailableGuild(v) => Ok(Self::UnavailableGuild(v)),
            Event::UserUpdate(v) => Ok(Self::UserUpdate(v)),
            Event::VoiceServerUpdate(v) => Ok(Self::VoiceServerUpdate(v)),
            Event::VoiceStateUpdate(v) => Ok(Self::VoiceStateUpdate(v)),
            Event::WebhooksUpdate(v) => Ok(Self::WebhooksUpdate(v)),

            _ => Err(EventConversionError::new(
                event,
                "event is not a DispatchEvent",
            )),
        }
    }
}

/// Deserialize into a [`DispatchEvent`] by knowing its event name.
///
/// An event name is something like `"CHANNEL_CREATE"` or `"GUILD_MEMBER_ADD"`.
///
/// [`DispatchEvent`]: enum.DispatchEvent.html
#[derive(PartialEq, Eq)]
pub struct DispatchEventWithTypeDeserializer<'a>(&'a str);

impl<'a> DispatchEventWithTypeDeserializer<'a> {
    /// Create a new deserializer.
    pub fn new(event_name: &'a str) -> Self {
        Self(event_name)
    }
}

impl<'de, 'a> DeserializeSeed<'de> for DispatchEventWithTypeDeserializer<'a> {
    type Value = DispatchEvent;

    #[allow(clippy::too_many_lines)]
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(match self.0 {
            "CHANNEL_CREATE" => {
                DispatchEvent::ChannelCreate(ChannelCreate::deserialize(deserializer)?)
            }
            "CHANNEL_DELETE" => {
                DispatchEvent::ChannelDelete(ChannelDelete::deserialize(deserializer)?)
            }
            "CHANNEL_PINS_UPDATE" => {
                DispatchEvent::ChannelPinsUpdate(ChannelPinsUpdate::deserialize(deserializer)?)
            }
            "CHANNEL_UPDATE" => {
                DispatchEvent::ChannelUpdate(ChannelUpdate::deserialize(deserializer)?)
            }
            "GUILD_BAN_ADD" => DispatchEvent::BanAdd(BanAdd::deserialize(deserializer)?),
            "GUILD_BAN_REMOVE" => DispatchEvent::BanRemove(BanRemove::deserialize(deserializer)?),
            "GUILD_CREATE" => {
                DispatchEvent::GuildCreate(Box::new(GuildCreate::deserialize(deserializer)?))
            }
            "GUILD_DELETE" => {
                DispatchEvent::GuildDelete(Box::new(GuildDelete::deserialize(deserializer)?))
            }
            "GUILD_EMOJIS_UPDATE" => {
                DispatchEvent::GuildEmojisUpdate(GuildEmojisUpdate::deserialize(deserializer)?)
            }
            "GUILD_INTEGRATIONS_UPDATE" => DispatchEvent::GuildIntegrationsUpdate(
                GuildIntegrationsUpdate::deserialize(deserializer)?,
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
            "GUILD_UPDATE" => {
                DispatchEvent::GuildUpdate(Box::new(GuildUpdate::deserialize(deserializer)?))
            }
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
