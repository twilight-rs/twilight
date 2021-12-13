use crate::{
    client::Client,
    request::{AuditLogReason, AuditLogReasonError, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{GuildId, UserId};
use twilight_validate::misc::{
    create_guild_ban_delete_message_days as validate_create_guild_ban_delete_message_days,
    ValidationError,
};

struct CreateBanFields<'a> {
    delete_message_days: Option<u64>,
    reason: Option<&'a str>,
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
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = GuildId::new(100).expect("non zero");
/// let user_id = UserId::new(200).expect("non zero");
/// client.create_ban(guild_id, user_id)
///     .delete_message_days(1)?
///     .reason("memes")?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateBan<'a> {
    fields: CreateBanFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> CreateBan<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: CreateBanFields {
                delete_message_days: None,
                reason: None,
            },
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
    /// Returns an error of type [`CreateGuildBanDeleteMessageDays`] if the
    /// number of days is greater than 7.
    ///
    /// [`CreateGuildBanDeleteMessageDays`]: twilight_validate::misc::ValidationErrorType::CreateGuildBanDeleteMessageDays
    pub const fn delete_message_days(mut self, days: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_create_guild_ban_delete_message_days(days) {
            return Err(source);
        }

        self.fields.delete_message_days = Some(days);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let request = Request::from_route(&Route::CreateBan {
            delete_message_days: self.fields.delete_message_days,
            guild_id: self.guild_id.get(),
            reason: self.fields.reason,
            user_id: self.user_id.get(),
        });

        self.http.request(request)
    }
}

impl<'a> AuditLogReason<'a> for CreateBan<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.fields
            .reason
            .replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}
