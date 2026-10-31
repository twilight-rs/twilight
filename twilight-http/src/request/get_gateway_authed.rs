use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::gateway::connection_info::BotConnectionInfo;

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
}

impl IntoFuture for GetGatewayAuthed<'_> {
    type Output = Result<Response<BotConnectionInfo>, Error>;

    type IntoFuture = ResponseFuture<BotConnectionInfo>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGatewayAuthed<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("gateway")
            .action("bot")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
