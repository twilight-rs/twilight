use crate::{
    client::Client,
    error::Error,
    request::{PendingOption, Request},
    routing::Route,
};
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, GuildId},
};

/// Get an emoji for a guild by the the guild's ID and emoji's ID.
///
/// # Examples
///
/// Get emoji `100` from guild `50`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::{EmojiId, GuildId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
///
/// let guild_id = GuildId(50);
/// let emoji_id = EmojiId(100);
///
/// client.emoji(guild_id, emoji_id).await?;
/// # Ok(()) }
/// ```
pub struct GetEmoji<'a> {
    emoji_id: EmojiId,
    fut: Option<PendingOption<'a>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            emoji_id,
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetEmoji {
            emoji_id: self.emoji_id.0,
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request_bytes(request)));

        Ok(())
    }
}

poll_req!(opt, GetEmoji<'_>, Emoji);
