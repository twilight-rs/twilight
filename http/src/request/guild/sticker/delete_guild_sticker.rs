use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{GuildMarker, StickerMarker},
    Id,
};

/// Deletes a guild sticker by the ID of the guild and its ID.
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
///
/// client
///     .delete_guild_sticker(guild_id, sticker_id)
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
pub struct DeleteGuildSticker<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    sticker_id: Id<StickerMarker>,
}

impl<'a> DeleteGuildSticker<'a> {
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
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let request = Request::from_route(&Route::DeleteGuildSticker {
            guild_id: self.guild_id.get(),
            sticker_id: self.sticker_id.get(),
        });

        self.http.request(request)
    }
}
