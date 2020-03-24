use crate::request::prelude::*;
use twilight_model::id::{GuildId, UserId};

#[derive(Default)]
struct CreateBanFields {
    delete_message_days: Option<u64>,
    reason: Option<String>,
}

pub struct CreateBan<'a> {
    fields: CreateBanFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> CreateBan<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fields: CreateBanFields::default(),
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    pub fn delete_message_days(mut self, days: u64) -> Self {
        self.fields.delete_message_days.replace(days);

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.fields.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::CreateBan {
                delete_message_days: self.fields.delete_message_days,
                guild_id: self.guild_id.0,
                reason: self.fields.reason.clone(),
                user_id: self.user_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(CreateBan<'_>, ());
