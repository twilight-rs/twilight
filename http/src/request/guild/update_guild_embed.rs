use crate::request::prelude::*;
use dawn_model::{
    guild::GuildEmbed,
    id::{ChannelId, GuildId},
};

#[derive(Default, Serialize)]
struct UpdateGuildEmbedFields {
    channel_id: Option<ChannelId>,
    enabled: Option<bool>,
}

pub struct UpdateGuildEmbed<'a> {
    fields: UpdateGuildEmbedFields,
    fut: Option<Pending<'a, GuildEmbed>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateGuildEmbed<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: UpdateGuildEmbedFields::default(),
            fut: None,
            guild_id,
            http,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.fields.enabled.replace(enabled);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateGuildEmbed {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateGuildEmbed<'_>, GuildEmbed);
