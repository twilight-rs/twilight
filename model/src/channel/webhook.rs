use crate::{
    id::{ChannelId, GuildId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Webhook {
    pub id: WebhookId,
    pub avatar: Option<String>,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub name: Option<String>,
    pub token: String,
    pub user: Option<User>,
}
