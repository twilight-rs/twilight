use crate::request::prelude::*;
use twilight_model::{
    guild::Member,
    id::{GuildId, UserId},
};

pub struct GetMember<'a> {
    fut: Option<Pending<'a, Option<Member>>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> GetMember<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetMember {
                guild_id: self.guild_id.0,
                user_id: self.user_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetMember<'_>, Option<Member>);
