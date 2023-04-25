use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{marker::GuildMarker, Id},
};

/// Get the emojis for a guild, by the guild's id.
///
/// # Examples
///
/// Get the emojis for guild `100`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(100);
///
/// client.emojis(guild_id).await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetEmojis<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetEmojis<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetEmojis<'_> {
    type Output = Result<Response<ListBody<Emoji>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Emoji>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetEmojis<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetEmojis {
            guild_id: self.guild_id.get(),
        }))
    }
}
