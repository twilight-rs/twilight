use crate::{
    channel::WebhookType,
    id::{ChannelId, GuildId, WebhookId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Webhook {
    pub id: WebhookId,
    #[cfg_attr(
        feature = "serde-support",
        serde(default = "WebhookType::default", rename = "type")
    )]
    pub kind: WebhookType,
    pub avatar: Option<String>,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub name: Option<String>,
    pub token: Option<String>,
    pub user: Option<User>,
}
