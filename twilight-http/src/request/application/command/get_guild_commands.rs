use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

/// Fetch all commands for a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommands<'a> {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    with_localizations: Option<bool>,
}

impl<'a> GetGuildCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            application_id,
            guild_id,
            http,
            with_localizations: None,
        }
    }

    /// Whether to include full localization dictionaries in the response.
    ///
    /// Defaults to [`false`].
    pub const fn with_localizations(mut self, with_localizations: bool) -> Self {
        self.with_localizations = Some(with_localizations);

        self
    }
}

impl IntoFuture for GetGuildCommands<'_> {
    type Output = Result<Response<ListBody<Command>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Command>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildCommands {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
            with_localizations: self.with_localizations,
        }))
    }
}
