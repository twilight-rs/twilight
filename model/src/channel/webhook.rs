use crate::{
    id::{ChannelId, GuildId, WebhookId},
    user::User,
};
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Webhook {
    pub id: WebhookId,
    pub avatar: Option<String>,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub name: Option<String>,
    pub token: String,
    pub user: Option<User>,
}

impl Hash for Webhook {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
