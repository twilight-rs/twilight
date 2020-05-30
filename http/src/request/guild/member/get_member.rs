use crate::request::prelude::*;
use bytes::Bytes;
use serde::de::{value::BorrowedBytesDeserializer, DeserializeSeed};
use serde_json::Error as JsonError;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::{
    guild::member::{MemberDeserializer, Member},
    id::{GuildId, UserId},
};

pub struct GetMember<'a> {
    fut: Option<Pending<'a, Bytes>>,
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
        self.fut.replace(Box::pin(self.http.request_bytes(Request::from(
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

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        if self.fut.is_none() {
            self.as_mut().start()?;
        }

        let fut = self.fut.as_mut().expect("future is created");

        match fut.as_mut().poll(cx) {
            Poll::Ready(res) => {
                let bytes = res?;

                let member_deserializer = MemberDeserializer::new(self.guild_id);
                let deserializer: BorrowedBytesDeserializer<'_, JsonError> = BorrowedBytesDeserializer::new(&bytes);
                let member = member_deserializer.deserialize(deserializer)?;

                Poll::Ready(Ok(Some(member)))
            },
            Poll::Pending => Poll::Pending,
        }
    }
}
