use crate::{
    client::Client,
    request::Request,
    response::{marker::MemberBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::GuildId;

/// Get the current user in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserGuildMember<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetCurrentUserGuildMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            http,
            guild_id
        }
    }

    pub fn exec(self) -> ResponseFuture<MemberBody> {
        let request = Request::from_route(&Route::GetCurrentUserGuildMember {
            guild_id: self.guild_id.get(),
        });

        let mut future = self.http.request(request);
        future.set_guild_id(self.guild_id);

        future
    }
}
