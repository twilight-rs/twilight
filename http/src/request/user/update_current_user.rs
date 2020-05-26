use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::user::User;

#[derive(Clone, Debug)]
pub enum UpdateCurrentUserError {
    /// The length of the username is either fewer than 2 UTF-16 characters or
    /// more than 32 UTF-16 characters.
    UsernameInvalid,
}

impl Display for UpdateCurrentUserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UsernameInvalid => f.write_str("the username length is invalid"),
        }
    }
}

impl Error for UpdateCurrentUserError {}

#[derive(Default, Serialize)]
struct UpdateCurrentUserFields {
    avatar: Option<String>,
    username: Option<String>,
}

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

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    /// Set the username.
    ///
    /// The minimum length is 2 UTF-16 characters and the maximum is 32 UTF-16
    /// characters.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateCurrentUserError::UsernameInvalid`] if the username
    /// length is too short or too long.
    ///
    /// [`UpdateCurrentUserError::UsernameInvalid`]: enum.UpdateCurrentUserError.html#variant.UsernameInvalid
    pub fn username(self, username: impl Into<String>) -> Result<Self, UpdateCurrentUserError> {
        self._username(username.into())
    }

    fn _username(mut self, username: String) -> Result<Self, UpdateCurrentUserError> {
        if !validate::username(&username) {
            return Err(UpdateCurrentUserError::UsernameInvalid);
        }

        self.fields.username.replace(username);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateCurrentUser,
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUser<'_>, User);
