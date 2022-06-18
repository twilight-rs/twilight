use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    id::{
        marker::{ApplicationMarker, ChannelMarker, UserMarker},
        Id,
    },
    invite::{Invite, TargetType},
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason, invite_max_age as validate_invite_max_age,
    invite_max_uses as validate_invite_max_uses, ValidationError,
};

#[derive(Serialize)]
struct CreateInviteFields {
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
/// let invite = client
///     .create_invite(channel_id)
///     .max_uses(3)?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`CREATE_INVITE`]: twilight_model::guild::Permissions::CREATE_INVITE
#[must_use = "requests must be configured and executed"]
pub struct CreateInvite<'a> {
    channel_id: Id<ChannelMarker>,
    fields: CreateInviteFields,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateInvite<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: CreateInviteFields {
                max_age: None,
                max_uses: None,
                temporary: None,
                target_application_id: None,
                target_user_id: None,
                target_type: None,
                unique: None,
            },
            http,
            reason: None,
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
    /// let invite = client.create_invite(Id::new(1))
    ///     .max_age(60 * 60)?
    ///     .exec()
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// println!("invite code: {}", invite.code);
    /// # Ok(()) }
    /// ```
    pub const fn max_age(mut self, max_age: u32) -> Result<Self, ValidationError> {
        if let Err(source) = validate_invite_max_age(max_age) {
            return Err(source);
        }

        self.fields.max_age = Some(max_age);

        Ok(self)
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
    /// let invite = client.create_invite(Id::new(1))
    ///     .max_uses(5)?
    ///     .exec()
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// println!("invite code: {}", invite.code);
    /// # Ok(()) }
    /// ```
    pub const fn max_uses(mut self, max_uses: u16) -> Result<Self, ValidationError> {
        if let Err(source) = validate_invite_max_uses(max_uses) {
            return Err(source);
        }

        self.fields.max_uses = Some(max_uses);

        Ok(self)
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
        self.fields.target_application_id = Some(target_application_id);

        self
    }

    /// Set the target user id for this invite.
    pub const fn target_user_id(mut self, target_user_id: Id<UserMarker>) -> Self {
        self.fields.target_user_id = Some(target_user_id);

        self
    }

    /// Set the target type for this invite.
    pub const fn target_type(mut self, target_type: TargetType) -> Self {
        self.fields.target_type = Some(target_type);

        self
    }

    /// Specify true if the invite should grant temporary membership.
    ///
    /// Defaults to false.
    pub const fn temporary(mut self, temporary: bool) -> Self {
        self.fields.temporary = Some(temporary);

        self
    }

    /// Specify true if the invite should be unique. Defaults to false.
    ///
    /// If true, don't try to reuse a similar invite (useful for creating many
    /// unique one time use invites). See [Discord Docs/Create Channel Invite].
    ///
    /// [Discord Docs/Create Channel Invite]: https://discord.com/developers/docs/resources/channel#create-channel-invite
    pub const fn unique(mut self, unique: bool) -> Self {
        self.fields.unique = Some(unique);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Invite> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateInvite<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for CreateInvite<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::CreateInvite {
            channel_id: self.channel_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
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
        let mut builder = CreateInvite::new(&client, Id::new(1)).max_age(0)?;
        assert_eq!(Some(0), builder.fields.max_age);
        builder = builder.max_age(604_800)?;
        assert_eq!(Some(604_800), builder.fields.max_age);
        assert!(builder.max_age(604_801).is_err());

        Ok(())
    }

    #[test]
    fn max_uses() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let mut builder = CreateInvite::new(&client, Id::new(1)).max_uses(0)?;
        assert_eq!(Some(0), builder.fields.max_uses);
        builder = builder.max_uses(100)?;
        assert_eq!(Some(100), builder.fields.max_uses);
        assert!(builder.max_uses(101).is_err());

        Ok(())
    }
}
