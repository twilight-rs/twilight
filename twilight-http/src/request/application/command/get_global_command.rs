use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{
        Id,
        marker::{ApplicationMarker, CommandMarker},
    },
};

pub struct GetGlobalCommandPath {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
}

/// Retrieve a global command for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGlobalCommand<'a> {
    http: &'a Client,
    path: GetGlobalCommandPath,
}

impl<'a> GetGlobalCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            http,
            path: GetGlobalCommandPath {
                application_id,
                command_id,
            },
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
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("commands")
            .id(self.path.command_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
