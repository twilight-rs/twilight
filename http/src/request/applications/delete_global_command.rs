use crate::request::prelude::*;
use twilight_model::id::*;

pub struct DeleteGlobalCommand<'a> {
    application_id: ApplicationId,
    command_id: CommandId,
    fut: Option<Pending<'a, ()>>,
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

    fn start(&mut self) -> Result<()> {
        let req = Request::from(Route::DeleteGlobalCommand {
            application_id: self.application_id.0,
            command_id: self.command_id.0,
        });
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(DeleteGlobalCommand<'_>, ());
