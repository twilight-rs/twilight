use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker},
    },
};

struct GetEmojiPath {
    emoji_id: Id<EmojiMarker>,
    guild_id: Id<GuildMarker>,
}

/// Get an emoji for a guild by the the guild's ID and emoji's ID.
///
/// # Examples
///
/// Get emoji `100` from guild `50`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(50);
/// let emoji_id = Id::new(100);
///
/// client.emoji(guild_id, emoji_id).await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetEmoji<'a> {
    http: &'a Client,
    path: GetEmojiPath,
}

impl<'a> GetEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            http,
            path: GetEmojiPath { emoji_id, guild_id },
        }
    }
}

impl IntoFuture for GetEmoji<'_> {
    type Output = Result<Response<Emoji>, Error>;

    type IntoFuture = ResponseFuture<Emoji>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetEmojiPath { emoji_id, guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("emojis")
            .id(emoji_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
