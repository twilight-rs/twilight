use crate::{
    client::Client,
    request::Request,
    response::{marker::MemberListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::GuildId;
use twilight_validate::misc::{
    search_guild_members_limit as validate_search_guild_members_limit, ValidationError,
};

struct SearchGuildMembersFields<'a> {
    query: &'a str,
    limit: Option<u64>,
}

/// Search the members of a specific guild by a query.
///
/// The upper limit to this request is 1000. Discord defaults the limit to 1.
///
/// # Examples
///
/// Get the first 10 members of guild `100` matching `Wumpus`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = GuildId::new(100).expect("non zero");
/// let members = client.search_guild_members(guild_id, "Wumpus")
///     .limit(10)?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns an error of type [`SearchGuildMembers`] if the limit is 0 or greater
/// than 1000.
///
/// [`SearchGuildMembers`]: twilight_validate::misc::ValidationErrorType::SearchGuildMembers
#[must_use = "requests must be configured and executed"]
pub struct SearchGuildMembers<'a> {
    fields: SearchGuildMembersFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> SearchGuildMembers<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId, query: &'a str) -> Self {
        Self {
            fields: SearchGuildMembersFields { query, limit: None },
            guild_id,
            http,
        }
    }

    /// Sets the number of members to retrieve per request.
    ///
    /// The limit must be greater than 0 and less than 1000.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`SearchGuildMembers`] if the limit is 0 or
    /// greater than 1000.
    ///
    /// [`SearchGuildMembers`]: twilight_validate::misc::ValidationErrorType::SearchGuildMembers
    pub const fn limit(mut self, limit: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_search_guild_members_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<MemberListBody> {
        let request = Request::from_route(&Route::SearchGuildMembers {
            guild_id: self.guild_id.get(),
            limit: self.fields.limit,
            query: self.fields.query,
        });

        let mut future = self.http.request(request);
        future.set_guild_id(self.guild_id);

        future
    }
}
