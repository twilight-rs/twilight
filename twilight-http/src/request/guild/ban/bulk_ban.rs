//! Builder and response body for the Bulk Ban request.

use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{GuildMarker, UserMarker},
};
use twilight_validate::request::{
    ValidationError, audit_reason as validate_audit_reason,
    bulk_guild_ban_delete_message_seconds as validate_bulk_guild_ban_delete_message_seconds,
    bulk_guild_ban_users as validate_bulk_guild_ban_users,
};

/// Response body for the [`BulkGuildBan`] request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BulkGuildBanResponseBody {
    /// List of user IDs that were successfully banned.
    pub banned_users: Vec<Id<UserMarker>>,
    /// List of user IDs that were not banned.
    ///
    /// A user ban may have failed for reasons including, but not limited to, a
    /// server-side or the user already being banned.
    pub failed_users: Vec<Id<UserMarker>>,
}

#[derive(Serialize)]
struct BulkGuildBanFields<'a> {
    /// Number of seconds to delete messages for, between `0` and `604800`.
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_message_seconds: Option<u32>,
    /// List of user IDs to ban from the guild.
    user_ids: &'a [Id<UserMarker>],
}

/// Ban up to 200 users from a guild in bulk, optionally deleting their recent
/// messages and specifying an Audit Log reason.
///
/// Requires both the [`BAN_MEMBERS`] and [`MANAGE_GUILD`] permissions.
///
/// # Examples
///
/// Ban a list of users from a guild along with the last day's messages:
///
/// ```no_run
/// use twilight_http::{Client, request::AuditLogReason};
/// use twilight_model::id::{marker::UserMarker, Id};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # fn determine_users_to_ban() -> Vec<Id<UserMarker>> { Vec::new() }
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(1);
/// let user_ids: Vec<Id<UserMarker>> = determine_users_to_ban();
/// client
///     .bulk_guild_ban(guild_id, &user_ids)
///     .delete_message_seconds(86_400)
///     .reason("insulting my little pony")
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`BAN_MEMBERS`]: twilight_model::guild::Permissions::BAN_MEMBERS
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct BulkGuildBan<'a> {
    fields: Result<BulkGuildBanFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> BulkGuildBan<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_ids: &'a [Id<UserMarker>],
    ) -> Self {
        let fields = Ok(BulkGuildBanFields {
            delete_message_seconds: None,
            user_ids,
        })
        .and_then(|fields| {
            validate_bulk_guild_ban_users(user_ids)?;

            Ok(fields)
        });

        Self {
            fields,
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Set the number of seconds' worth of messages to delete.
    ///
    /// The number of seconds must be less than or equal to `604_800` (this is equivalent to `7` days).
    ///
    /// # Errors
    ///
    /// Returns an error of type [`BulkGuildBanDeleteMessageSeconds`] if the
    /// number of seconds is greater than `604_800` (this is equivalent to `7` days).
    ///
    /// [`BulkGuildBanDeleteMessageSeconds`]: twilight_validate::request::ValidationErrorType::BulkGuildBanDeleteMessageSeconds
    pub fn delete_message_seconds(mut self, seconds: u32) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_bulk_guild_ban_delete_message_seconds(seconds)?;
            fields.delete_message_seconds = Some(seconds);

            Ok(fields)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for BulkGuildBan<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for BulkGuildBan<'_> {
    type Output = Result<Response<BulkGuildBanResponseBody>, Error>;

    type IntoFuture = ResponseFuture<BulkGuildBanResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for BulkGuildBan<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::BulkGuildBan {
            guild_id: self.guild_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
