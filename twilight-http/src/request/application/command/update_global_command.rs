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
        marker::{ApplicationMarker, CommandMarker},
    },
};

#[derive(Serialize)]
struct UpdateGlobalCommandBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<&'a [CommandOption]>,
}

pub struct UpdateGlobalCommandFields {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
}

/// Edit a global command, by ID.
///
/// You must specify a name and description. See
/// [Discord Docs/Edit Global Application Command].
///
/// [Discord Docs/Edit Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct UpdateGlobalCommand<'a> {
    body: UpdateGlobalCommandBody<'a>,
    fields: UpdateGlobalCommandFields,
    http: &'a Client,
}

impl<'a> UpdateGlobalCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            body: UpdateGlobalCommandBody {
                description: None,
                name: None,
                nsfw: None,
                options: None,
            },
            fields: UpdateGlobalCommandFields {
                application_id,
                command_id,
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

impl IntoFuture for UpdateGlobalCommand<'_> {
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

impl Route for UpdateGlobalCommand<'_> {
    type Fields = UpdateGlobalCommandFields;

    const METHOD: Method = Method::Patch;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("commands")
            .id(fields.command_id)
            .build()
    }
}

impl TryIntoRequest for UpdateGlobalCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.fields)
            .json(&self.body)
            .build()
    }
}
