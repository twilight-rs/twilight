use crate::request::prelude::*;
use twilight_model::{
    id::{ChannelId, UserId},
    invite::{Invite, TargetUserType},
};

#[derive(Default, Serialize)]
struct CreateInviteFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_uses: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_user_type: Option<TargetUserType>,
}

/// Create an invite, with options.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let invite = client
///     .create_invite(channel_id)
///     .max_uses(3)
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateInvite<'a> {
    channel_id: ChannelId,
    fields: CreateInviteFields,
    fut: Option<Pending<'a, Invite>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateInvite<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: CreateInviteFields::default(),
            fut: None,
            http,
            reason: None,
        }
    }

    /// Set the maximum age for an invite.
    ///
    /// If no age is specified, Discord sets the age to 86400 seconds, or 24 hours.
    /// Set to 0 to never expire.
    pub fn max_age(mut self, max_age: u64) -> Self {
        self.fields.max_age.replace(max_age);

        self
    }

    /// Set the maximum uses for an invite, or 0 for infinite.
    ///
    /// Discord defaults this to 0, or infinite.
    pub fn max_uses(mut self, max_uses: u64) -> Self {
        self.fields.max_uses.replace(max_uses);

        self
    }

    /// Specify true if the invite should grant temporary membership. Defaults to false.
    pub fn temporary(mut self, temporary: bool) -> Self {
        self.fields.temporary.replace(temporary);

        self
    }

    /// Specify true if the invite should be unique. Defaults to false.
    ///
    /// If true, don't try to reuse a similar invite (useful for creating many unique one time use
    /// invites). Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#create-channel-invite
    pub fn unique(mut self, unique: bool) -> Self {
        self.fields.unique.replace(unique);

        self
    }

    /// Set the target user for this invite.
    pub fn target_user(mut self, target_user: UserId) -> Self {
        self.fields.target_user.replace(target_user.0.to_string());

        self
    }

    /// Set the target user type for this invite.
    pub fn target_user_type(mut self, target_user_type: TargetUserType) -> Self {
        self.fields.target_user_type.replace(target_user_type);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                headers,
                Route::CreateInvite {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                Route::CreateInvite {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for CreateInvite<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(CreateInvite<'_>, Invite);
