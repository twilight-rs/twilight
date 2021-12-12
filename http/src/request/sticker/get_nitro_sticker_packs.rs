use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Deserialize;
use twilight_model::channel::message::sticker::StickerPack;

#[derive(Deserialize)]
pub struct StickerPackListing {
    pub sticker_packs: Vec<StickerPack>,
}

/// Returns a list of sticker packs available to Nitro subscribers.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let packs = client.nitro_sticker_packs().exec().await?.model().await?;
///
/// println!("{}", packs.sticker_packs.len());
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetNitroStickerPacks<'a> {
    http: &'a Client,
}

impl<'a> GetNitroStickerPacks<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<StickerPackListing> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetNitroStickerPacks<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetNitroStickerPacks))
    }
}
