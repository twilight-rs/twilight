use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, validate_inner, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::GuildPrune,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

/// The error created when the guild prune can not be created as configured.
#[derive(Debug)]
pub struct CreateGuildPruneError {
    kind: CreateGuildPruneErrorType,
}

impl CreateGuildPruneError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CreateGuildPruneErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CreateGuildPruneErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for CreateGuildPruneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CreateGuildPruneErrorType::DaysInvalid { .. } => {
                f.write_str("the number of days is invalid")
            }
        }
    }
}

impl Error for CreateGuildPruneError {}

/// Type of [`CreateGuildPruneError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CreateGuildPruneErrorType {
    /// The number of days is 0.
    DaysInvalid,
}

struct CreateGuildPruneFields<'a> {
    compute_prune_count: Option<bool>,
    days: Option<u64>,
    include_roles: &'a [Id<RoleMarker>],
}

/// Begin a guild prune.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#begin-guild-prune
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
    /// Returns a [`CreateGuildPruneErrorType::DaysInvalid`] error type if the
    /// number of days is 0.
    pub const fn days(mut self, days: u64) -> Result<Self, CreateGuildPruneError> {
        if !validate_inner::guild_prune_days(days) {
            return Err(CreateGuildPruneError {
                kind: CreateGuildPruneErrorType::DaysInvalid,
            });
        }

        self.fields.days = Some(days);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildPrune> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildPrune<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildPrune<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
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
