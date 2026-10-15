use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, CommandMarker},
};

pub(in crate::request) struct DeleteGlobalCommandPath {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
}

/// Delete a global command, by ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGlobalCommand<'a> {
    http: &'a Client,
    path: DeleteGlobalCommandPath,
}

impl<'a> DeleteGlobalCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        command_id: Id<CommandMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteGlobalCommandPath {
                application_id,
                command_id,
            },
        }
    }
}

impl IntoFuture for DeleteGlobalCommand<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for DeleteGlobalCommand<'_> {
    type Fields = DeleteGlobalCommandPath;

    const METHOD: Method = Method::Delete;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("commands")
            .id(fields.command_id)
            .build()
    }
}

impl TryIntoRequest for DeleteGlobalCommand<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.path))
    }
}
