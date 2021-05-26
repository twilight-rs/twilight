use crate::request::prelude::*;
use twilight_model::{application::command::Command, id::ApplicationId};

/// Fetch all global commands for an application.
pub struct GetGlobalCommands<'a> {
    application_id: ApplicationId,
    fut: Option<Pending<'a, Vec<Command>>>,
    http: &'a Client,
}

impl<'a> GetGlobalCommands<'a> {
    pub(crate) fn new(http: &'a Client, application_id: ApplicationId) -> Self {
        Self {
            application_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetGlobalCommands {
            application_id: self.application_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGlobalCommands<'_>, Vec<Command>);
