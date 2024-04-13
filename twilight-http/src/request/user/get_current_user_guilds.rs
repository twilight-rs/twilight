use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
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
    limit: Option<u16>,
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
/// let after = Id::new(300);
/// let before = Id::new(400);
/// let guilds = client
///     .current_user_guilds()
///     .after(after)
///     .before(before)
///     .limit(25)
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserGuilds<'a> {
    fields: Result<GetCurrentUserGuildsFields, ValidationError>,
    http: &'a Client,
}

impl<'a> GetCurrentUserGuilds<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            fields: Ok(GetCurrentUserGuildsFields {
                after: None,
                before: None,
                limit: None,
            }),
            http,
        }
    }

    /// Get guilds after this guild id.
    pub fn after(mut self, guild_id: Id<GuildMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.after = Some(guild_id);
        }

        self
    }

    /// Get guilds before this guild id.
    pub fn before(mut self, guild_id: Id<GuildMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.before = Some(guild_id);
        }

        self
    }

    /// Set the maximum number of guilds to retrieve.
    ///
    /// The minimum is 1 and the maximum is 200. See
    /// [Discord Docs/Get Current User Guilds].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetCurrentUserGuilds`] if the name length is
    /// too short or too long.
    ///
    /// [`GetCurrentUserGuilds`]: twilight_validate::request::ValidationErrorType::GetCurrentUserGuilds
    /// [Discord Docs/Get Current User Guilds]: https://discordapp.com/developers/docs/resources/user#get-current-user-guilds-query-string-params
    pub fn limit(mut self, limit: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_get_current_user_guilds_limit(limit)?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for GetCurrentUserGuilds<'_> {
    type Output = Result<Response<ListBody<CurrentUserGuild>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<CurrentUserGuild>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUserGuilds<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetGuilds {
            after: fields.after.map(Id::get),
            before: fields.before.map(Id::get),
            limit: fields.limit,
        }))
    }
}
