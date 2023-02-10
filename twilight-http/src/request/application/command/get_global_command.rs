use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ApplicationMarker, CommandMarker},
        Id,
    },
};

/// Retrieve a global command for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGlobalCommand<'a> {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    http: &'a Client,
}

impl<'a> GetGlobalCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            application_id,
            command_id,
            http,
        }
    }
}

impl IntoFuture for GetGlobalCommand<'_> {
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

impl TryIntoRequest for GetGlobalCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGlobalCommand {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
        }))
    }
}
