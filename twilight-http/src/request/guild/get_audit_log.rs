use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::audit_log::{AuditLog, AuditLogEventType},
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};
use twilight_validate::request::{
    ValidationError, get_guild_audit_log_limit as validate_get_guild_audit_log_limit,
};

struct GetAuditLogPath {
    action_type: Option<AuditLogEventType>,
    after: Option<u64>,
    before: Option<u64>,
    guild_id: Id<GuildMarker>,
    limit: Option<u16>,
    user_id: Option<Id<UserMarker>>,
}

/// Get the audit log for a guild.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("token".to_owned());
///
/// let guild_id = Id::new(101);
/// let audit_log = client.audit_log(guild_id).await?.model().await?;
///
/// for entry in audit_log.entries {
///     println!("ID: {}", entry.id);
///     println!("  Action Type: {}", u16::from(entry.action_type));
///     println!("  Changes:");
///
///     for change in entry.changes {
///         println!("{change:?}");
///     }
/// }
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetAuditLog<'a> {
    http: &'a Client,
    path: Result<GetAuditLogPath, ValidationError>,
}

impl<'a> GetAuditLog<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: Ok(GetAuditLogPath {
                action_type: None,
                after: None,
                before: None,
                guild_id,
                limit: None,
                user_id: None,
            }),
        }
    }

    /// Filter by an action type.
    pub const fn action_type(mut self, action_type: AuditLogEventType) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.action_type = Some(action_type);
        }

        self
    }

    /// Get audit log entries after the entry specified.
    pub const fn after(mut self, after: u64) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(after);
        }

        self
    }

    /// Get audit log entries before the entry specified.
    pub const fn before(mut self, before: u64) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.before = Some(before);
        }

        self
    }

    /// Set the maximum number of audit logs to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetGuildAuditLog`] if the `limit` is 0 or
    /// greater than 100.
    ///
    /// [`GetGuildAuditLog`]: twilight_validate::request::ValidationErrorType::GetGuildAuditLog
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_get_guild_audit_log_limit(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }

    /// Filter audit log for entries from a user.
    ///
    /// This is the user who did the auditable action, not the target of the auditable action.
    pub const fn user_id(mut self, user_id: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.user_id = Some(user_id);
        }

        self
    }
}

impl IntoFuture for GetAuditLog<'_> {
    type Output = Result<Response<AuditLog>, Error>;

    type IntoFuture = ResponseFuture<AuditLog>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetAuditLog<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = self.path.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetAuditLogs {
            action_type: path.action_type.map(|x| u64::from(u16::from(x))),
            after: path.after,
            before: path.before,
            guild_id: path.guild_id.get(),
            limit: path.limit,
            user_id: path.user_id.map(Id::get),
        }))
    }
}
