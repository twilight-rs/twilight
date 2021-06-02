use crate::{
    client::Client,
    error::Error,
    request::{PendingOption, Request},
    routing::Route,
};
use twilight_model::{channel::Channel, id::ChannelId};

/// Get a channel by its ID.
///
/// # Examples
///
/// Get channel `100`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(100);
///
/// let channel = client.channel(channel_id).await?;
/// # Ok(()) }
/// ```
pub struct GetChannel<'a> {
    channel_id: ChannelId,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
}

impl<'a> GetChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetChannel {
            channel_id: self.channel_id.0,
        });

        self.fut.replace(Box::pin(self.http.request_bytes(request)));

        Ok(())
    }
}

poll_req!(opt, GetChannel<'_>, Channel);
