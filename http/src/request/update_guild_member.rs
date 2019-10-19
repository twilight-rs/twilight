use super::prelude::*;
use dawn_model::{
    guild::Member,
    id::{ChannelId, GuildId, RoleId, UserId},
};

#[derive(Default, Serialize)]
struct UpdateGuildMemberFields {
    channel_id: Option<ChannelId>,
    deaf: Option<bool>,
    mute: Option<bool>,
    nick: Option<String>,
    roles: Option<Vec<RoleId>>,
}

pub struct UpdateGuildMember<'a> {
    fields: UpdateGuildMemberFields,
    fut: Option<Pending<'a, Member>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> UpdateGuildMember<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: UpdateGuildMemberFields::default(),
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

        self
    }

    pub fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf.replace(deaf);

        self
    }

    pub fn mute(mut self, mute: bool) -> Self {
        self.fields.mute.replace(mute);

        self
    }

    pub fn nick(mut self, nick: impl Into<String>) -> Self {
        self.fields.nick.replace(nick.into());

        self
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateMember {
                guild_id: self.guild_id.0,
                user_id: self.user_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateGuildMember<'_>, Member);
