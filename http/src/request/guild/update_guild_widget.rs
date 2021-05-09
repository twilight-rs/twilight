use crate::request::prelude::*;
use twilight_model::{
    guild::GuildWidget,
    id::{ChannelId, GuildId},
};

#[derive(Default, Serialize)]
struct UpdateGuildWidgetFields {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Option<ChannelId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

/// Modify the guild widget.
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

    /// Set which channel to display on the widget.
    pub fn channel_id(mut self, channel_id: impl Into<Option<ChannelId>>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

        self
    }

    /// Set to true to enable the guild widget.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.fields.enabled.replace(enabled);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::UpdateGuildWidget {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateGuildWidget<'_>, GuildWidget);
