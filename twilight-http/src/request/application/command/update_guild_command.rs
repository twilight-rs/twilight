use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
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

#[expect(clippy::struct_field_names)]
struct UpdateGuildCommandPath {
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
    http: &'a Client,
    path: UpdateGuildCommandPath,
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
            http,
            path: UpdateGuildCommandPath {
                application_id,
                command_id,
                guild_id,
            },
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

impl TryIntoRequest for UpdateGuildCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_from_route(&Route::UpdateGuildCommand {
            application_id: self.path.application_id.get(),
            command_id: self.path.command_id.get(),
            guild_id: self.path.guild_id.get(),
        })
        .json(&self.body)
        .build()
    }
}
