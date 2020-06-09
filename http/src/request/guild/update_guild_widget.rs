use crate::json_to_vec;
use crate::request::prelude::*;
use twilight_model::{
    guild::GuildWidget,
    id::{ChannelId, GuildId},
};

#[derive(Default, Serialize)]
struct UpdateGuildWidgetFields {
    channel_id: Option<ChannelId>,
    enabled: Option<bool>,
}

pub struct UpdateGuildWidget<'a> {
    fields: UpdateGuildWidgetFields,
    fut: Option<Pending<'a, GuildWidget>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateGuildWidget<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: UpdateGuildWidgetFields::default(),
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
            json_to_vec(&self.fields)?,
            Route::UpdateGuildWidget {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateGuildWidget<'_>, GuildWidget);
