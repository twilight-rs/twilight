use crate::{
    client::Client,
    error::Error,
    request::{GetGatewayAuthed, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::gateway::connection_info::ConnectionInfo;

/// Get information about the gateway, optionally with additional information detailing the
/// number of shards to use and sessions remaining.
///
/// # Examples
///
/// Get the gateway connection URL without bot information:
///
/// ```no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let info = client.gateway().exec().await?.model().await?;
/// # Ok(()) }
/// ```
///
/// Get the gateway connection URL with additional shard and session information, which
/// requires specifying a bot token:
///
/// ```no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let info = client.gateway().authed().exec().await?.model().await?;
///
/// println!("URL: {}", info.url);
/// println!("Recommended shards to use: {}", info.shards);
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetGateway<'a> {
    http: &'a Client,
}

impl<'a> GetGateway<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Call to authenticate this request.
    ///
    /// Returns additional information: the recommended number of shards to use, and information on
    /// the current session start limit.
    pub const fn authed(self) -> GetGatewayAuthed<'a> {
        GetGatewayAuthed::new(self.http)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ConnectionInfo> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGateway<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGateway))
    }
}
