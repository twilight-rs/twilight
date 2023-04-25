use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::sticker::Sticker,
    id::{
        marker::{GuildMarker, StickerMarker},
        Id,
    },
};

/// Returns a guild sticker by the guild's ID and the sticker's ID.
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
/// let sticker_id = Id::new(2);
/// let sticker = client
///     .guild_sticker(guild_id, sticker_id)
///     .await?
///     .model()
///     .await?;
///
/// println!("{sticker:#?}");
/// # Ok(()) }
/// ```
pub struct GetGuildSticker<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    sticker_id: Id<StickerMarker>,
}

impl<'a> GetGuildSticker<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sticker_id: Id<StickerMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            sticker_id,
        }
    }
}

impl IntoFuture for GetGuildSticker<'_> {
    type Output = Result<Response<Sticker>, Error>;

    type IntoFuture = ResponseFuture<Sticker>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildSticker<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildSticker {
            guild_id: self.guild_id.get(),
            sticker_id: self.sticker_id.get(),
        }))
    }
}
