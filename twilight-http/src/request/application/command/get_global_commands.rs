use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{Id, marker::ApplicationMarker},
};

pub struct GetGlobalCommandsFields {
    application_id: Id<ApplicationMarker>,
    with_localizations: Option<bool>,
}

/// Retrieve all global commands for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGlobalCommands<'a> {
    fields: GetGlobalCommandsFields,
    http: &'a Client,
}

impl<'a> GetGlobalCommands<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            fields: GetGlobalCommandsFields {
                application_id,
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

impl IntoFuture for GetGlobalCommands<'_> {
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

impl Route for GetGlobalCommands<'_> {
    type Fields = GetGlobalCommandsFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("commands")
            .optional_parameter("with_localizations", fields.with_localizations)
            .build()
    }
}

impl TryIntoRequest for GetGlobalCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
