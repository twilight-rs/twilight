use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::{application::command::Command, id::ApplicationId};

/// Set global commands.
///
/// This method is idempotent: it can be used on every start, without being
/// ratelimited if there aren't changes to the commands.
pub struct SetGlobalCommands<'a> {
    commands: Vec<Command>,
    application_id: ApplicationId,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    http: &'a Client,
}

impl<'a> SetGlobalCommands<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        commands: Vec<Command>,
    ) -> Self {
        Self {
            commands,
            application_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::SetGlobalCommands {
            application_id: self.application_id.0,
        })
        .json(&self.commands)?;

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

poll_req!(SetGlobalCommands<'_>, EmptyBody);
