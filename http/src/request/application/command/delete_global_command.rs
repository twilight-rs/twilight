use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ApplicationId, CommandId};

/// Delete a global command, by ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteGlobalCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    http: &'a Client,
}

impl<'a> DeleteGlobalCommand<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        command_id: CommandId,
    ) -> Self {
        Self {
            application_id,
            command_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for DeleteGlobalCommand<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::DeleteGlobalCommand {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
        }))
    }
}
