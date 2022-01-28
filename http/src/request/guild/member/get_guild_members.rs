use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::{marker::MemberListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};
use twilight_validate::request::{
    get_guild_members_limit as validate_get_guild_members_limit, ValidationError,
};

struct GetGuildMembersFields {
    after: Option<Id<UserMarker>>,
    limit: Option<u16>,
    presences: Option<bool>,
}

/// Get the members of a guild, by id.
///
/// The upper limit to this request is 1000. If more than 1000 members are needed, the requests
/// must be chained. Discord defaults the limit to 1.
///
/// # Examples
///
/// Get the first 500 members of guild `100` after user ID `3000`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(100);
/// let user_id = Id::new(3000);
/// let members = client.guild_members(guild_id).after(user_id).exec().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetGuildMembers<'a> {
    fields: GetGuildMembersFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildMembers<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: GetGuildMembersFields {
                after: None,
                limit: None,
                presences: None,
            },
            guild_id,
            http,
        }
    }

    /// Sets the user ID to get members after.
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        self.fields.after = Some(after);

        self
    }

    /// Sets the number of members to retrieve per request.
    ///
    /// The limit must be greater than 0 and less than 1000.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetGuildMembers`] if the limit is 0 or
    /// greater than 1000.
    ///
    /// [`GetGuildMembers`]: twilight_validate::request::ValidationErrorType::GetGuildMembers
    pub const fn limit(mut self, limit: u16) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_guild_members_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Sets whether to retrieve matched member presences
    pub const fn presences(mut self, presences: bool) -> Self {
        self.fields.presences = Some(presences);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<MemberListBody> {
        let guild_id = self.guild_id;
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => {
                let mut future = http.request(request);
                future.set_guild_id(guild_id);

                future
            }
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildMembers<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Ok(Request::from_route(&Route::GetGuildMembers {
            after: self.fields.after.map(Id::get),
            guild_id: self.guild_id.get(),
            limit: self.fields.limit,
            presences: self.fields.presences,
        }))
    }
}
