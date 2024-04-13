use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{marker::ApplicationMarker, Id},
};

/// Retrieve all global commands for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGlobalCommands<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
    with_localizations: Option<bool>,
}

impl<'a> GetGlobalCommands<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            http,
            with_localizations: None,
        }
    }

    /// Whether to include full localization dictionaries in the response.
    ///
    /// Defaults to [`false`].
    pub const fn with_localizations(mut self, with_localizations: bool) -> Self {
        self.with_localizations = Some(with_localizations);

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

impl TryIntoRequest for GetGlobalCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGlobalCommands {
            application_id: self.application_id.get(),
            with_localizations: self.with_localizations,
        }))
    }
}
