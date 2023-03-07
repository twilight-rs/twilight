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
    id::{marker::GuildMarker, Id},
};
use twilight_validate::request::{
    search_guild_members_limit as validate_search_guild_members_limit, ValidationError,
};

struct SearchGuildMembersFields<'a> {
    query: &'a str,
    limit: Option<u16>,
}

/// Search the members of a specific guild by a query.
///
/// The upper limit to this request is 1000. Discord defaults the limit to 1.
///
/// # Examples
///
/// Get the first 10 members of guild `100` matching `Wumpus`:
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
/// let members = client
///     .search_guild_members(guild_id, "Wumpus")
///     .limit(10)
///     .await?;
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns an error of type [`SearchGuildMembers`] if the limit is 0 or greater
/// than 1000.
///
/// [`SearchGuildMembers`]: twilight_validate::request::ValidationErrorType::SearchGuildMembers
#[must_use = "requests must be configured and executed"]
pub struct SearchGuildMembers<'a> {
    fields: Result<SearchGuildMembersFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> SearchGuildMembers<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>, query: &'a str) -> Self {
        Self {
            fields: Ok(SearchGuildMembersFields { query, limit: None }),
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
    /// [`SearchGuildMembers`]: twilight_validate::request::ValidationErrorType::SearchGuildMembers
    pub fn limit(mut self, limit: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_search_guild_members_limit(limit)?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for SearchGuildMembers<'_> {
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

impl TryIntoRequest for SearchGuildMembers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::SearchGuildMembers {
            guild_id: self.guild_id.get(),
            limit: fields.limit,
            query: fields.query,
        }))
    }
}
