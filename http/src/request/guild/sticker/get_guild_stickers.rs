use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{channel::message::sticker::Sticker, id::GuildId};

/// Returns a list of stickers in a guild.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::{
///     channel::message::sticker::StickerId,
///     id::GuildId,
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = GuildId::new(1).expect("non zero");
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
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildStickers<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Sticker>> {
        let request = Request::from_route(&Route::GetGuildStickers {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
