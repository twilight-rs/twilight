use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Sticker> {
        let request = Request::from_route(&Route::GetGuildSticker {
            guild_id: self.guild_id.get(),
            sticker_id: self.sticker_id.get(),
        });

        self.http.request(request)
    }
}
