use crate::request::prelude::*;
use serde::de::{value::BorrowedBytesDeserializer, DeserializeSeed};
use serde_json::Error as JsonError;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::{
    guild::member::{Member, MemberDeserializer},
    id::{GuildId, UserId},
};

pub struct GetMember<'a> {
    fut: Option<PendingOption<'a>>,
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
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetMember {
                    guild_id: self.guild_id.0,
                    user_id: self.user_id.0,
                },
            ))));

        Ok(())
    }
}

impl Future for GetMember<'_> {
    type Output = Result<Option<Member>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                let bytes = match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(bytes)) => bytes,
                    Poll::Ready(Err(crate::Error::Response { status, .. }))
                        if status == reqwest::StatusCode::NOT_FOUND =>
                    {
                        return Poll::Ready(Ok(None));
                    }
                    Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                    Poll::Pending => return Poll::Pending,
                };

                let member_deserializer = MemberDeserializer::new(self.guild_id);
                let deserializer: BorrowedBytesDeserializer<'_, JsonError> =
                    BorrowedBytesDeserializer::new(&bytes);
                let member = member_deserializer.deserialize(deserializer)?;

                return Poll::Ready(Ok(Some(member)));
            }

            if let Err(why) = self.as_mut().start() {
                return Poll::Ready(Err(why));
            }
        }
    }
}
