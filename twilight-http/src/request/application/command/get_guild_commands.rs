use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{
        Id,
        marker::{ApplicationMarker, GuildMarker},
    },
};

pub struct GetGuildCommandsFields {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    with_localizations: Option<bool>,
}

/// Fetch all commands for a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommands<'a> {
    fields: GetGuildCommandsFields,
    http: &'a Client,
}

impl<'a> GetGuildCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            fields: GetGuildCommandsFields {
                application_id,
                guild_id,
                with_localizations: None,
            },
            http,
        }
    }

    /// Whether to include full localization dictionaries in the response.
    ///
    /// Defaults to [`false`].
    pub const fn with_localizations(mut self, with_localizations: bool) -> Self {
        self.fields.with_localizations = Some(with_localizations);

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

impl Route for GetGuildCommands<'_> {
    type Fields = GetGuildCommandsFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("guilds")
            .id(fields.guild_id)
            .resource("commands")
            .parameter("with_localizations", fields.with_localizations)
            .build()
    }
}

impl TryIntoRequest for GetGuildCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
