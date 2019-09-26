use super::prelude::*;
use dawn_model::id::{GuildId, UserId};

pub struct CreateBan<'a> {
    delete_message_days: Option<u64>,
    guild_id: GuildId,
    reason: Option<String>,
    user_id: UserId,
    fut: Option<Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>>,
    http: &'a Client,
}

impl<'a> CreateBan<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            user_id: user_id.into(),
            delete_message_days: None,
            reason: None,
            fut: None,
            http,
        }
    }

    pub fn delete_message_days(mut self, days: u64) -> Self {
        self.delete_message_days.replace(days);

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::CreateBan {
                delete_message_days: self.delete_message_days,
                guild_id: self.guild_id.0,
                reason: self.reason.as_ref().map(ToOwned::to_owned),
                user_id: self.user_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(CreateBan<'_>, ());
