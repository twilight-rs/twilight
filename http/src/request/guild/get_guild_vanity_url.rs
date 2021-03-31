use crate::{request::prelude::*, Error};
use hyper::StatusCode;
use serde::Deserialize;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::id::GuildId;

#[derive(Deserialize)]
struct VanityUrl {
    code: String,
}

/// Get a guild's vanity url, if there is one.
pub struct GetGuildVanityUrl<'a> {
    fut: Option<PendingOption<'a>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildVanityUrl<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let fut = self
            .http
            .request_bytes(Request::from(Route::GetGuildVanityUrl {
                guild_id: self.guild_id.0,
            }));
        self.fut.replace(Box::pin(fut));

        Ok(())
    }
}

impl Future for GetGuildVanityUrl<'_> {
    type Output = Result<Option<String>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                let bytes = match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(bytes)) => bytes,
                    Poll::Ready(Err(crate::Error::Response { status, .. }))
                        if status == StatusCode::NOT_FOUND =>
                    {
                        return Poll::Ready(Ok(None));
                    }
                    Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                    Poll::Pending => return Poll::Pending,
                };

                let mut bytes = bytes.as_ref().to_vec();
                let vanity_url =
                    crate::json_from_slice::<VanityUrl>(&mut bytes).map_err(|source| {
                        Error::Parsing {
                            body: bytes.clone(),
                            source,
                        }
                    })?;

                return Poll::Ready(Ok(Some(vanity_url.code)));
            }

            if let Err(why) = self.as_mut().start() {
                return Poll::Ready(Err(why));
            }
        }
    }
}
