use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, Pending, Request},
    routing::Route,
};
use twilight_model::id::{GuildId, UserId};

/// Remove a ban from a user in a guild.
///
/// # Examples
///
/// Unban user `200` from guild `100`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::{GuildId, UserId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
///
/// let guild_id = GuildId(100);
/// let user_id = UserId(200);
///
/// client.delete_ban(guild_id, user_id).await?;
/// # Ok(()) }
/// ```
pub struct DeleteBan<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
    reason: Option<String>,
}

impl<'a> DeleteBan<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            user_id,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::DeleteBan {
            guild_id: self.guild_id.0,
            user_id: self.user_id.0,
        });

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.verify(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteBan<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteBan<'_>, ());
