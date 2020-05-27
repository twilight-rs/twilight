use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::GuildPrune,
    id::{GuildId, RoleId},
};

#[derive(Clone, Debug)]
pub enum CreateGuildPruneError {
    /// The number of days is 0.
    DaysInvalid,
}

impl Display for CreateGuildPruneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DaysInvalid => f.write_str("the number of days is invalid"),
        }
    }
}

impl Error for CreateGuildPruneError {}

#[derive(Default)]
struct CreateGuildPruneFields {
    compute_prune_count: Option<bool>,
    days: Option<u64>,
    include_roles: Vec<u64>,
}

pub struct CreateGuildPrune<'a> {
    fields: CreateGuildPruneFields,
    guild_id: GuildId,
    fut: Option<Pending<'a, Option<GuildPrune>>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: CreateGuildPruneFields::default(),
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    /// List of roles to include when pruning.
    pub fn include_roles(mut self, roles: impl Iterator<Item = RoleId>) -> Self {
        let roles = roles.map(|e| e.0).collect::<Vec<_>>();

        self.fields.include_roles = roles;

        self
    }

    pub fn compute_prune_count(mut self, compute_prune_count: bool) -> Self {
        self.fields.compute_prune_count.replace(compute_prune_count);

        self
    }

    /// Set the number of days that a user must be inactive before being
    /// pruned.
    ///
    /// The number of days must be greater than 0.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildPruneError::DaysInvalid`] if the number of days is
    /// 0.
    ///
    /// [`CreateGuildPruneError::DaysInvalid`]: enum.CreateGuildPruneError.html#variant.DaysInvalid
    pub fn days(mut self, days: u64) -> Result<Self, CreateGuildPruneError> {
        if !validate::guild_prune_days(days) {
            return Err(CreateGuildPruneError::DaysInvalid);
        }

        self.fields.days.replace(days);

        Ok(self)
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                headers,
                Route::CreateGuildPrune {
                    compute_prune_count: self.fields.compute_prune_count,
                    days: self.fields.days,
                    guild_id: self.guild_id.0,
                    include_roles: self.fields.include_roles.clone(),
                },
            ))
        } else {
            Request::from(Route::CreateGuildPrune {
                compute_prune_count: self.fields.compute_prune_count,
                days: self.fields.days,
                guild_id: self.guild_id.0,
                include_roles: self.fields.include_roles.clone(),
            })
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateGuildPrune<'_>, Option<GuildPrune>);
