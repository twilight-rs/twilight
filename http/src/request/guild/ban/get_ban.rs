use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{
    guild::Ban,
    id::{GuildId, UserId},
};

/// Get information about a ban of a guild.
///
/// Includes the user banned and the reason.
pub struct GetBan<'a> {
    fut: Option<PendingResponse<'a, Ban>>,
    guild_id: GuildId,
    http: &'a Client,
    user_id: UserId,
}

impl<'a> GetBan<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            user_id,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetBan {
            guild_id: self.guild_id.0,
            user_id: self.user_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetBan<'_>, Ban);
