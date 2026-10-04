use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    application::command::{Command, CommandOption},
    id::{
        Id,
        marker::{ApplicationMarker, CommandMarker, GuildMarker},
    },
};

#[derive(Serialize)]
struct UpdateGuildCommandBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<&'a [CommandOption]>,
}

pub(crate) struct UpdateGuildCommandFields {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    guild_id: Id<GuildMarker>,
}

/// Edit a command in a guild, by ID.
///
/// You must specify a name and description. See
/// [Discord Docs/Edit Guild Application Command].
///
/// [Discord Docs/Edit Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#edit-guild-application-command
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildCommand<'a> {
    body: UpdateGuildCommandBody<'a>,
    fields: UpdateGuildCommandFields,
    http: &'a Client,
}

impl<'a> UpdateGuildCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            body: UpdateGuildCommandBody {
                description: None,
                name: None,
                nsfw: None,
                options: None,
            },
            fields: UpdateGuildCommandFields {
                application_id,
                command_id,
                guild_id,
            },
            http,
        }
    }

    /// Edit the name of the command.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.body.name = Some(name);

        self
    }

    /// Edit the description of the command.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.body.description = Some(description);

        self
    }

    /// Edit the command options of the command.
    pub const fn command_options(mut self, options: &'a [CommandOption]) -> Self {
        self.body.options = Some(options);

        self
    }

    /// Edit whether the command is age-restricted.
    pub const fn nsfw(mut self, nsfw: bool) -> Self {
        self.body.nsfw = Some(nsfw);

        self
    }
}

impl IntoFuture for UpdateGuildCommand<'_> {
    type Output = Result<Response<Command>, Error>;

    type IntoFuture = ResponseFuture<Command>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for UpdateGuildCommand<'_> {
    type Fields = UpdateGuildCommandFields;

    const METHOD: Method = Method::Patch;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("guilds")
            .id(fields.guild_id)
            .resource("commands")
            .id(fields.command_id)
            .build()
    }
}

impl TryIntoRequest for UpdateGuildCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.fields)
            .json(&self.body)
            .build()
    }
}
