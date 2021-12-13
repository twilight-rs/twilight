use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    guild::audit_log::{AuditLog, AuditLogEventType},
    id::{GuildId, UserId},
};
use twilight_validate::misc::{
    get_guild_audit_log_limit as validate_get_guild_audit_log_limit, ValidationError,
};

struct GetAuditLogFields {
    action_type: Option<AuditLogEventType>,
    before: Option<u64>,
    limit: Option<u64>,
    user_id: Option<UserId>,
}

/// Get the audit log for a guild.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("token".to_owned());
///
/// let guild_id = GuildId::new(101).expect("non zero");
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
///         println!("{:?}", change);
///     }
/// }
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetAuditLog<'a> {
    fields: GetAuditLogFields,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetAuditLog<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
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
    /// [`GetGuildAuditLog`]: twilight_validate::misc::ValidationErrorType::GetGuildAuditLog
    pub const fn limit(mut self, limit: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_guild_audit_log_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Filter audit log for entries from a user.
    ///
    /// This is the user who did the auditable action, not the target of the auditable action.
    pub const fn user_id(mut self, user_id: UserId) -> Self {
        self.fields.user_id = Some(user_id);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<AuditLog> {
        let request = Request::from_route(&Route::GetAuditLogs {
            action_type: self.fields.action_type.map(|x| x as u64),
            before: self.fields.before,
            guild_id: self.guild_id.get(),
            limit: self.fields.limit,
            user_id: self.fields.user_id.map(UserId::get),
        });

        self.http.request(request)
    }
}
