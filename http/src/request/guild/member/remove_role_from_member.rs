use crate::request::prelude::*;
use dawn_model::id::{GuildId, RoleId, UserId};

pub struct RemoveRoleFromMember<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    user_id: UserId,
}

impl<'a> RemoveRoleFromMember<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
        role_id: impl Into<RoleId>,
    ) -> Self {
        Self {
            fut: None,
            guild_id: guild_id.into(),
            http,
            role_id: role_id.into(),
            user_id: user_id.into(),
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::RemoveMemberRole {
                guild_id: self.guild_id.0,
                role_id: self.role_id.0,
                user_id: self.user_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(RemoveRoleFromMember<'_>, ());
