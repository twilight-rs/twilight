use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::user::User;

/// The error created when the user can not be updated as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UpdateCurrentUserError {
    /// The length of the username is either fewer than 2 UTF-16 characters or more than 32 UTF-16
    /// characters.
    UsernameInvalid {
        /// Provided username.
        username: String,
    },
}

impl Display for UpdateCurrentUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UsernameInvalid { .. } => f.write_str("the username length is invalid"),
        }
    }
}

impl Error for UpdateCurrentUserError {}

#[derive(Default, Serialize)]
struct UpdateCurrentUserFields {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
}

/// Update the current user.
///
/// All paramaters are optional. If the username is changed, it may cause the discriminator to be
/// rnadomized.
pub struct UpdateCurrentUser<'a> {
    fields: UpdateCurrentUserFields,
    fut: Option<Pending<'a, User>>,
    http: &'a Client,
}

impl<'a> UpdateCurrentUser<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fields: UpdateCurrentUserFields::default(),
            fut: None,
            http,
        }
    }

    /// Set the user's avatar.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub fn avatar(mut self, avatar: impl Into<Option<String>>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    /// Set the username.
    ///
    /// The minimum length is 2 UTF-16 characters and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateCurrentUserError::UsernameInvalid`] if the username length is too short or
    /// too long.
    pub fn username(self, username: impl Into<String>) -> Result<Self, UpdateCurrentUserError> {
        self._username(username.into())
    }

    fn _username(mut self, username: String) -> Result<Self, UpdateCurrentUserError> {
        if !validate::username(&username) {
            return Err(UpdateCurrentUserError::UsernameInvalid { username });
        }

        self.fields.username.replace(username);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::UpdateCurrentUser,
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUser<'_>, User);
