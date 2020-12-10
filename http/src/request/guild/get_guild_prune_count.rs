use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::GuildPrune,
    id::{GuildId, RoleId},
};

/// The error created when the guild prune count can not be requested as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GetGuildPruneCountError {
    /// The number of days is 0.
    DaysInvalid,
}

impl Display for GetGuildPruneCountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DaysInvalid => f.write_str("the number of days is invalid"),
        }
    }
}

impl Error for GetGuildPruneCountError {}

#[derive(Default)]
struct GetGuildPruneCountFields {
    days: Option<u64>,
    include_roles: Vec<u64>,
}

/// Get the counts of guild members to be pruned.
pub struct GetGuildPruneCount<'a> {
    fields: GetGuildPruneCountFields,
    fut: Option<Pending<'a, GuildPrune>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPruneCount<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: GetGuildPruneCountFields::default(),
            fut: None,
            guild_id,
            http,
        }
    }

    /// Set the number of days that a user must be inactive before being
    /// able to be pruned.
    ///
    /// The number of days must be greater than 0.
    ///
    /// # Errors
    ///
    /// Returns [`GetGuildPruneCountError::DaysInvalid`] if the number of days
    /// is 0.
    pub fn days(mut self, days: u64) -> Result<Self, GetGuildPruneCountError> {
        if validate::guild_prune_days(days) {
            return Err(GetGuildPruneCountError::DaysInvalid);
        }

        self.fields.days.replace(days);

        Ok(self)
    }

    /// List of roles to include when calculating prune count
    pub fn include_roles(mut self, roles: impl Iterator<Item = RoleId>) -> Self {
        let roles = roles.map(|e| e.0).collect::<Vec<_>>();

        self.fields.include_roles = roles;

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildPruneCount {
                days: self.fields.days,
                guild_id: self.guild_id.0,
                include_roles: self.fields.include_roles.clone(),
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildPruneCount<'_>, GuildPrune);
