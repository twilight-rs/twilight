use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildPrune,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason, guild_prune_days as validate_guild_prune_days,
    ValidationError,
};

struct CreateGuildPruneFields<'a> {
    compute_prune_count: Option<bool>,
    days: Option<u16>,
    include_roles: &'a [Id<RoleMarker>],
}

/// Begin a guild prune.
///
/// See [Discord Docs/Begin Guild Prune].
///
/// [Discord Docs/Begin Guild Prune]: https://discord.com/developers/docs/resources/guild#begin-guild-prune
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildPrune<'a> {
    fields: CreateGuildPruneFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
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
    pub const fn include_roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
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
    /// [`GuildPruneDays`]: twilight_validate::request::ValidationErrorType::GuildPruneDays
    pub const fn days(mut self, days: u16) -> Result<Self, ValidationError> {
        #[allow(clippy::question_mark)]
        if let Err(source) = validate_guild_prune_days(days) {
            return Err(source);
        }

        self.fields.days = Some(days);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<GuildPrune> {
        self.into_future()
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildPrune<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl IntoFuture for CreateGuildPrune<'_> {
    type Output = Result<Response<GuildPrune>, Error>;

    type IntoFuture = ResponseFuture<GuildPrune>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateGuildPrune<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateGuildPrune {
            compute_prune_count: self.fields.compute_prune_count,
            days: self.fields.days,
            guild_id: self.guild_id.get(),
            include_roles: self.fields.include_roles,
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
