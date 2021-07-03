use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::{marker::MemberBody, Response},
    routing::Route,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::id::{GuildId, UserId};

/// Get a member of a guild, by id.
pub struct GetMember<'a> {
    fut: Option<PendingResponse<'a, MemberBody>>,
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

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetMember {
            guild_id: self.guild_id.0,
            user_id: self.user_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

impl Future for GetMember<'_> {
    type Output = Result<Response<MemberBody>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                return fut.as_mut().poll(cx).map_ok(|mut res| {
                    res.set_guild_id(self.guild_id);

                    res
                });
            }

            if let Err(why) = self.as_mut().start() {
                return Poll::Ready(Err(why));
            }
        }
    }
}
