use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{guild::GuildPrune, id::GuildId};

#[derive(Clone, Debug)]
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
}

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
    ///
    /// [`GetGuildPruneCountError::DaysInvalid`]: enum.GetGuildPruneCountError.html#variant.DaysInvalid
    pub fn days(mut self, days: u64) -> Result<Self, GetGuildPruneCountError> {
        if validate::guild_prune_days(days) {
            return Err(GetGuildPruneCountError::DaysInvalid);
        }

        self.fields.days.replace(days);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildPruneCount {
                days: self.fields.days,
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildPruneCount<'_>, GuildPrune);
