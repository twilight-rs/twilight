use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use twilight_validate::request::{
    get_guild_members_limit as validate_get_guild_members_limit, ValidationError,
};

struct GetGuildMembersFields {
    after: Option<Id<UserMarker>>,
    limit: Option<u16>,
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
/// let members = client
///     .guild_members(guild_id)
///     .after(user_id)
///     .limit(500)
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetGuildMembers<'a> {
    fields: Result<GetGuildMembersFields, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildMembers<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: Ok(GetGuildMembersFields {
                after: None,
                limit: None,
            }),
            guild_id,
            http,
        }
    }

    /// Sets the user ID to get members after.
    pub fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.after = Some(after);
        }

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
    pub fn limit(mut self, limit: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_get_guild_members_limit(limit)?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for GetGuildMembers<'_> {
    type Output = Result<Response<ListBody<Member>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Member>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildMembers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetGuildMembers {
            after: fields.after.map(Id::get),
            guild_id: self.guild_id.get(),
            limit: fields.limit,
        }))
    }
}
