use super::prelude::*;
use dawn_model::{
    guild::Member,
    id::{GuildId, UserId},
};

/// Gets a list of members from a guild.
///
/// # Examples
///
/// Get the first 500 members of guild `620316809459138607` after user ID
/// `587175671973937162`:
///
/// ```rust,no_run
/// use dawn_http::Client;
/// use dawn_model::id::{GuildId, UserId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("bot token");
/// let guild_id = GuildId(620316809459138607);
/// let user_id = UserId(587175671973937162);
/// let members = client.guild_members(guild_id).after(user_id).await?;
///
/// for member in members {
///     println!("name: {}#{}", member.user.name, member.user.discriminator);
/// }
/// # Ok(()) }
/// ```
pub struct GetGuildMembers<'a> {
    after: Option<UserId>,
    limit: Option<u64>,
    fut: Option<Pending<'a, Vec<Member>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildMembers<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            after: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
            limit: None,
        }
    }

    /// Sets the user ID to get members after.
    pub fn after(mut self, after: UserId) -> Self {
        self.after.replace(after);

        self
    }

    /// Sets the number of members to retrieve per request.
    ///
    /// The maximum value accepted by the API is 1000.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildMembers {
                after: self.after.map(|x| x.0),
                guild_id: self.guild_id.0,
                limit: self.limit,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildMembers<'_>, Vec<Member>);
