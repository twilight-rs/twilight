use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{Id, marker::GuildMarker},
};

struct GetEmojisPath {
    guild_id: Id<GuildMarker>,
}

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
    http: &'a Client,
    path: GetEmojisPath,
}

impl<'a> GetEmojis<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetEmojisPath { guild_id },
        }
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
        let GetEmojisPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("emojis")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
