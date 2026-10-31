use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    id::{Id, marker::GuildMarker},
    user::CurrentUserGuild,
};
use twilight_validate::request::{
    ValidationError, get_current_user_guilds_limit as validate_get_current_user_guilds_limit,
};

pub struct GetCurrentUserGuildsPath {
    after: Option<Id<GuildMarker>>,
    before: Option<Id<GuildMarker>>,
    limit: Option<u16>,
    shard: Option<u64>,
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
    http: &'a Client,
    path: Result<GetCurrentUserGuildsPath, ValidationError>,
}

impl<'a> GetCurrentUserGuilds<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            http,
            path: Ok(GetCurrentUserGuildsPath {
                after: None,
                before: None,
                limit: None,
                shard: None,
            }),
        }
    }

    /// Get guilds after this guild id.
    pub const fn after(mut self, guild_id: Id<GuildMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(guild_id);
        }

        self
    }

    /// Get guilds before this guild id.
    pub const fn before(mut self, guild_id: Id<GuildMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.before = Some(guild_id);
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
        self.path = self.path.and_then(|mut path| {
            validate_get_current_user_guilds_limit(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }

    /// Set the shard to return guilds for. Mandatory for applications using
    /// Large Bot Sharding.
    ///
    /// This is a value of 0 to `max_concurrency` - 1. By default this parameter
    /// is absent.
    pub const fn shard(mut self, shard: u64) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.shard = Some(shard);
        }

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
        let GetCurrentUserGuildsPath {
            after,
            before,
            limit,
            shard,
        } = self.path.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("users")
            .me()
            .resource("guilds")
            .parameter(("after", after))
            .parameter(("before", before))
            .parameter(("limit", limit))
            .parameter(("shard", shard))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
