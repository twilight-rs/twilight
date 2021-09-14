use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
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
/// ```rust,no_run
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
        let request = Request::from_route(&Route::GetNitroStickerPacks);

        self.http.request(request)
    }
}
