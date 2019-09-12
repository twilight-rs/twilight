use futures_core::Stream;
use dawn_model::{
    guild::Member,
    id::{GuildId, UserId},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use super::prelude::*;

enum IterState<'a> {
    Done,
    InFlight {
        fut: PendingBody<'a, Vec<Member>>,
    },
    Prepared {
        last_id: UserId,
    },
}

pub struct GetGuildMembersIter<'a> {
    guild_id: GuildId,
    http: &'a Client,
    limit: u64,
    state: IterState<'a>,
}

impl<'a> GetGuildMembersIter<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        after: Option<UserId>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            guild_id: guild_id.into(),
            http,
            limit: limit.unwrap_or(1000),
            state: IterState::Prepared {
                last_id: after.unwrap_or(UserId(0)),
            },
        }
    }

    fn start(&mut self, id: UserId) -> Result<()> {
        self.state = IterState::InFlight {
            fut: self.http.request(Request::from(Route::GetGuildMembers {
                after: Some(id.0),
                guild_id: self.guild_id.0,
                limit: Some(self.limit),
            }))?,
        };

        Ok(())
    }
}

impl Stream for GetGuildMembersIter<'_> {
    type Item = Result<Vec<Member>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = Pin::get_mut(self);

        match &mut this.state {
            IterState::Done => return Poll::Ready(None),
            IterState::InFlight { fut } => {
                match Pin::new(fut).poll(cx) {
                    Poll::Ready(Ok(ref v)) if v.is_empty() => {
                        this.state = IterState::Done;

                        Poll::Ready(None)
                    },
                    Poll::Ready(Ok(v)) => {
                        if (v.len() as u64) < this.limit {
                            this.state = IterState::Done;
                        } else if let Some(member) = v.last() {
                            this.state = IterState::Prepared {
                                last_id: member.user.id,
                            };
                        }

                        Poll::Ready(Some(Ok(v)))
                    },
                    Poll::Ready(Err(why)) => Poll::Ready(Some(Err(why))),
                    Poll::Pending => Poll::Pending,
                }
            },
            IterState::Prepared { last_id } => {
                let id = *last_id;

                match this.start(id) {
                    Ok(()) => Poll::Pending,
                    Err(why) => Poll::Ready(Some(Err(why))),
                }
            },
        }
    }
}
