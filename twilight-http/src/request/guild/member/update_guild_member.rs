use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::Member,
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::Timestamp,
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason,
    communication_disabled_until as validate_communication_disabled_until,
    nickname as validate_nickname, ValidationError,
};

#[derive(Serialize)]
struct UpdateGuildMemberFields<'a> {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    communication_disabled_until: Option<Nullable<Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [Id<RoleMarker>]>,
}

/// Update a guild member.
///
/// All fields are optional. See [Discord Docs/Modify Guild Member].
///
/// [Discord Docs/Modify Guild Member]: https://discord.com/developers/docs/resources/guild#modify-guild-member
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildMember<'a> {
    fields: Result<UpdateGuildMemberFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            fields: Ok(UpdateGuildMemberFields {
                channel_id: None,
                communication_disabled_until: None,
                deaf: None,
                mute: None,
                nick: None,
                roles: None,
            }),
            guild_id,
            http,
            user_id,
            reason: Ok(None),
        }
    }

    /// Move the member to a different voice channel.
    pub fn channel_id(mut self, channel_id: Option<Id<ChannelMarker>>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.channel_id = Some(Nullable(channel_id));
        }

        self
    }

    /// Set the member's [Guild Timeout].
    ///
    /// The timestamp indicates when the user will be able to communicate again.
    /// It can be up to 28 days in the future. Set to [`None`] to remove the
    /// timeout. Requires the [`MODERATE_MEMBERS`] permission. If this is set,
    /// and if the target member is an administrator or the owner of the guild,
    /// the response status code will be 403.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`CommunicationDisabledUntil`] if the expiry
    /// timestamp is more than 28 days from the current time.
    ///
    /// [Guild Timeout]: https://support.discord.com/hc/en-us/articles/4413305239191-Time-Out-FAQ
    /// [`CommunicationDisabledUntil`]: twilight_validate::request::ValidationErrorType::CommunicationDisabledUntil
    /// [`MODERATE_MEMBERS`]: twilight_model::guild::Permissions::MODERATE_MEMBERS
    pub fn communication_disabled_until(mut self, timestamp: Option<Timestamp>) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(timestamp) = timestamp {
                validate_communication_disabled_until(timestamp)?;
            }

            fields.communication_disabled_until = Some(Nullable(timestamp));

            Ok(fields)
        });

        self
    }

    /// If true, restrict the member's ability to hear sound from a voice channel.
    pub fn deaf(mut self, deaf: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.deaf = Some(deaf);
        }

        self
    }

    /// If true, restrict the member's ability to speak in a voice channel.
    pub fn mute(mut self, mute: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.mute = Some(mute);
        }

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
    pub fn nick(mut self, nick: Option<&'a str>) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(nick) = nick {
                validate_nickname(nick)?;
            }

            fields.nick = Some(Nullable(nick));

            Ok(fields)
        });

        self
    }

    /// Set the new list of roles for a member.
    pub fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.roles = Some(roles);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildMember<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuildMember<'_> {
    type Output = Result<Response<Member>, Error>;

    type IntoFuture = ResponseFuture<Member>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateMember {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{UpdateGuildMember, UpdateGuildMemberFields};
    use crate::{
        request::{Nullable, Request, TryIntoRequest},
        routing::Route,
        Client,
    };
    use std::error::Error;
    use twilight_model::id::{
        marker::{GuildMarker, UserMarker},
        Id,
    };

    const GUILD_ID: Id<GuildMarker> = Id::new(1);
    const USER_ID: Id<UserMarker> = Id::new(1);

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, GUILD_ID, USER_ID)
            .deaf(true)
            .mute(true);
        let actual = builder.try_into_request()?;

        let body = UpdateGuildMemberFields {
            channel_id: None,
            communication_disabled_until: None,
            deaf: Some(true),
            mute: Some(true),
            nick: None,
            roles: None,
        };
        let route = Route::UpdateMember {
            guild_id: GUILD_ID.get(),
            user_id: USER_ID.get(),
        };
        let expected = Request::builder(&route).json(&body).build()?;

        assert_eq!(actual.body, expected.body);
        assert_eq!(actual.path, expected.path);

        Ok(())
    }

    #[test]
    fn nick_set_null() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, GUILD_ID, USER_ID).nick(None);
        let actual = builder.try_into_request()?;

        let body = UpdateGuildMemberFields {
            channel_id: None,
            communication_disabled_until: None,
            deaf: None,
            mute: None,
            nick: Some(Nullable(None)),
            roles: None,
        };
        let route = Route::UpdateMember {
            guild_id: GUILD_ID.get(),
            user_id: USER_ID.get(),
        };
        let expected = Request::builder(&route).json(&body).build()?;

        assert_eq!(actual.body, expected.body);

        Ok(())
    }

    #[test]
    fn nick_set_value() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());
        let builder = UpdateGuildMember::new(&client, GUILD_ID, USER_ID).nick(Some("foo"));
        let actual = builder.try_into_request()?;

        let body = UpdateGuildMemberFields {
            channel_id: None,
            communication_disabled_until: None,
            deaf: None,
            mute: None,
            nick: Some(Nullable(Some("foo"))),
            roles: None,
        };
        let route = Route::UpdateMember {
            guild_id: GUILD_ID.get(),
            user_id: USER_ID.get(),
        };
        let expected = Request::builder(&route).json(&body).build()?;

        assert_eq!(actual.body, expected.body);

        Ok(())
    }
}
