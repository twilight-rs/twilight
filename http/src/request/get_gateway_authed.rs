use crate::request::prelude::*;
use twilight_model::gateway::connection_info::BotConnectionInfo;

/// Get information about the gateway through the lens of a bot.
///
/// Returns additional information: the recommended number of shards to use, and information on
/// the current session start limit.
pub struct GetGatewayAuthed<'a> {
    fut: Option<Pending<'a, BotConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetGatewayAuthed<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http.request(Request::from(Route::GetGatewayBot)),
        ));

        Ok(())
    }
}

poll_req!(GetGatewayAuthed<'_>, BotConnectionInfo);
