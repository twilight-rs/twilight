use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::gateway::connection_info::BotConnectionInfo;

use super::IntoRequest;

/// Get information about the gateway, authenticated as a bot user.
///
/// Returns additional information: the recommended number of shards to use, and information on
/// the current session start limit.
#[must_use = "requests must be configured and executed"]
pub struct GetGatewayAuthed<'a> {
    http: &'a Client,
}

impl<'a> GetGatewayAuthed<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<BotConnectionInfo> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGatewayAuthed<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetGatewayBot))
    }
}
