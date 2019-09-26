use async_trait::async_trait;
use dawn_model::{
    channel::Channel,
    gateway::presence::Presence,
    guild::{Emoji, Guild, Member, Role},
    id::{ChannelId, EmojiId, GuildId, RoleId, UserId},
    user::{CurrentUser, User},
    voice::VoiceState,
};
use std::fmt::Debug;

#[async_trait]
pub trait Cache {
    type Error: Debug;
    type Channel;
    type CurrentUser;
    type Emoji;
    type Guild;
    type Member;
    type Presence;
    type Role;
    type User;
    type VoiceState;

    async fn channel(&self, channel_id: ChannelId) -> Result<Option<Self::Channel>, Self::Error>;
    async fn delete_channel(&self, channel_id: ChannelId) -> Result<Option<Self::Channel>, Self::Error>;
    async fn update_channel(&self, channel_id: ChannelId, channel: Channel) -> Result<(), Self::Error>;

    async fn current_user(&self) -> Result<Self::CurrentUser, Self::Error>;
    async fn delete_current_user(&self) -> Result<Self::CurrentUser, Self::Error>;
    async fn update_current_user(&self, current_user: CurrentUser) -> Result<(), Self::Error>;

    async fn emoji(&self, emoji_id: EmojiId) -> Result<Self::Emoji, Self::Error>;
    async fn delete_emoji(&self, emoji_id: EmojiId) -> Result<Self::Emoji, Self::Error>;
    async fn update_emoji(&self, emoji_id: EmojiId, emoji: Emoji) -> Result<(), Self::Error>;

    async fn guild(&self, guild_id: GuildId) -> Result<Self::Guild, Self::Error>;
    async fn delete_guild(&self, guild_id: GuildId) -> Result<Self::Guild, Self::Error>;
    async fn update_guild(&self, guild_id: GuildId, guild: Guild) -> Result<(), Self::Error>;

    async fn member(&self, guild_id: GuildId, user_id: UserId) -> Result<Self::Member, Self::Error>;
    async fn delete_member(&self, guild_id: GuildId, user_id: UserId) -> Result<Self::Member, Self::Error>;
    async fn update_member(&self, guild_id: GuildId, user_id: UserId, member: Member) -> Result<(), Self::Error>;

    async fn presence(&self, guild_id: Option<GuildId>, user_id: UserId) -> Result<Self::Presence, Self::Error>;
    async fn delete_presence(&self, guild_id: Option<GuildId>, user_id: UserId) -> Result<Self::Presence, Self::Error>;
    async fn update_presence(&self, guild_id: Option<GuildId>, user_id: UserId, presence: Presence) -> Result<(), Self::Error>;

    async fn role(&self, role_id: RoleId) -> Result<Self::Role, Self::Error>;
    async fn delete_role(&self, role_id: RoleId) -> Result<Self::Role, Self::Error>;
    async fn update_role(&self, role_id: RoleId, role: Role) -> Result<(), Self::Error>;

    async fn user(&self, user_id: UserId) -> Result<Self::User, Self::Error>;
    async fn delete_user(&self, user_id: UserId) -> Result<Self::User, Self::Error>;
    async fn update_user(&self, user_id: UserId, user: User) -> Result<(), Self::Error>;

    async fn voice_state(&self, channel_id: ChannelId, user_id: UserId) -> Result<Self::VoiceState, Self::Error>;
    async fn delete_voice_state(&self, channel_id: ChannelId, user_id: UserId) -> Result<Self::VoiceState, Self::Error>;
    async fn update_voice_state(&self, channel_id: ChannelId, user_id: UserId, voice_state: VoiceState) -> Result<(), Self::Error>;

    async fn clear(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait UpdateCache<T> {
    type Error: Debug;

    async fn update(&self, item: &T) -> Result<(), Self::Error>;
}
