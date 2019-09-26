use super::prelude::*;
use dawn_model::{
    guild::GuildEmbed,
    id::{ChannelId, GuildId},
};

#[derive(Serialize)]
pub struct UpdateGuildEmbed<'a> {
    channel_id: Option<ChannelId>,
    enabled: Option<bool>,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<GuildEmbed>> + Send + 'a>>>,
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
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdateGuildEmbed {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateGuildEmbed<'_>, GuildEmbed);
