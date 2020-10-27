use crate::request::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{guild::Permissions, id::GuildId};

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

/// Information about a guild the current user is in.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentUserGuild {
    /// Unique ID.
    pub id: GuildId,
    /// Name of the guild.
    ///
    /// The name must be at least 2 characters long and at most 100 characters
    /// long.
    pub name: String,
    /// Hash of the icon.
    ///
    /// Refer to the [Discord documentation] for more information.
    ///
    /// [Discord documentation]: https://discord.com/developers/docs/reference#image-formatting
    pub icon: Option<String>,
    /// Whether the current user is the owner.
    pub owner: bool,
    /// Permissions of the current user in the guild. This excludes channels'
    /// permission overwrites.
    pub permissions: Permissions,
    /// List of enabled guild features.
    pub features: Vec<String>,
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
    /// [`GetCurrentUserGuildsError::LimitInvalid`]: enum.GetCurrentUserGuildsError.html#variant.LimitInvalid
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

#[cfg(test)]
mod tests {
    use super::{CurrentUserGuild, GuildId};
    use serde_test::Token;
    use twilight_model::guild::Permissions;

    #[test]
    fn test_current_user_guild() {
        // The example partial guild from the discord docs
        let value = CurrentUserGuild {
            id: GuildId(80_351_110_224_678_912),
            name: "abcd".to_owned(),
            icon: Some("8342729096ea3675442027381ff50dfe".to_owned()),
            owner: true,
            permissions: Permissions::from_bits_truncate(36_953_089),
            features: vec!["a feature".to_owned()],
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentUserGuild",
                    len: 6,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("80351110224678912"),
                Token::Str("name"),
                Token::Str("abcd"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("8342729096ea3675442027381ff50dfe"),
                Token::Str("owner"),
                Token::Bool(true),
                Token::Str("permissions"),
                Token::Str("36953089"),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("a feature"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
