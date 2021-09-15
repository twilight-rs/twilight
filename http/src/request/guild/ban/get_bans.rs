use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{guild::Ban, id::GuildId};

/// Retrieve the bans for a guild.
///
/// # Examples
///
/// Retrieve the bans for guild `1`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = GuildId::new(1).expect("non zero");
///
/// let bans = client.bans(guild_id).exec().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetBans<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetBans<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Ban>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetBans<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetBans {
            guild_id: self.guild_id.get(),
        }))
    }
}
