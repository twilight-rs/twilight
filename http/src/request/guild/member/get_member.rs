use crate::{
    client::Client,
    error::{Error, ErrorType},
    request::{PendingOption, Request},
    routing::Route,
};
use hyper::StatusCode;
use serde::de::DeserializeSeed;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::{
    guild::member::{Member, MemberDeserializer},
    id::{GuildId, UserId},
};

#[cfg(not(feature = "simd-json"))]
use serde_json::Value;
#[cfg(feature = "simd-json")]
use simd_json::value::OwnedValue as Value;

/// Get a member of a guild, by id.
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

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetMember {
            guild_id: self.guild_id.0,
            user_id: self.user_id.0,
        });

        self.fut.replace(Box::pin(self.http.request_bytes(request)));

        Ok(())
    }
}

impl Future for GetMember<'_> {
    type Output = Result<Option<Member>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                let bytes = match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(bytes)) => bytes,
                    Poll::Ready(Err(Error {
                        kind: ErrorType::Response { status, .. },
                        ..
                    })) if status == StatusCode::NOT_FOUND => {
                        return Poll::Ready(Ok(None));
                    }
                    Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                    Poll::Pending => return Poll::Pending,
                };

                let value = crate::json::from_bytes::<Value>(&bytes).map_err(Error::json)?;

                let member_deserializer = MemberDeserializer::new(self.guild_id);
                let member = member_deserializer
                    .deserialize(value)
                    .map_err(Error::json)?;

                return Poll::Ready(Ok(Some(member)));
            }

            if let Err(why) = self.as_mut().start() {
                return Poll::Ready(Err(why));
            }
        }
    }
}
