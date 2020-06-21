use crate::request::prelude::*;
use twilight_model::{guild::Emoji, id::GuildId};

/// Get the emojis for a guild, by the guild's id.
///
/// # Examples
///
/// Get the emojis for guild `100`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let guild_id = GuildId(100);
///
/// client.emojis(guild_id).await?;
/// # Ok(()) }
/// ```
pub struct GetEmojis<'a> {
    fut: Option<Pending<'a, Vec<Emoji>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetEmojis<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetEmojis {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetEmojis<'_>, Vec<Emoji>);
