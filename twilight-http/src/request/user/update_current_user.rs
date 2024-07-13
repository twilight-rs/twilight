use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::user::User;
use twilight_validate::request::{
    audit_reason as validate_audit_reason, username as validate_username, ValidationError,
};

#[derive(Serialize)]
struct UpdateCurrentUserFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<&'a str>,
}

/// Update the current user.
///
/// All parameters are optional. If the username is changed, it may cause the discriminator to be
/// randomized.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentUser<'a> {
    fields: Result<UpdateCurrentUserFields<'a>, ValidationError>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateCurrentUser<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            fields: Ok(UpdateCurrentUserFields {
                avatar: None,
                banner: None,
                username: None,
            }),
            http,
            reason: Ok(None),
        }
    }

    /// Set the user's avatar.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn avatar(mut self, avatar: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.avatar = Some(Nullable(avatar));
        }

        self
    }

    /// Set the user's banner.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn banner(mut self, banner: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.banner = Some(Nullable(banner));
        }

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
    /// [`Username`]: twilight_validate::request::ValidationErrorType::Username
    pub fn username(mut self, username: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_username(username)?;
            fields.username.replace(username);

            Ok(fields)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateCurrentUser<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateCurrentUser<'_> {
    type Output = Result<Response<User>, Error>;

    type IntoFuture = ResponseFuture<User>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateCurrentUser<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        let mut request = Request::builder(&Route::UpdateCurrentUser).json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_clear_attachment() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".into());

        {
            let expected = r"{}";
            let actual = UpdateCurrentUser::new(&client).try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":null}"#;
            let actual = UpdateCurrentUser::new(&client)
                .avatar(None)
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());

            let expected = r#"{"avatar":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI"}"#;
            let actual = UpdateCurrentUser::new(&client).avatar(Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI")).try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"username":"other side"}"#;
            let actual = UpdateCurrentUser::new(&client)
                .username("other side")
                .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"avatar":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI","username":"other side"}"#;
            let actual = UpdateCurrentUser::new(&client).avatar(Some("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI")).username("other side").try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }
        Ok(())
    }
}
