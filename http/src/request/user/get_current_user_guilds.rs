use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    id::{marker::GuildMarker, Id},
    user::CurrentUserGuild,
};
use twilight_validate::request::{
    get_current_user_guilds_limit as validate_get_current_user_guilds_limit, ValidationError,
};

struct GetCurrentUserGuildsFields {
    after: Option<Id<GuildMarker>>,
    before: Option<Id<GuildMarker>>,
    limit: Option<u64>,
}

/// Returns a list of guilds for the current user.
///
/// # Examples
///
/// Get the first 25 guilds with an ID after `300` and before
/// `400`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let after = Id::new(300).expect("non zero");
/// let before = Id::new(400).expect("non zero");
/// let guilds = client.current_user_guilds()
///     .after(after)
///     .before(before)
///     .limit(25)?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserGuilds<'a> {
    fields: GetCurrentUserGuildsFields,
    http: &'a Client,
}

impl<'a> GetCurrentUserGuilds<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            fields: GetCurrentUserGuildsFields {
                after: None,
                before: None,
                limit: None,
            },
            http,
        }
    }

    /// Get guilds after this guild id.
    pub const fn after(mut self, guild_id: Id<GuildMarker>) -> Self {
        self.fields.after = Some(guild_id);

        self
    }

    /// Get guilds before this guild id.
    pub const fn before(mut self, guild_id: Id<GuildMarker>) -> Self {
        self.fields.before = Some(guild_id);

        self
    }

    /// Set the maximum number of guilds to retrieve.
    ///
    /// The minimum is 1 and the maximum is 200. Refer to [the discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetCurrentUserGuilds`] if the name length is
    /// too short or too long.
    ///
    /// [`GetCurrentUserGuilds`]: twilight_validate::misc::ValidationErrorType::GetCurrentUserGuilds
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/user#get-current-user-guilds-query-string-params
    pub const fn limit(mut self, limit: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_current_user_guilds_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<CurrentUserGuild>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUserGuilds<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Ok(Request::from_route(&Route::GetGuilds {
            after: self.fields.after.map(Id::get),
            before: self.fields.before.map(Id::get),
            limit: self.fields.limit,
        }))
    }
}
