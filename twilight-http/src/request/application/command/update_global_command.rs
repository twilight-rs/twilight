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
        marker::{ApplicationMarker, CommandMarker},
        Id,
    },
};

#[derive(Serialize)]
struct UpdateGlobalCommandFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<&'a [CommandOption]>,
}

/// Edit a global command, by ID.
///
/// You must specify a name and description. See
/// [Discord Docs/Edit Global Application Command].
///
/// [Discord Docs/Edit Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command
#[must_use = "requests must be configured and executed"]
pub struct UpdateGlobalCommand<'a> {
    fields: UpdateGlobalCommandFields<'a>,
    command_id: Id<CommandMarker>,
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
}

impl<'a> UpdateGlobalCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            application_id,
            command_id,
            fields: UpdateGlobalCommandFields {
                description: None,
                name: None,
                nsfw: None,
                options: None,
            },
            http,
        }
    }

    /// Edit the name of the command.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);

        self
    }

    /// Edit the description of the command.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.fields.description = Some(description);

        self
    }

    /// Edit the command options of the command.
    pub const fn command_options(mut self, options: &'a [CommandOption]) -> Self {
        self.fields.options = Some(options);

        self
    }

    /// Edit whether the command is age-restricted.
    pub const fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw = Some(nsfw);

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

impl TryIntoRequest for UpdateGlobalCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateGlobalCommand {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
        })
        .json(&self.fields)
        .build()
    }
}
