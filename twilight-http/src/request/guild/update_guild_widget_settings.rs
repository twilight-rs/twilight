use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::widget::GuildWidgetSettings,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct UpdateGuildWidgetSettingsFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

/// Modify a guild's widget settings.
///
/// See [Discord Docs/Modify Guild Widget].
///
/// [Discord Docs/Modify Guild Widget]: https://discord.com/developers/docs/resources/guild#modify-guild-widget
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildWidgetSettings<'a> {
    fields: UpdateGuildWidgetSettingsFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateGuildWidgetSettings<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: UpdateGuildWidgetSettingsFields {
                channel_id: None,
                enabled: None,
            },
            guild_id,
            http,
            reason: Ok(None),
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
}

impl<'a> AuditLogReason<'a> for UpdateGuildWidgetSettings<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuildWidgetSettings<'_> {
    type Output = Result<Response<GuildWidgetSettings>, Error>;

    type IntoFuture = ResponseFuture<GuildWidgetSettings>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildWidgetSettings<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateGuildWidgetSettings {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
