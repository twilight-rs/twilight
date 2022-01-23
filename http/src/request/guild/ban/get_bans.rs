use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    guild::Ban,
    id::{marker::GuildMarker, Id},
};

/// Retrieve the bans for a guild.
///
/// # Examples
///
/// Retrieve the bans for guild `1`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(1);
///
/// let bans = client.bans(guild_id).exec().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetBans<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetBans<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Ban>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetBans<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetBans {
            guild_id: self.guild_id.get(),
        }))
    }
}
