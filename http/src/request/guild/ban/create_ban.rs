use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::{GuildId, UserId};

/// The error created when the ban can not be created as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CreateBanError {
    /// The number of days' worth of messages to delete is greater than 7.
    DeleteMessageDaysInvalid {
        /// Provided number of days' worth of messages to delete.
        days: u64,
    },
}

impl Display for CreateBanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DeleteMessageDaysInvalid { .. } => {
                f.write_str("the number of days' worth of messages to delete is invalid")
            }
        }
    }
}

impl Error for CreateBanError {}

#[derive(Default)]
struct CreateBanFields {
    delete_message_days: Option<u64>,
    reason: Option<String>,
}

/// Bans a user from a guild, optionally with the number of days' worth of
/// messages to delete and the reason.
///
/// # Examples
///
/// Ban user `200` from guild `100`, deleting
/// 1 day's worth of messages, for the reason `"memes"`:
///
/// ```rust,no_run
/// use twilight_http::{request::AuditLogReason, Client};
/// use twilight_model::id::{GuildId, UserId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let guild_id = GuildId(100);
/// let user_id = UserId(200);
/// client.create_ban(guild_id, user_id)
///     .delete_message_days(1)?
///     .reason("memes")?
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateBan<'a> {
    fields: CreateBanFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> CreateBan<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: CreateBanFields::default(),
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    /// Set the number of days' worth of messages to delete.
    ///
    /// The number of days must be less than or equal to 7.
    ///
    /// # Errors
    ///
    /// Returns [`CreateBanError::DeleteMessageDaysInvalid`] if the number of days
    /// is greater than 7.
    pub fn delete_message_days(mut self, days: u64) -> Result<Self, CreateBanError> {
        if !validate::ban_delete_message_days(days) {
            return Err(CreateBanError::DeleteMessageDaysInvalid { days });
        }

        self.fields.delete_message_days.replace(days);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::CreateBan {
                delete_message_days: self.fields.delete_message_days,
                guild_id: self.guild_id.0,
                reason: self.fields.reason.clone(),
                user_id: self.user_id.0,
            },
        ))));

        Ok(())
    }
}

impl<'a> AuditLogReason for CreateBan<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.fields
            .reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(CreateBan<'_>, ());
