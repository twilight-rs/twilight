use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::channel::message::sticker::{Sticker, StickerId};

/// Returns a single sticker by its ID.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::channel::message::sticker::StickerId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let id = StickerId(123);
/// let sticker = client.sticker(id).exec().await?.model().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetSticker<'a> {
    http: &'a Client,
    sticker_id: StickerId,
}

impl<'a> GetSticker<'a> {
    pub(crate) const fn new(http: &'a Client, sticker_id: StickerId) -> Self {
        Self { http, sticker_id }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Sticker> {
        let request = Request::from_route(&Route::GetSticker {
            sticker_id: self.sticker_id.0,
        });

        self.http.request(request)
    }
}
