use crate::{
    client::Client,
    error::{Error, ErrorType},
    request::{self, AuditLogReason, Form, Request, TryIntoRequest, invite::target_user},
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

/// Maximum number of target user IDs allowed in a JSON request body.
///
/// If this number is exceeded then the request must use a multipart body.
pub const TARGET_USER_IDS_JSON_MAX: usize = 1000;

#[derive(Serialize)]
struct CreateChannelInviteFields {
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
    /// An array of IDs of all users able to see and accept this invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    target_user_ids: Option<Vec<Id<UserMarker>>>,
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
/// let response = client.create_channel_invite(channel_id).max_uses(3).await?;
/// let invite = response.model().await?;
/// # Ok(()) }
/// ```
///
/// [`CREATE_INVITE`]: twilight_model::guild::Permissions::CREATE_INVITE
#[must_use = "requests must be configured and executed"]
pub struct CreateChannelInvite<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<CreateChannelInviteFields, ValidationError>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateChannelInvite<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: Ok(CreateChannelInviteFields {
                max_age: None,
                max_uses: None,
                temporary: None,
                target_application_id: None,
                target_user_id: None,
                target_type: None,
                target_user_ids: None,
                unique: None,
            }),
            http,
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
    ///     .create_channel_invite(Id::new(1))
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
        self.fields = self.fields.and_then(|mut fields| {
            validate_invite_max_age(max_age)?;
            fields.max_age = Some(max_age);

            Ok(fields)
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
    ///     .create_channel_invite(Id::new(1))
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
        self.fields = self.fields.and_then(|mut fields| {
            validate_invite_max_uses(max_uses)?;
            fields.max_uses = Some(max_uses);

            Ok(fields)
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
        if let Ok(fields) = self.fields.as_mut() {
            fields.target_application_id = Some(target_application_id);
        }

        self
    }

    /// Set the target user ID for this invite.
    ///
    /// This sets the ID of the user whose stream to display for the invite.
    /// This is unrelated to [`target_user_ids`] which sets the users to add
    /// the invite's target users.
    ///
    /// [`target_user_ids`]: Self::target_user_ids
    pub const fn target_user_id(mut self, target_user_id: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.target_user_id = Some(target_user_id);
        }

        self
    }

    /// Set the target type for this invite.
    pub const fn target_type(mut self, target_type: TargetType) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.target_type = Some(target_type);
        }

        self
    }

    /// Set the IDs of the target users for this invite.
    ///
    /// This operates with the bulk user invite API, similar to calling the
    /// [`BulkAddTargetUsers`] request on an existing invite. This is unrelated
    /// to [`target_user_id`] which sets the ID of the user whose stream to
    /// display for the invite.
    ///
    /// # Implementation details
    ///
    /// The Create Channel Invite endpoint supports two mutually exclusive
    /// fields: `target_user_ids` and `target_users_file`. `target_user_ids`
    /// supports up to 1,000 users when sent as a request with a JSON body while
    /// `target_users_file` supports more users when sent as a multipart
    /// request. If the provided number of target users is above the threshold
    /// allowed by the JSON body then the request body will automatically use a
    /// multipart request body.
    ///
    /// [`BulkAddTargetUsers`]: crate::request::invite::target_user::BulkAddTargetUsers
    /// [`target_user_id`]: Self::target_user_id
    pub fn target_user_ids(mut self, target_user_ids: Vec<Id<UserMarker>>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.target_user_ids = Some(target_user_ids);
        }

        self
    }

    /// Specify true if the invite should grant temporary membership.
    ///
    /// Defaults to false.
    pub const fn temporary(mut self, temporary: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.temporary = Some(temporary);
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
        if let Ok(fields) = self.fields.as_mut() {
            fields.unique = Some(unique);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateChannelInvite<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateChannelInvite<'_> {
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

impl TryIntoRequest for CreateChannelInvite<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::CreateInvite {
            channel_id: self.channel_id.get(),
        });

        if fields
            .target_user_ids
            .as_ref()
            .is_some_and(|target_user_ids| target_user_ids.len() > TARGET_USER_IDS_JSON_MAX)
        {
            // Take the user IDs out because they need to be in a separate field
            // from the payload JSON.
            let target_user_ids = fields.target_user_ids.take().expect("must be present");
            let payload_json = crate::json::to_vec(&fields).map_err(Error::json)?;

            let target_users_file =
                target_user::file(&target_user_ids).map_err(|source| Error {
                    kind: ErrorType::WritingCsv,
                    source: Some(Box::new(source)),
                })?;

            let form = Form::new()
                .json_part(b"payload_json", &payload_json)
                .file_part(
                    b"target_users_file",
                    b"file.csv",
                    target_users_file.as_slice(),
                );

            request = request.form(form);
        } else {
            request = request.json(&fields);
        }

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::CreateChannelInvite;
    use crate::Client;
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn max_age() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let mut builder = CreateChannelInvite::new(&client, Id::new(1)).max_age(0);
        assert_eq!(Some(0), builder.fields.as_ref().unwrap().max_age);
        builder = builder.max_age(604_800);
        assert_eq!(Some(604_800), builder.fields.as_ref().unwrap().max_age);
        builder = builder.max_age(604_801);
        assert!(builder.fields.is_err());

        Ok(())
    }

    #[test]
    fn max_uses() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let mut builder = CreateChannelInvite::new(&client, Id::new(1)).max_uses(0);
        assert_eq!(Some(0), builder.fields.as_ref().unwrap().max_uses);
        builder = builder.max_uses(100);
        assert_eq!(Some(100), builder.fields.as_ref().unwrap().max_uses);
        builder = builder.max_uses(101);
        assert!(builder.fields.is_err());

        Ok(())
    }
}
