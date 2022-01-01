use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    channel::message::sticker::Sticker,
    id::{marker::GuildMarker, Id},
};

/// Returns a list of stickers in a guild.
///
/// # Examples
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
/// let stickers = client
///     .guild_stickers(guild_id)
///     .exec()
///     .await?
///     .models()
///     .await?;
///
/// println!("{}", stickers.len());
/// # Ok(()) }
/// ```
pub struct GetGuildStickers<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildStickers<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Sticker>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildStickers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildStickers {
            guild_id: self.guild_id.get(),
        }))
    }
}
