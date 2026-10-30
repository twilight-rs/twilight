use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::sticker::Sticker,
    id::{
        Id,
        marker::{GuildMarker, StickerMarker},
    },
};

struct GetGuildStickerPath {
    guild_id: Id<GuildMarker>,
    sticker_id: Id<StickerMarker>,
}

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
    http: &'a Client,
    path: GetGuildStickerPath,
}

impl<'a> GetGuildSticker<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sticker_id: Id<StickerMarker>,
    ) -> Self {
        Self {
            http,
            path: GetGuildStickerPath {
                guild_id,
                sticker_id,
            },
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
        let GetGuildStickerPath {
            guild_id,
            sticker_id,
        } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("stickers")
            .id(sticker_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
