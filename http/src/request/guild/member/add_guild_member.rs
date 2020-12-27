use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::PartialMember,
    id::{GuildId, RoleId, UserId},
};

/// The error created when the member cannot be added as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AddGuildMemberError {
    // The nickname is either empty or the length is more than 32 UTF-16 characters.
    NickNameInvalid { nickname: String },
}

impl Display for AddGuildMemberError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NickNameInvalid { .. } => f.write_str("the nickname length is invalid"),
        }
    }
}

impl Error for AddGuildMemberError {}

#[derive(Serialize)]
struct AddGuildMemberFields {
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,
}

pub struct AddGuildMember<'a> {
    fields: AddGuildMemberFields,
    fut: Option<PendingOption<'a>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

/// Adds a user to a guild.
///
/// An access token for the user with `guilds.join` scope is required. All other fields are
/// optional. Refer to [the discord docs] for more information.
///
/// # Errors
///
/// Returns [`AddGuildMemberError::NicknameInvalid`] if the nickname is too short or too long.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#add-guild-member
impl<'a> AddGuildMember<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        user_id: UserId,
        access_token: impl Into<String>,
    ) -> Self {
        Self::_new(http, guild_id, user_id, access_token.into())
    }

    fn _new(http: &'a Client, guild_id: GuildId, user_id: UserId, access_token: String) -> Self {
        Self {
            fields: AddGuildMemberFields {
                access_token,
                deaf: None,
                mute: None,
                nick: None,
                roles: None,
            },
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    /// If true, the new member will be unable to hear audio when connected to a voice channel.
    pub fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf.replace(deaf);

        self
    }

    /// If true, the new member will be unable to speak in voice channels.
    pub fn mute(mut self, mute: bool) -> Self {
        self.fields.mute.replace(mute);

        self
    }

    /// Set the user's initial nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`AddGuildMemberError::NicknameInvalid`] if the nickname is too short or too long.
    pub fn nick(self, nick: impl Into<String>) -> Result<Self, AddGuildMemberError> {
        self._nick(nick.into())
    }

    fn _nick(mut self, nick: String) -> Result<Self, AddGuildMemberError> {
        if !validate::nickname(&nick) {
            return Err(AddGuildMemberError::NickNameInvalid {
                nickname: nick.to_owned(),
            });
        }

        self.fields.nick.replace(nick);

        Ok(self)
    }

    /// The list of roles to assign the new member.
    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::AddGuildMember {
                guild_id: self.guild_id.0,
                user_id: self.user_id.0,
            },
        ));

        self.fut.replace(Box::pin(self.http.request_bytes(request)));

        Ok(())
    }
}

poll_req!(opt, AddGuildMember<'_>, PartialMember);
