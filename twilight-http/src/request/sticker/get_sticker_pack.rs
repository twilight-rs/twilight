use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::sticker::StickerPack,
    id::{Id, marker::StickerPackMarker},
};

/// Get a sticker pack by its ID.
///
/// # Examples
///
/// ```no_run
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
///
/// let id = Id::new(123);
/// let sticker_pack = client.sticker_pack(id).await?.model().await?;
///
/// println!("Sticker pack name: {}", sticker_pack.name);
/// println!("Sticker pack item names:");
///
/// for sticker in sticker_pack.stickers {
///     println!("- {}", sticker.name);
/// }
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetStickerPack<'a> {
    http: &'a Client,
    sticker_pack_id: Id<StickerPackMarker>,
}

impl<'a> GetStickerPack<'a> {
    pub(crate) const fn new(http: &'a Client, sticker_pack_id: Id<StickerPackMarker>) -> Self {
        Self {
            http,
            sticker_pack_id,
        }
    }
}

impl IntoFuture for GetStickerPack<'_> {
    type Output = Result<Response<StickerPack>, Error>;

    type IntoFuture = ResponseFuture<StickerPack>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetStickerPack<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetStickerPack {
            sticker_pack_id: self.sticker_pack_id.get(),
        }))
    }
}
