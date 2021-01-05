use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{id::GuildId, user::CurrentUserGuild};

/// The error created when the current guilds can not be retrieved as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GetCurrentUserGuildsError {
    /// The maximum number of guilds to retrieve is 0 or more than 100.
    LimitInvalid {
        /// Provided maximum number of guilds to retrieve.
        limit: u64,
    },
}

impl Display for GetCurrentUserGuildsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid { .. } => f.write_str("the limit is invalid"),
        }
    }
}

impl Error for GetCurrentUserGuildsError {}

struct GetCurrentUserGuildsFields {
    after: Option<GuildId>,
    before: Option<GuildId>,
    limit: Option<u64>,
}

/// Returns a list of guilds for the current user.
///
/// # Examples
///
/// Get the first 25 guilds with an ID after `300` and before
/// `400`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let after = GuildId(300);
/// let before = GuildId(400);
/// let guilds = client.current_user_guilds()
///     .after(after)
///     .before(before)
///     .limit(25)?
///     .await?;
/// # Ok(()) }
/// ```
pub struct GetCurrentUserGuilds<'a> {
    fields: GetCurrentUserGuildsFields,
    fut: Option<Pending<'a, Vec<CurrentUserGuild>>>,
    http: &'a Client,
}

impl<'a> GetCurrentUserGuilds<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fields: GetCurrentUserGuildsFields {
                after: None,
                before: None,
                limit: None,
            },
            fut: None,
            http,
        }
    }

    /// Get guilds after this guild id.
    pub fn after(mut self, guild_id: GuildId) -> Self {
        self.fields.after.replace(guild_id);

        self
    }

    /// Get guilds before this guild id.
    pub fn before(mut self, guild_id: GuildId) -> Self {
        self.fields.before.replace(guild_id);

        self
    }

    /// Set the maximum number of guilds to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100. Refer to [the discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns [`GetCurrentUserGuildsError::LimitInvalid`] if the amount is greater
    /// than 100.
    ///
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/user#get-current-user-guilds-query-string-params
    pub fn limit(mut self, limit: u64) -> Result<Self, GetCurrentUserGuildsError> {
        if !validate::get_current_user_guilds_limit(limit) {
            return Err(GetCurrentUserGuildsError::LimitInvalid { limit });
        }

        self.fields.limit.replace(limit);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuilds {
                after: self.fields.after.map(|x| x.0),
                before: self.fields.before.map(|x| x.0),
                limit: self.fields.limit,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetCurrentUserGuilds<'_>, Vec<CurrentUserGuild>);
