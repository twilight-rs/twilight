use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel},
    id::{marker::ChannelMarker, Id},
};
use twilight_validate::channel::{
    name as validate_name, rate_limit_per_user as validate_rate_limit_per_user,
    ChannelValidationError,
};

#[derive(Serialize)]
struct UpdateThreadFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u64>,
}

/// Update a thread.
///
/// All fields are optional. The minimum length of the name is 1 UTF-16
/// characters and the maximum is 100 UTF-16 characters.
#[must_use = "requests must be configured and executed"]
pub struct UpdateThread<'a> {
    channel_id: Id<ChannelMarker>,
    fields: UpdateThreadFields<'a>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> UpdateThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: UpdateThreadFields {
                archived: None,
                auto_archive_duration: None,
                invitable: None,
                locked: None,
                name: None,
                rate_limit_per_user: None,
            },
            http,
            reason: None,
        }
    }

    /// Set whether the thread is archived.
    ///
    /// Requires that the user have [`SEND_MESSAGES`] in the thread. However, if
    /// the thread is locked, the user must have [`MANAGE_THREADS`].
    ///
    /// [`SEND_MESSAGES`]: twilight_model::guild::Permissions::SEND_MESSAGES
    /// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
    pub const fn archived(mut self, archived: bool) -> Self {
        self.fields.archived = Some(archived);

        self
    }

    /// Set the thread's auto archive duration.
    ///
    /// Values of [`ThreeDays`] and [`Week`] require the guild to be boosted.
    /// The guild's features will indicate if a guild is able to use these
    /// settings.
    ///
    /// [`ThreeDays`]: twilight_model::channel::thread::AutoArchiveDuration::ThreeDays
    /// [`Week`]: twilight_model::channel::thread::AutoArchiveDuration::Week
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        self.fields.auto_archive_duration = Some(auto_archive_duration);

        self
    }

    /// Whether non-moderators can add other non-moderators to a thread.
    pub const fn invitable(mut self, invitable: bool) -> Self {
        self.fields.invitable = Some(invitable);

        self
    }

    /// Set whether the thread is locked.
    ///
    /// If the thread is already locked, only users with [`MANAGE_THREADS`] can
    /// unlock it.
    ///
    /// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
    pub const fn locked(mut self, locked: bool) -> Self {
        self.fields.locked = Some(locked);

        self
    }

    /// Set the name of the thread.
    ///
    /// Must be between 1 and 100 characters in length.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameInvalid`] if the name is invalid.
    ///
    /// [`NameInvalid`]: twilight_validate::channel::ChannelValidationErrorType::NameInvalid
    pub fn name(mut self, name: &'a str) -> Result<Self, ChannelValidationError> {
        validate_name(name)?;

        self.fields.name = Some(name);

        Ok(self)
    }

    /// Set the number of seconds that a user must wait before before they are
    /// able to send another message.
    ///
    /// The minimum is 0 and the maximum is 21600. Refer to [the discord docs]
    /// for more details.  This is also known as "Slow Mode".
    ///
    /// # Errors
    ///
    /// Returns an error of type [`RateLimitPerUserInvalid`] if the name is
    /// invalid.
    ///
    /// [`RateLimitPerUserInvalid`]: twilight_validate::channel::ChannelValidationErrorType::RateLimitPerUserInvalid
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
    pub const fn rate_limit_per_user(
        mut self,
        rate_limit_per_user: u64,
    ) -> Result<Self, ChannelValidationError> {
        if let Err(source) = validate_rate_limit_per_user(rate_limit_per_user) {
            return Err(source);
        }

        self.fields.rate_limit_per_user = Some(rate_limit_per_user);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Channel> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateThread<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateThread<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateChannel {
            channel_id: self.channel_id.get(),
        })
        .json(&self.fields)?;

        if let Some(reason) = &self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        Ok(request.build())
    }
}

#[cfg(test)]
mod tests {
    use super::{UpdateThread, UpdateThreadFields};
    use crate::{
        request::{Request, TryIntoRequest},
        routing::Route,
        Client,
    };
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".to_string());
        let channel_id = Id::new(123);

        let actual = UpdateThread::new(&client, channel_id)
            .rate_limit_per_user(60)?
            .try_into_request()?;

        let expected = Request::builder(&Route::UpdateChannel {
            channel_id: channel_id.get(),
        })
        .json(&UpdateThreadFields {
            archived: None,
            auto_archive_duration: None,
            invitable: None,
            locked: None,
            name: None,
            rate_limit_per_user: Some(60),
        })?
        .build();

        assert_eq!(expected.body(), actual.body());
        assert_eq!(expected.path(), actual.path());
        assert_eq!(expected.ratelimit_path(), actual.ratelimit_path());

        Ok(())
    }
}
