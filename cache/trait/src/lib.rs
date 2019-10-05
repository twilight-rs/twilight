use async_trait::async_trait;
use dawn_model::id::{ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId};
use std::fmt::Debug;

#[async_trait]
pub trait Cache {
    type Error: Debug;
    type CurrentUser;
    type Emoji;
    type Group;
    type Guild;
    type GuildChannel;
    type Member;
    type Message;
    type Presence;
    type PrivateChannel;
    type Role;
    type User;
    type VoiceState;

    async fn guild_channel(
        &self,
        channel_id: ChannelId,
    ) -> Result<Option<Self::GuildChannel>, Self::Error>;

    async fn current_user(&self) -> Result<Option<Self::CurrentUser>, Self::Error>;

    async fn emoji(&self, emoji_id: EmojiId) -> Result<Option<Self::Emoji>, Self::Error>;

    async fn group(&self, channel_id: ChannelId) -> Result<Option<Self::Group>, Self::Error>;

    async fn guild(&self, guild_id: GuildId) -> Result<Option<Self::Guild>, Self::Error>;

    async fn member(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<Option<Self::Member>, Self::Error>;

    async fn message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<Option<Self::Message>, Self::Error>;

    async fn presence(
        &self,
        guild_id: Option<GuildId>,
        user_id: UserId,
    ) -> Result<Option<Self::Presence>, Self::Error>;

    async fn private_channel(
        &self,
        channel_id: ChannelId,
    ) -> Result<Option<Self::PrivateChannel>, Self::Error>;

    async fn role(&self, role_id: RoleId) -> Result<Option<Self::Role>, Self::Error>;

    async fn user(&self, user_id: UserId) -> Result<Option<Self::User>, Self::Error>;

    async fn voice_state(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<Option<Self::VoiceState>, Self::Error>;

    async fn clear(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait UpdateCache<T>: Cache {
    async fn update(&self, item: &T) -> Result<(), Self::Error>;
}
