use crate::{
    client::Client,
    error::Error,
    request::{Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildWidget,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

#[derive(Serialize)]
struct UpdateGuildWidgetFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

/// Modify the guild widget.
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildWidget<'a> {
    fields: UpdateGuildWidgetFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> UpdateGuildWidget<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: UpdateGuildWidgetFields {
                channel_id: None,
                enabled: None,
            },
            guild_id,
            http,
        }
    }

    /// Set which channel to display on the widget.
    pub const fn channel_id(mut self, channel_id: Option<Id<ChannelMarker>>) -> Self {
        self.fields.channel_id = Some(Nullable(channel_id));

        self
    }

    /// Set to true to enable the guild widget.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.fields.enabled = Some(enabled);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<GuildWidget> {
        self.into_future()
    }
}

impl IntoFuture for UpdateGuildWidget<'_> {
    type Output = Result<Response<GuildWidget>, Error>;

    type IntoFuture = ResponseFuture<GuildWidget>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildWidget<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateGuildWidget {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
