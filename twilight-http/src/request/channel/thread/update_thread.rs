use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::{Channel, ChannelFlags, thread::AutoArchiveDuration},
    id::{
        Id,
        marker::{ChannelMarker, TagMarker},
    },
};
use twilight_validate::{
    channel::{
        ChannelValidationError, name as validate_name,
        rate_limit_per_user as validate_rate_limit_per_user,
    },
    request::{ValidationError, audit_reason as validate_audit_reason},
};

#[derive(Serialize)]
struct UpdateThreadBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    applied_tags: Option<Nullable<&'a [Id<TagMarker>]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<Nullable<ChannelFlags>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u16>,
}

struct UpdateThreadPath {
    channel_id: Id<ChannelMarker>,
}

/// Update a thread.
///
/// All fields are optional. The minimum length of the name is 1 UTF-16
/// characters and the maximum is 100 UTF-16 characters.
#[must_use = "requests must be configured and executed"]
pub struct UpdateThread<'a> {
    body: Result<UpdateThreadBody<'a>, ChannelValidationError>,
    http: &'a Client,
    path: UpdateThreadPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            body: Ok(UpdateThreadBody {
                applied_tags: None,
                archived: None,
                auto_archive_duration: None,
                flags: None,
                invitable: None,
                locked: None,
                name: None,
                rate_limit_per_user: None,
            }),
            http,
            path: UpdateThreadPath { channel_id },
            reason: Ok(None),
        }
    }

    /// Set the forum thread's applied tags.
    pub const fn applied_tags(mut self, applied_tags: Option<&'a [Id<TagMarker>]>) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.applied_tags = Some(Nullable(applied_tags));
        }

        self
    }

    /// Set whether the thread is archived.
    ///
    /// Requires that the user have [`SEND_MESSAGES`] in the thread. However, if
    /// the thread is locked, the user must have [`MANAGE_THREADS`].
    ///
    /// [`SEND_MESSAGES`]: twilight_model::guild::Permissions::SEND_MESSAGES
    /// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
    pub const fn archived(mut self, archived: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.archived = Some(archived);
        }

        self
    }

    /// Set the thread's auto archive duration.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.auto_archive_duration = Some(auto_archive_duration);
        }

        self
    }

    /// Sets the thread's flags.
    pub const fn flags(mut self, flags: Option<ChannelFlags>) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.flags = Some(Nullable(flags));
        }

        self
    }

    /// Whether non-moderators can add other non-moderators to a thread.
    pub const fn invitable(mut self, invitable: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.invitable = Some(invitable);
        }

        self
    }

    /// Set whether the thread is locked.
    ///
    /// If the thread is already locked, only users with [`MANAGE_THREADS`] can
    /// unlock it.
    ///
    /// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
    pub const fn locked(mut self, locked: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.locked = Some(locked);
        }

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
    pub fn name(mut self, name: &'a str) -> Self {
        self.body = self.body.and_then(|mut body| {
            validate_name(name)?;
            body.name = Some(name);

            Ok(body)
        });

        self
    }

    /// Set the number of seconds that a user must wait before before they are
    /// able to send another message.
    ///
    /// The minimum is 0 and the maximum is 21600. This is also known as "Slow
    /// Mode". See [Discord Docs/Channel Object].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`RateLimitPerUserInvalid`] if the name is
    /// invalid.
    ///
    /// [`RateLimitPerUserInvalid`]: twilight_validate::channel::ChannelValidationErrorType::RateLimitPerUserInvalid
    /// [Discord Docs/Channel Object]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn rate_limit_per_user(mut self, rate_limit_per_user: u16) -> Self {
        self.body = self.body.and_then(|mut body| {
            validate_rate_limit_per_user(rate_limit_per_user)?;
            body.rate_limit_per_user = Some(rate_limit_per_user);

            Ok(body)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateThread<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateThread<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateThread<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateChannel {
            channel_id: self.path.channel_id.get(),
        })
        .json(&body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{UpdateThread, UpdateThreadBody};
    use crate::{
        Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".to_string());
        let channel_id = Id::new(123);

        let actual = UpdateThread::new(&client, channel_id)
            .rate_limit_per_user(60)
            .try_into_request()?;

        let expected = Request::builder(&Route::UpdateChannel {
            channel_id: channel_id.get(),
        })
        .json(&UpdateThreadBody {
            applied_tags: None,
            archived: None,
            auto_archive_duration: None,
            flags: None,
            invitable: None,
            locked: None,
            name: None,
            rate_limit_per_user: Some(60),
        })
        .build()?;

        assert_eq!(expected.body(), actual.body());
        assert_eq!(expected.path(), actual.path());

        Ok(())
    }
}
