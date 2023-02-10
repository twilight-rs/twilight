use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{
        marker::{EmojiMarker, GuildMarker},
        Id,
    },
};

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
    emoji_id: Id<EmojiMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            emoji_id,
            guild_id,
            http,
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
        Ok(Request::from_route(&Route::GetEmoji {
            emoji_id: self.emoji_id.get(),
            guild_id: self.guild_id.get(),
        }))
    }
}
