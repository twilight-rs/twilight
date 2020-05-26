use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{guild::PartialGuild, id::GuildId};

#[derive(Clone, Debug)]
pub enum GetCurrentUserGuildsError {
    /// The maximum number of guilds to retrieve is 0 or more than 100.
    LimitInvalid,
}

impl Display for GetCurrentUserGuildsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid => f.write_str("the limit is invalid"),
        }
    }
}

impl Error for GetCurrentUserGuildsError {}

struct GetCurrentUserGuildsFields {
    after: Option<GuildId>,
    before: Option<GuildId>,
    limit: Option<u64>,
}

pub struct GetCurrentUserGuilds<'a> {
    fields: GetCurrentUserGuildsFields,
    fut: Option<Pending<'a, Vec<PartialGuild>>>,
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

    pub fn after(mut self, guild_id: GuildId) -> Self {
        self.fields.after.replace(guild_id);

        self
    }

    pub fn before(mut self, guild_id: GuildId) -> Self {
        self.fields.before.replace(guild_id);

        self
    }

    /// Set the maximum number of guilds to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns [`GetCurrentUserGuildsError::LimitInvalid`] if the amount is greater
    /// than 100.
    ///
    /// [`GetCurrentUserGuildsError::LimitInvalid`]: enum.GetCurrentUserGuildsError.hLml#variant.LimitInvalid
    pub fn limit(mut self, limit: u64) -> Result<Self, GetCurrentUserGuildsError> {
        // <https://discordapp.com/developers/docs/resources/user#get-current-user-guilds-query-string-params>
        if !validate::get_current_user_guilds_limit(limit) {
            return Err(GetCurrentUserGuildsError::LimitInvalid);
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

poll_req!(GetCurrentUserGuilds<'_>, Vec<PartialGuild>);
