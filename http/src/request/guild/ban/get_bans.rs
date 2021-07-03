use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{guild::Ban, id::GuildId};

/// Retrieve the bans for a guild.
///
/// # Examples
///
/// Retrieve the bans for guild `1`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
///
/// let guild_id = GuildId(1);
///
/// let bans = client.bans(guild_id).await?;
/// # Ok(()) }
/// ```
pub struct GetBans<'a> {
    fut: Option<PendingResponse<'a, ListBody<Ban>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetBans<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetBans {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetBans<'_>, ListBody<Ban>);
