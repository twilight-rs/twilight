use dawn_model::{
    guild::Member,
    id::{ChannelId, GuildId, RoleId, UserId},
};
use serde::Serialize;
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateGuildMember<'a> {
    channel_id: Option<ChannelId>,
    deaf: Option<bool>,
    mute: Option<bool>,
    nick: Option<String>,
    roles: Option<Vec<RoleId>>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Member>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    user_id: UserId,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> Self {
        Self {
            channel_id: None,
            deaf: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
            mute: None,
            nick: None,
            roles: None,
            user_id: user_id.into(),
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.channel_id.replace(channel_id.into());

        self
    }

    pub fn deaf(mut self, deaf: bool) -> Self {
        self.deaf.replace(deaf);

        self
    }

    pub fn mute(mut self, mute: bool) -> Self {
        self.mute.replace(mute);

        self
    }

    pub fn nick(mut self, nick: impl Into<String>) -> Self {
        self.nick.replace(nick.into());

        self
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            body: Some(serde_json::to_vec(self)?),
            route: Route::UpdateMember {
                guild_id: self.guild_id.0,
                user_id: self.user_id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(UpdateGuildMember<'_>, Member);
