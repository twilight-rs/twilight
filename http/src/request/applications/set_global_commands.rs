use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{applications::command::Command, id::ApplicationId};

/// Set commands globally
///
/// This will set the commands availible globally to a list of commands,
/// this method is idempotent which means it can be used every time the bot
/// starts without any issues with ratelimits if there are no changes.
pub struct SetGlobalCommands<'a> {
    commands: Vec<Command>,
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> SetGlobalCommands<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        commands: Vec<Command>,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            commands,
            application_id,
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.commands)?,
            Route::SetGlobalCommands {
                application_id: self.application_id.0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(SetGlobalCommands<'_>, ());
