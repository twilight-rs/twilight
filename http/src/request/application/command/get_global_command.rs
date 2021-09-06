use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    application::command::Command,
    id::{ApplicationId, CommandId},
};

/// Retrieve a global command for an application.
#[must_use = "requests must be configured and executed"]
pub struct GetGlobalCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    http: &'a Client,
}

impl<'a> GetGlobalCommand<'a> {
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
    pub fn exec(self) -> ResponseFuture<Command> {
        let request = Request::from_route(&Route::GetGlobalCommand {
            application_id: self.application_id.0,
            command_id: self.command_id.0,
        });

        self.http.request(request)
    }
}
