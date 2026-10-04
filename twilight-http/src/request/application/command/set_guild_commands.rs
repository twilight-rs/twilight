use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{
        Id,
        marker::{ApplicationMarker, GuildMarker},
    },
};

#[derive(Serialize)]
#[serde(transparent)]
struct SetGuildCommandsBody<'a>(&'a [Command]);

pub(crate) struct SetGuildCommandsFields {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
}

/// Set a guild's commands.
///
/// This method is idempotent: it can be used on every start, without being
/// ratelimited if there aren't changes to the commands.
///
/// The [`Command`] struct has an [associated builder] in the
/// [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/struct.CommandBuilder.html
#[must_use = "requests must be configured and executed"]
pub struct SetGuildCommands<'a> {
    body: SetGuildCommandsBody<'a>,
    fields: SetGuildCommandsFields,
    http: &'a Client,
}

impl<'a> SetGuildCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        commands: &'a [Command],
    ) -> Self {
        Self {
            body: SetGuildCommandsBody(commands),
            fields: SetGuildCommandsFields {
                application_id,
                guild_id,
            },
            http,
        }
    }
}

impl IntoFuture for SetGuildCommands<'_> {
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

impl Route for SetGuildCommands<'_> {
    type Fields = SetGuildCommandsFields;

    const METHOD: Method = Method::Put;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("guilds")
            .id(fields.guild_id)
            .resource("commands")
            .build()
    }
}

impl TryIntoRequest for SetGuildCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.fields)
            .json(&self.body)
            .build()
    }
}
