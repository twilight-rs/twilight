use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Ban,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};
use twilight_validate::request::{
    ValidationError, get_guild_bans_limit as validate_get_guild_bans_limit,
};

struct GetBansPath {
    after: Option<Id<UserMarker>>,
    before: Option<Id<UserMarker>>,
    guild_id: Id<GuildMarker>,
    limit: Option<u16>,
}

/// Retrieve the bans for a guild.
///
/// # Examples
///
/// Retrieve the first 25 bans of a guild after a particular user ID:
///
/// ```no_run
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
///
/// let guild_id = Id::new(1);
/// let user_id = Id::new(2);
///
/// let response = client.bans(guild_id).after(user_id).limit(25).await?;
/// let bans = response.models().await?;
///
/// for ban in bans {
///     println!("{} was banned for: {:?}", ban.user.name, ban.reason);
/// }
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetBans<'a> {
    http: &'a Client,
    path: Result<GetBansPath, ValidationError>,
}

impl<'a> GetBans<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: Ok(GetBansPath {
                after: None,
                before: None,
                guild_id,
                limit: None,
            }),
        }
    }

    /// Set the user ID after which to retrieve bans.
    ///
    /// Mutually exclusive with [`before`]. If both are provided then [`before`]
    /// is respected.
    ///
    /// [`before`]: Self::before
    pub const fn after(mut self, user_id: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(user_id);
        }

        self
    }

    /// Set the user ID before which to retrieve bans.
    ///
    /// Mutually exclusive with [`after`]. If both are provided then [`before`]
    /// is respected.
    ///
    /// [`after`]: Self::after
    /// [`before`]: Self::before
    pub const fn before(mut self, user_id: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.before = Some(user_id);
        }

        self
    }

    /// Set the maximum number of bans to retrieve.
    ///
    /// Defaults to Discord's default.
    ///
    /// Refer to [Discord Docs/Get Guild Bans] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetGuildBans`] if the limit is invalid.
    ///
    /// [`GetGuildBans`]: twilight_validate::request::ValidationErrorType::GetGuildBans
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_get_guild_bans_limit(limit)?;
            path.limit.replace(limit);

            Ok(path)
        });

        self
    }
}

impl IntoFuture for GetBans<'_> {
    type Output = Result<Response<ListBody<Ban>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Ban>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetBans<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = self.path.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetBansWithParameters {
            after: path.after.map(Id::get),
            before: path.before.map(Id::get),
            limit: path.limit,
            guild_id: path.guild_id.get(),
        }))
    }
}
