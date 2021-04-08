use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::{ChannelId, GuildId, RoleId, UserId};

/// The error created when the member can not be updated as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UpdateGuildMemberError {
    /// The nickname is either empty or the length is more than 32 UTF-16 characters.
    NicknameInvalid { nickname: String },
}

impl Display for UpdateGuildMemberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NicknameInvalid { .. } => f.write_str("the nickname length is invalid"),
        }
    }
}

impl Error for UpdateGuildMemberError {}

#[derive(Default, Serialize)]
struct UpdateGuildMemberFields {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Option<ChannelId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<RoleId>>,
}

/// Update a guild member.
///
/// All fields are optional. Refer to [the discord docs] for more information.
///
/// # Errors
///
/// Returns [`UpdateGuildMemberError::NicknameInvalid`] if the nickname length is too short or too
/// long.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild-member
pub struct UpdateGuildMember<'a> {
    fields: UpdateGuildMemberFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
    reason: Option<String>,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: UpdateGuildMemberFields::default(),
            fut: None,
            guild_id,
            http,
            user_id,
            reason: None,
        }
    }

    /// Move the member to a different voice channel.
    pub fn channel_id(mut self, channel_id: impl Into<Option<ChannelId>>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

        self
    }

    /// If true, restrict the member's ability to hear sound from a voice channel.
    pub fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf.replace(deaf);

        self
    }

    /// If true, restrict the member's ability to speak in a voice channel.
    pub fn mute(mut self, mute: bool) -> Self {
        self.fields.mute.replace(mute);

        self
    }

    /// Set the nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateGuildMemberError::NicknameInvalid`] if the nickname length is too short or
    /// too long.
    pub fn nick(self, nick: impl Into<Option<String>>) -> Result<Self, UpdateGuildMemberError> {
        self._nick(nick.into())
    }

    fn _nick(mut self, nick: Option<String>) -> Result<Self, UpdateGuildMemberError> {
        if let Some(nick) = nick {
            if !validate::nickname(&nick) {
                return Err(UpdateGuildMemberError::NicknameInvalid { nickname: nick });
            }

            self.fields.nick.replace(Some(nick));
        } else {
            self.fields.nick = None;
        }

        Ok(self)
    }

    /// Set the new list of roles for a member.
    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields)?,
                headers,
                Route::UpdateMember {
                    guild_id: self.guild_id.0,
                    user_id: self.user_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields)?,
                Route::UpdateMember {
                    guild_id: self.guild_id.0,
                    user_id: self.user_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for UpdateGuildMember<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(UpdateGuildMember<'_>, ());
