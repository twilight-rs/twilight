use crate::{
    client::Client,
    request::{validate_inner, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::GuildId;

/// The error created when the member can not be updated as configured.
#[derive(Debug)]
pub struct UpdateCurrentMemberError {
    kind: UpdateCurrentMemberErrorType,
}

impl UpdateCurrentMemberError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &UpdateCurrentMemberErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        UpdateCurrentMemberErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for UpdateCurrentMemberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            UpdateCurrentMemberErrorType::NicknameInvalid => {
                f.write_str("the nickname length is invalid")
            }
        }
    }
}

impl Error for UpdateCurrentMemberError {}

/// Type of [`UpdateCurrentMemberError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateCurrentMemberErrorType {
    /// The nickname is either empty or the length is more than 32 UTF-16 characters.
    NicknameInvalid,
}

#[derive(Serialize)]
struct UpdateCurrentMemberFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<&'a str>,
}

/// Update the user's member in a guild.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentMember<'a> {
    fields: UpdateCurrentMemberFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateCurrentMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: UpdateCurrentMemberFields { nick: None },
            guild_id,
            http,
        }
    }

    /// Set the current user's nickname.
    ///
    /// Set to [`None`] to clear the nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an [`UpdateCurrentMemberErrorType::NicknameInvalid`] error type
    /// if the nickname length is too short or too long.
    pub fn nick(mut self, nick: Option<&'a str>) -> Result<Self, UpdateCurrentMemberError> {
        if let Some(nick) = nick {
            if !validate_inner::nickname(&nick) {
                return Err(UpdateCurrentMemberError {
                    kind: UpdateCurrentMemberErrorType::NicknameInvalid,
                });
            }
        }

        self.fields.nick = nick;

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let mut request = Request::builder(&Route::UpdateCurrentMember {
            guild_id: self.guild_id.get(),
        });

        request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        self.http.request(request.build())
    }
}
