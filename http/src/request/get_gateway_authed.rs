use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::gateway::connection_info::BotConnectionInfo;

/// Get information about the gateway, authenticated as a bot user.
///
/// Returns additional information: the recommended number of shards to use, and information on
/// the current session start limit.
pub struct GetGatewayAuthed<'a> {
    fut: Option<PendingResponse<'a, BotConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetGatewayAuthed<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGatewayBot);

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGatewayAuthed<'_>, BotConnectionInfo);
