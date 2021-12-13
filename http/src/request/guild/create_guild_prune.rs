use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    guild::GuildPrune,
    id::{GuildId, RoleId},
};
use twilight_validate::misc::{guild_prune_days as validate_guild_prune_days, ValidationError};

struct CreateGuildPruneFields<'a> {
    compute_prune_count: Option<bool>,
    days: Option<u64>,
    include_roles: &'a [RoleId],
}

/// Begin a guild prune.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#begin-guild-prune
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildPrune<'a> {
    fields: CreateGuildPruneFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: CreateGuildPruneFields {
                compute_prune_count: None,
                days: None,
                include_roles: &[],
            },
            guild_id,
            http,
            reason: None,
        }
    }

    /// List of roles to include when pruning.
    pub const fn include_roles(mut self, roles: &'a [RoleId]) -> Self {
        self.fields.include_roles = roles;

        self
    }

    /// Return the amount of pruned members. Discouraged for large guilds.
    pub const fn compute_prune_count(mut self, compute_prune_count: bool) -> Self {
        self.fields.compute_prune_count = Some(compute_prune_count);

        self
    }

    /// Set the number of days that a user must be inactive before being pruned.
    ///
    /// The number of days must be greater than 0.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GuildPruneDays`] if the number of days is 0
    /// or more than 30.
    ///
    /// [`GuildPruneDays`]: twilight_validate::misc::ValidationErrorType::GuildPruneDays
    pub const fn days(mut self, days: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_guild_prune_days(days) {
            return Err(source);
        }

        self.fields.days = Some(days);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildPrune> {
        let mut request = Request::builder(&Route::CreateGuildPrune {
            compute_prune_count: self.fields.compute_prune_count,
            days: self.fields.days,
            guild_id: self.guild_id.get(),
            include_roles: self.fields.include_roles,
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = match request::audit_header(reason) {
                Ok(header) => header,
                Err(source) => return ResponseFuture::error(source),
            };

            request = request.headers(header);
        }

        self.http.request(request.build())
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildPrune<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}
