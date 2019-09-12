use dawn_model::{
    guild::GuildEmbed,
    id::{ChannelId, GuildId},
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateGuildEmbed<'a> {
    channel_id: Option<ChannelId>,
    enabled: Option<bool>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, GuildEmbed>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> UpdateGuildEmbed<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            channel_id: None,
            enabled: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.channel_id.replace(channel_id.into());

        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled.replace(enabled);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            body: Some(serde_json::to_vec(self)?),
            route: Route::UpdateGuildEmbed {
                guild_id: self.guild_id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(UpdateGuildEmbed<'_>, GuildEmbed);
