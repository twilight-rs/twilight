use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::Member,
    id::{GuildId, RoleId, UserId},
};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AddGuildMemberError {
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

    pub fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf.replace(deaf);

        self
    }

    pub fn mute(mut self, mute: bool) -> Self {
        self.fields.mute.replace(mute);

        self
    }

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

poll_req!(opt, AddGuildMember<'_>, Member);
