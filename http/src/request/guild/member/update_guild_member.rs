use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AuditLogReason, AuditLogReasonError, NullableField, Request, TryIntoRequest},
    response::{marker::MemberBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    Id,
};
use twilight_validate::request::{nickname as validate_nickname, ValidationError};

#[derive(Serialize)]
struct UpdateGuildMemberFields<'a> {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<NullableField<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [Id<RoleMarker>]>,
}

/// Update a guild member.
///
/// All fields are optional. Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild-member
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildMember<'a> {
    fields: UpdateGuildMemberFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
    reason: Option<&'a str>,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
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
    pub const fn channel_id(mut self, channel_id: Option<Id<ChannelMarker>>) -> Self {
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
    /// Returns an error of type [`Nickname`] if the nickname length is too
    /// short or too long.
    ///
    /// [`Nickname`]: twilight_validate::request::ValidationErrorType::Nickname
    pub fn nick(mut self, nick: Option<&'a str>) -> Result<Self, ValidationError> {
        if let Some(nick) = nick {
            validate_nickname(nick)?;
        }

        self.fields.nick = Some(NullableField(nick));

        Ok(self)
    }

    /// Set the new list of roles for a member.
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        self.fields.roles = Some(roles);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<MemberBody> {
        let guild_id = self.guild_id;
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => {
                let mut future = http.request(request);
                future.set_guild_id(guild_id);

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

impl TryIntoRequest for UpdateGuildMember<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
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
}

#[cfg(test)]
mod tests {
    use super::{UpdateGuildMember, UpdateGuildMemberFields};
    use crate::{
        request::{NullableField, Request, TryIntoRequest},
        routing::Route,
        Client,
    };
    use std::error::Error;
    use twilight_model::id::{
        marker::{GuildMarker, UserMarker},
        Id,
    };

    fn guild_id() -> Id<GuildMarker> {
        Id::new(1).expect("non zero")
    }

    fn user_id() -> Id<UserMarker> {
        Id::new(1).expect("non zero")
    }

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, guild_id(), user_id())
            .deaf(true)
            .mute(true);
        let actual = builder.try_into_request()?;

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
        let actual = builder.try_into_request()?;

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
        let actual = builder.try_into_request()?;

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
