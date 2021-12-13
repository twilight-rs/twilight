use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::message::sticker::Sticker,
    id::{marker, Id},
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
/// let guild_id = Id::new(1).expect("non zero");
/// let sticker_id = Id::new(2).expect("non zero");
/// let sticker = client
///     .guild_sticker(guild_id, sticker_id)
///     .exec()
///     .await?
///     .model()
///     .await?;
///
/// println!("{:#?}", sticker);
/// # Ok(()) }
/// ```
pub struct GetGuildSticker<'a> {
    guild_id: Id<marker::Guild>,
    http: &'a Client,
    sticker_id: Id<marker::Sticker>,
}

impl<'a> GetGuildSticker<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<marker::Guild>,
        sticker_id: Id<marker::Sticker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            sticker_id,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Sticker> {
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
