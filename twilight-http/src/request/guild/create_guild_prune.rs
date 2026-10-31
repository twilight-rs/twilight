use crate::{
    client::Client,
    error::Error,
    request::{
        self, AuditLogReason, Method, Request, TryIntoRequest,
        path::{Path, query_parameter::CsvQueryParameter},
    },
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildPrune,
    id::{
        Id,
        marker::{GuildMarker, RoleMarker},
    },
};
use twilight_validate::request::{
    ValidationError, audit_reason as validate_audit_reason,
    guild_prune_days as validate_guild_prune_days,
};

struct CreateGuildPrunePath<'a> {
    compute_prune_count: Option<bool>,
    days: Option<u16>,
    guild_id: Id<GuildMarker>,
    include_roles: &'a [Id<RoleMarker>],
}

/// Begin a guild prune.
///
/// See [Discord Docs/Begin Guild Prune].
///
/// # Permissions
///
/// If the target guild has the
/// [`PruneRequiresAdmin`][`GuildFeature::PruneRequiresAdmin`] feature enabled
/// then this requires the [`ADMINISTRATOR`][`Permissions::ADMINISTRATOR`] guild
/// permission; otherwise, this requires both the
/// [`MANAGE_GUILD`][`Permissions::MANAGE_GUILD`] and
/// [`KICK_MEMBERS`][`Permissions::KICK_MEMBERS`] permissions.
///
/// [`GuildFeature::PruneRequiresAdmin`]: twilight_model::guild::GuildFeature::PruneRequiresAdmin
/// [`Permissions::ADMINISTRATOR`]: twilight_model::guild::Permissions::ADMINISTRATOR
/// [`Permissions::KICK_MEMBERS`]: twilight_model::guild::Permissions::KICK_MEMBERS
/// [`Permissions::MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
/// [Discord Docs/Begin Guild Prune]: https://discord.com/developers/docs/resources/guild#begin-guild-prune
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildPrune<'a> {
    path: Result<CreateGuildPrunePath<'a>, ValidationError>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: Ok(CreateGuildPrunePath {
                compute_prune_count: None,
                days: None,
                guild_id,
                include_roles: &[],
            }),
            http,
            reason: Ok(None),
        }
    }

    /// List of roles to include when pruning.
    pub const fn include_roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.include_roles = roles;
        }

        self
    }

    /// Return the amount of pruned members. Discouraged for large guilds.
    pub const fn compute_prune_count(mut self, compute_prune_count: bool) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.compute_prune_count = Some(compute_prune_count);
        }

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
    pub fn days(mut self, days: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_guild_prune_days(days)?;
            path.days = Some(days);

            Ok(path)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildPrune<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
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
        let CreateGuildPrunePath {
            compute_prune_count,
            days,
            guild_id,
            include_roles,
        } = self.path.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .action("prune")
            .parameter(("compute_prune_count", compute_prune_count))
            .parameter(("days", days))
            .parameter(CsvQueryParameter::new("include_roles", include_roles))
            .build()
            .map_err(Error::path)?;

        let mut request = Request::builder(Method::Post, path);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
