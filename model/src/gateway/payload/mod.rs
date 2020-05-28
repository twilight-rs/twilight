pub mod identify;
pub mod reaction_remove_emoji;
pub mod resume;
pub mod update_status;

mod ban_add;
mod ban_remove;
mod channel_create;
mod channel_delete;
mod channel_pins_update;
mod channel_update;
mod guild_create;
mod guild_delete;
mod guild_emojis_update;
mod guild_integrations_update;
mod guild_update;
mod heartbeat;
mod invite_create;
mod invite_delete;
mod member_add;
mod member_chunk;
mod member_remove;
mod member_update;
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_update;
mod presence_update;
mod reaction_add;
mod reaction_remove;
mod reaction_remove_all;
mod ready;
mod request_guild_members;
mod role_create;
mod role_delete;
mod role_update;
mod typing_start;
mod unavailable_guild;
mod update_voice_state;
mod user_update;
mod voice_server_update;
mod voice_state_update;
mod webhooks_update;

pub use self::{
    ban_add::BanAdd, ban_remove::BanRemove, channel_create::ChannelCreate,
    channel_delete::ChannelDelete, channel_pins_update::ChannelPinsUpdate,
    channel_update::ChannelUpdate, guild_create::GuildCreate, guild_delete::GuildDelete,
    guild_emojis_update::GuildEmojisUpdate, guild_integrations_update::GuildIntegrationsUpdate,
    guild_update::GuildUpdate, heartbeat::Heartbeat, invite_create::InviteCreate,
    invite_delete::InviteDelete, member_add::MemberAdd, member_chunk::MemberChunk,
    member_remove::MemberRemove, member_update::MemberUpdate, message_create::MessageCreate,
    message_delete::MessageDelete, message_delete_bulk::MessageDeleteBulk,
    message_update::MessageUpdate, presence_update::PresenceUpdate, reaction_add::ReactionAdd,
    reaction_remove::ReactionRemove, reaction_remove_all::ReactionRemoveAll,
    reaction_remove_emoji::ReactionRemoveEmoji, ready::Ready,
    request_guild_members::RequestGuildMembers, role_create::RoleCreate, role_delete::RoleDelete,
    role_update::RoleUpdate, typing_start::TypingStart, unavailable_guild::UnavailableGuild,
    update_status::UpdateStatus, update_voice_state::UpdateVoiceState, user_update::UserUpdate,
    voice_server_update::VoiceServerUpdate, voice_state_update::VoiceStateUpdate,
    webhooks_update::WebhooksUpdate,
};

/// A dispatch event, containing information about a created guild, a member
/// added, etc.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    BanAdd(BanAdd),
    BanRemove(BanRemove),
    ChannelCreate(ChannelCreate),
    ChannelDelete(ChannelDelete),
    ChannelPinsUpdate(ChannelPinsUpdate),
    ChannelUpdate(ChannelUpdate),
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

impl Event {
    pub fn kind(&self) -> EventType {
        match self {
            Self::BanAdd(_) => EventType::BanAdd,
            Self::BanRemove(_) => EventType::BanRemove,
            Self::ChannelCreate(_) => EventType::ChannelCreate,
            Self::ChannelDelete(_) => EventType::ChannelDelete,
            Self::ChannelPinsUpdate(_) => EventType::ChannelPinsUpdate,
            Self::ChannelUpdate(_) => EventType::ChannelUpdate,
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

/// The type of an event.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EventType {
    BanAdd,
    BanRemove,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsUpdate,
    ChannelUpdate,
    GuildCreate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildUpdate,
    InviteCreate,
    InviteDelete,
    MemberAdd,
    MemberRemove,
    MemberUpdate,
    MemberChunk,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessageUpdate,
    PresenceUpdate,
    PresencesReplace,
    ReactionAdd,
    ReactionRemove,
    ReactionRemoveAll,
    ReactionRemoveEmoji,
    Ready,
    Resumed,
    RoleCreate,
    RoleDelete,
    RoleUpdate,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
}
