use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, NullableField, Request},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::user::User;
use twilight_validate::misc::{username as validate_username, ValidationError};

#[derive(Serialize)]
struct UpdateCurrentUserFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<&'a str>,
}

/// Update the current user.
///
/// All parameters are optional. If the username is changed, it may cause the discriminator to be
/// randomized.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentUser<'a> {
    fields: UpdateCurrentUserFields<'a>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> UpdateCurrentUser<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            fields: UpdateCurrentUserFields {
                avatar: None,
                username: None,
            },
            http,
            reason: None,
        }
    }

    /// Set the user's avatar.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub const fn avatar(mut self, avatar: Option<&'a str>) -> Self {
        self.fields.avatar = Some(NullableField(avatar));

        self
    }

    /// Set the username.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Username`] if the username length is too
    /// short or too long.
    ///
    /// [`Username`]: twilight_validate::misc::ValidationErrorType::Username
    pub fn username(mut self, username: &'a str) -> Result<Self, ValidationError> {
        validate_username(username)?;

        self.fields.username.replace(username);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<User> {
        let mut request = Request::builder(&Route::UpdateCurrentUser);

        request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        if let Some(reason) = &self.reason {
            let header = match request::audit_header(reason) {
                Ok(header) => header,
                Err(source) => return ResponseFuture::error(source),
            };

            request = request.headers(header);
        };

        self.http.request(request.build())
    }
}

impl<'a> AuditLogReason<'a> for UpdateCurrentUser<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}
