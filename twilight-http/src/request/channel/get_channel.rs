use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Channel,
    id::{Id, marker::ChannelMarker},
};

struct GetChannelPath {
    channel_id: Id<ChannelMarker>,
}

/// Get a channel by its ID.
///
/// # Examples
///
/// Get channel `100`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(100);
///
/// let channel = client.channel(channel_id).await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetChannel<'a> {
    http: &'a Client,
    path: GetChannelPath,
}

impl<'a> GetChannel<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: GetChannelPath { channel_id },
        }
    }
}

impl IntoFuture for GetChannel<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetChannelPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
