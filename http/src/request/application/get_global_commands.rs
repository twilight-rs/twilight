use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{application::command::Command, id::ApplicationId};

/// Retrieve all global commands for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGlobalCommands<'a> {
    application_id: ApplicationId,
    http: &'a Client,
}

impl<'a> GetGlobalCommands<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: ApplicationId) -> Self {
        Self {
            application_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Command>> {
        let request = Request::from_route(&Route::GetGlobalCommands {
            application_id: self.application_id.get(),
        });

        self.http.request(request)
    }
}
