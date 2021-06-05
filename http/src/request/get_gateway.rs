use crate::{
    client::Client,
    error::Error,
    request::{GetGatewayAuthed, Pending, Request},
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
/// ```rust,no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let info = client.gateway().await?;
/// # Ok(()) }
/// ```
///
/// Get the gateway connection URL with additional shard and session information, which
/// requires specifying a bot token:
///
/// ```rust,no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let info = client.gateway().authed().await?;
///
/// println!("URL: {}", info.url);
/// println!("Recommended shards to use: {}", info.shards);
/// # Ok(()) }
/// ```
pub struct GetGateway<'a> {
    fut: Option<Pending<'a, ConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetGateway<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    /// Call to authenticate this request.
    ///
    /// Returns additional information: the recommended number of shards to use, and information on
    /// the current session start limit.
    pub fn authed(self) -> GetGatewayAuthed<'a> {
        GetGatewayAuthed::new(self.http)
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGateway);

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGateway<'_>, ConnectionInfo);
