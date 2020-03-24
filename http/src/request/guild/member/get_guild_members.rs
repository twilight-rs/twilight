use crate::request::prelude::*;
use twilight_model::{
    guild::Member,
    id::{GuildId, UserId},
};

#[derive(Default)]
struct GetGuildMembersFields {
    after: Option<UserId>,
    limit: Option<u64>,
    presences: Option<bool>,
}

/// Gets a list of members from a guild.
///
/// # Examples
///
/// Get the first 500 members of guild `620316809459138607` after user ID
/// `587175671973937162`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::{GuildId, UserId};
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
    fields: GetGuildMembersFields,
    fut: Option<Pending<'a, Vec<Member>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildMembers<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: GetGuildMembersFields::default(),
            fut: None,
            guild_id,
            http,
        }
    }

    /// Sets the user ID to get members after.
    pub fn after(mut self, after: UserId) -> Self {
        self.fields.after.replace(after);

        self
    }

    /// Sets the number of members to retrieve per request.
    ///
    /// The maximum value accepted by the API is 1000.
    pub fn limit(mut self, limit: u64) -> Self {
        self.fields.limit.replace(limit);

        self
    }

    /// Sets whether to retrieve matched member presences
    pub fn presences(mut self, presences: bool) -> Self {
        self.fields.presences.replace(presences);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildMembers {
                after: self.fields.after.map(|x| x.0),
                guild_id: self.guild_id.0,
                limit: self.fields.limit,
                presences: self.fields.presences,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildMembers<'_>, Vec<Member>);
