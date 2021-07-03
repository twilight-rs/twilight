use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{ApplicationId, CommandId};

/// Delete a global command, by ID.
pub struct DeleteGlobalCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    http: &'a Client,
}

impl<'a> DeleteGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        command_id: CommandId,
    ) -> Self {
        Self {
            application_id,
            command_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::DeleteGlobalCommand {
            application_id: self.application_id.0,
            command_id: self.command_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(DeleteGlobalCommand<'_>, EmptyBody);
