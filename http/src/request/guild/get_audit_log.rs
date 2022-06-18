use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    guild::audit_log::{AuditLog, AuditLogEventType},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use twilight_validate::request::{
    get_guild_audit_log_limit as validate_get_guild_audit_log_limit, ValidationError,
};

struct GetAuditLogFields {
    action_type: Option<AuditLogEventType>,
    before: Option<u64>,
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
/// let audit_log = client
///     .audit_log(guild_id)
///     .exec()
///     .await?
///     .model()
///     .await?;
///
/// for entry in audit_log.entries {
///     println!("ID: {}", entry.id);
///     println!("  Action Type: {}", entry.action_type as u8);
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
    fields: GetAuditLogFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetAuditLog<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: GetAuditLogFields {
                action_type: None,
                before: None,
                limit: None,
                user_id: None,
            },
            guild_id,
            http,
        }
    }

    /// Filter by an action type.
    pub const fn action_type(mut self, action_type: AuditLogEventType) -> Self {
        self.fields.action_type = Some(action_type);

        self
    }

    /// Get audit log entries before the entry specified.
    pub const fn before(mut self, before: u64) -> Self {
        self.fields.before = Some(before);

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
    pub const fn limit(mut self, limit: u16) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_guild_audit_log_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Filter audit log for entries from a user.
    ///
    /// This is the user who did the auditable action, not the target of the auditable action.
    pub const fn user_id(mut self, user_id: Id<UserMarker>) -> Self {
        self.fields.user_id = Some(user_id);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<AuditLog> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetAuditLog<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Ok(Request::from_route(&Route::GetAuditLogs {
            action_type: self.fields.action_type.map(|x| x as u64),
            before: self.fields.before,
            guild_id: self.guild_id.get(),
            limit: self.fields.limit,
            user_id: self.fields.user_id.map(Id::get),
        }))
    }
}
