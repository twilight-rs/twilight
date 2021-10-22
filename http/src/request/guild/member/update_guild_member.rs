use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, validate_inner, AuditLogReason, AuditLogReasonError, NullableField, Request},
    response::{marker::MemberBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::{ChannelId, GuildId, RoleId, UserId};

/// The error created when the member can not be updated as configured.
#[derive(Debug)]
pub struct UpdateGuildMemberError {
    kind: UpdateGuildMemberErrorType,
}

impl UpdateGuildMemberError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &UpdateGuildMemberErrorType {
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
        UpdateGuildMemberErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for UpdateGuildMemberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            UpdateGuildMemberErrorType::NicknameInvalid => {
                f.write_str("the nickname length is invalid")
            }
        }
    }
}

impl Error for UpdateGuildMemberError {}

/// Type of [`UpdateGuildMemberError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateGuildMemberErrorType {
    /// The nickname is either empty or the length is more than 32 UTF-16 characters.
    NicknameInvalid,
}

#[derive(Serialize)]
struct UpdateGuildMemberFields<'a> {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<NullableField<ChannelId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [RoleId]>,
}

/// Update a guild member.
///
/// All fields are optional. Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild-member
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildMember<'a> {
    fields: UpdateGuildMemberFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
    reason: Option<&'a str>,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: UpdateGuildMemberFields {
                channel_id: None,
                deaf: None,
                mute: None,
                nick: None,
                roles: None,
            },
            guild_id,
            http,
            user_id,
            reason: None,
        }
    }

    /// Move the member to a different voice channel.
    pub const fn channel_id(mut self, channel_id: Option<ChannelId>) -> Self {
        self.fields.channel_id = Some(NullableField(channel_id));

        self
    }

    /// If true, restrict the member's ability to hear sound from a voice channel.
    pub const fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf = Some(deaf);

        self
    }

    /// If true, restrict the member's ability to speak in a voice channel.
    pub const fn mute(mut self, mute: bool) -> Self {
        self.fields.mute = Some(mute);

        self
    }

    /// Set the nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an [`UpdateGuildMemberErrorType::NicknameInvalid`] error type if
    /// the nickname length is too short or too long.
    pub fn nick(mut self, nick: Option<&'a str>) -> Result<Self, UpdateGuildMemberError> {
        if let Some(nick) = nick {
            if !validate_inner::nickname(&nick) {
                return Err(UpdateGuildMemberError {
                    kind: UpdateGuildMemberErrorType::NicknameInvalid,
                });
            }
        }

        self.fields.nick = Some(NullableField(nick));

        Ok(self)
    }

    /// Set the new list of roles for a member.
    pub const fn roles(mut self, roles: &'a [RoleId]) -> Self {
        self.fields.roles = Some(roles);

        self
    }

    fn request(&self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateMember {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        })
        .json(&self.fields)?;

        if let Some(reason) = &self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        Ok(request.build())
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<MemberBody> {
        match self.request() {
            Ok(request) => {
                let mut future = self.http.request(request);
                future.set_guild_id(self.guild_id);

                future
            }
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildMember<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{UpdateGuildMember, UpdateGuildMemberFields};
    use crate::{
        request::{NullableField, Request},
        routing::Route,
        Client,
    };
    use std::error::Error;
    use twilight_model::id::{GuildId, UserId};

    fn guild_id() -> GuildId {
        GuildId::new(1).expect("non zero")
    }

    fn user_id() -> UserId {
        UserId::new(1).expect("non zero")
    }

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, guild_id(), user_id())
            .deaf(true)
            .mute(true);
        let actual = builder.request()?;

        let body = UpdateGuildMemberFields {
            channel_id: None,
            deaf: Some(true),
            mute: Some(true),
            nick: None,
            roles: None,
        };
        let route = Route::UpdateMember {
            guild_id: guild_id().get(),
            user_id: user_id().get(),
        };
        let expected = Request::builder(&route).json(&body)?.build();

        assert_eq!(actual.body, expected.body);
        assert_eq!(actual.path, expected.path);

        Ok(())
    }

    #[test]
    fn test_nick_set_null() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, guild_id(), user_id()).nick(None)?;
        let actual = builder.request()?;

        let body = UpdateGuildMemberFields {
            channel_id: None,
            deaf: None,
            mute: None,
            nick: Some(NullableField(None)),
            roles: None,
        };
        let route = Route::UpdateMember {
            guild_id: guild_id().get(),
            user_id: user_id().get(),
        };
        let expected = Request::builder(&route).json(&body)?.build();

        assert_eq!(actual.body, expected.body);

        Ok(())
    }

    #[test]
    fn test_nick_set_value() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, guild_id(), user_id()).nick(Some("foo"))?;
        let actual = builder.request()?;

        let body = UpdateGuildMemberFields {
            channel_id: None,
            deaf: None,
            mute: None,
            nick: Some(NullableField(Some("foo"))),
            roles: None,
        };
        let route = Route::UpdateMember {
            guild_id: guild_id().get(),
            user_id: user_id().get(),
        };
        let expected = Request::builder(&route).json(&body)?.build();

        assert_eq!(actual.body, expected.body);

        Ok(())
    }
}
