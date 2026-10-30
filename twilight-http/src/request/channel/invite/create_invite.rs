use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::invite::{Invite, TargetType},
    id::{
        Id,
        marker::{ApplicationMarker, ChannelMarker, UserMarker},
    },
};
use twilight_validate::request::{
    ValidationError, audit_reason as validate_audit_reason,
    invite_max_age as validate_invite_max_age, invite_max_uses as validate_invite_max_uses,
};

#[derive(Serialize)]
struct CreateInviteBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_uses: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_user_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_type: Option<TargetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique: Option<bool>,
}

struct CreateInvitePath {
    channel_id: Id<ChannelMarker>,
}

/// Create an invite, with options.
///
/// Requires the [`CREATE_INVITE`] permission.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(123);
/// let invite = client.create_invite(channel_id).max_uses(3).await?;
/// # Ok(()) }
/// ```
///
/// [`CREATE_INVITE`]: twilight_model::guild::Permissions::CREATE_INVITE
#[must_use = "requests must be configured and executed"]
pub struct CreateInvite<'a> {
    body: Result<CreateInviteBody, ValidationError>,
    http: &'a Client,
    path: CreateInvitePath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateInvite<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            body: Ok(CreateInviteBody {
                max_age: None,
                max_uses: None,
                temporary: None,
                target_application_id: None,
                target_user_id: None,
                target_type: None,
                unique: None,
            }),
            http,
            path: CreateInvitePath { channel_id },
            reason: Ok(None),
        }
    }

    /// Set the maximum age for an invite.
    ///
    /// If no age is specified, Discord sets the age to 86400 seconds, or 24 hours.
    /// Set to 0 to never expire.
    ///
    /// # Examples
    ///
    /// Create an invite for a channel with a maximum of 1 use and an age of 1
    /// hour:
    ///
    /// ```no_run
    /// use std::env;
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let invite = client
    ///     .create_invite(Id::new(1))
    ///     .max_age(60 * 60)
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// println!("invite code: {}", invite.code);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error of type [`InviteMaxAge`] if the age is invalid.
    ///
    /// [`InviteMaxAge`]: twilight_validate::request::ValidationErrorType::InviteMaxAge
    pub fn max_age(mut self, max_age: u32) -> Self {
        self.body = self.body.and_then(|mut body| {
            validate_invite_max_age(max_age)?;
            body.max_age = Some(max_age);

            Ok(body)
        });

        self
    }

    /// Set the maximum uses for an invite, or 0 for infinite.
    ///
    /// Discord defaults this to 0, or infinite.
    ///
    /// # Examples
    ///
    /// Create an invite for a channel with a maximum of 5 uses:
    ///
    /// ```no_run
    /// use std::env;
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let invite = client
    ///     .create_invite(Id::new(1))
    ///     .max_uses(5)
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// println!("invite code: {}", invite.code);
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error of type [`InviteMaxUses`] if the uses is invalid.
    ///
    /// [`InviteMaxUses`]: twilight_validate::request::ValidationErrorType::InviteMaxUses
    pub fn max_uses(mut self, max_uses: u16) -> Self {
        self.body = self.body.and_then(|mut body| {
            validate_invite_max_uses(max_uses)?;
            body.max_uses = Some(max_uses);

            Ok(body)
        });

        self
    }

    /// Set the target application ID for this invite.
    ///
    /// This only works if [`target_type`] is set to [`TargetType::EmbeddedApplication`].
    ///
    /// [`target_type`]: Self::target_type
    pub const fn target_application_id(
        mut self,
        target_application_id: Id<ApplicationMarker>,
    ) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.target_application_id = Some(target_application_id);
        }

        self
    }

    /// Set the target user id for this invite.
    pub const fn target_user_id(mut self, target_user_id: Id<UserMarker>) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.target_user_id = Some(target_user_id);
        }

        self
    }

    /// Set the target type for this invite.
    pub const fn target_type(mut self, target_type: TargetType) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.target_type = Some(target_type);
        }

        self
    }

    /// Specify true if the invite should grant temporary membership.
    ///
    /// Defaults to false.
    pub const fn temporary(mut self, temporary: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.temporary = Some(temporary);
        }

        self
    }

    /// Specify true if the invite should be unique. Defaults to false.
    ///
    /// If true, don't try to reuse a similar invite (useful for creating many
    /// unique one time use invites). See [Discord Docs/Create Channel Invite].
    ///
    /// [Discord Docs/Create Channel Invite]: https://discord.com/developers/docs/resources/channel#create-channel-invite
    pub const fn unique(mut self, unique: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.unique = Some(unique);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateInvite<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateInvite<'_> {
    type Output = Result<Response<Invite>, Error>;

    type IntoFuture = ResponseFuture<Invite>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateInvite<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::CreateInvite {
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
    use super::CreateInvite;
    use crate::Client;
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn max_age() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let mut builder = CreateInvite::new(&client, Id::new(1)).max_age(0);
        assert_eq!(Some(0), builder.body.as_ref().unwrap().max_age);
        builder = builder.max_age(604_800);
        assert_eq!(Some(604_800), builder.body.as_ref().unwrap().max_age);
        builder = builder.max_age(604_801);
        assert!(builder.body.is_err());

        Ok(())
    }

    #[test]
    fn max_uses() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let mut builder = CreateInvite::new(&client, Id::new(1)).max_uses(0);
        assert_eq!(Some(0), builder.body.as_ref().unwrap().max_uses);
        builder = builder.max_uses(100);
        assert_eq!(Some(100), builder.body.as_ref().unwrap().max_uses);
        builder = builder.max_uses(101);
        assert!(builder.body.is_err());

        Ok(())
    }
}
