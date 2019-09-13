use crate::{
    error::{Error, ResponseError, Result},
    ratelimiting::{Ratelimit, RatelimitHeaders, Ratelimiter},
    routing::Path,
};
use futures_channel::oneshot::{Canceled, Sender};
use futures_util::future::{self, FutureExt};
use log::warn;
use reqwest::{Response, Result as ReqwestResult, StatusCode};
use serde::de::DeserializeOwned;
use std::{
    convert::TryFrom,
    future::Future,
    marker::PhantomData,
    mem,
    pin::Pin,
    result::Result as StdResult,
    task::{Context, Poll},
};

enum PendingState<'a> {
    Chunking {
        fut: Pin<Box<dyn Future<Output = Result<String>>>>,
    },
    Done,
    Empty,
    RatelimitQueued {
        fut: Pin<Box<dyn Future<Output = StdResult<Sender<Option<RatelimitHeaders>>, Canceled>>>>,
        req: Pin<Box<dyn Future<Output = ReqwestResult<Response>>>>,
    },
    RatelimitRetrieval {
        fut: Pin<Box<dyn Future<Output = Ratelimit> + 'a>>,
        req: Pin<Box<dyn Future<Output = ReqwestResult<Response>>>>,
    },
    Request {
        fut: Pin<Box<dyn Future<Output = ReqwestResult<Response>>>>,
        tx: Sender<Option<RatelimitHeaders>>,
    },
}

pub struct Pending<'a> {
    inner: PendingText<'a>,
}

impl<'a> Pending<'a> {
    pub(crate) fn new(
        fut: Pin<Box<dyn Future<Output = ReqwestResult<Response>>>>,
        ratelimiter: &'a Ratelimiter,
        bucket: Path,
    ) -> Self {
        Self {
            inner: PendingText::new(fut, ratelimiter, bucket),
        }
    }
}

impl Future for Pending<'_> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.get_mut().inner).poll(cx) {
            // Would just map this result, but most static analyzers wouldn't
            // detect the generic type in the Pin.
            Poll::Ready(Ok(_)) => Poll::Ready(Ok(())),
            Poll::Ready(Err(why)) => Poll::Ready(Err(why)),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct PendingBody<'a, T: DeserializeOwned> {
    inner: PendingText<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> PendingBody<'a, T> {
    pub(crate) fn new(
        fut: Pin<Box<dyn Future<Output = ReqwestResult<Response>>>>,
        ratelimiter: &'a Ratelimiter,
        bucket: Path,
    ) -> Self {
        Self {
            inner: PendingText::new(fut, ratelimiter, bucket),
            phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Unpin> Future for PendingBody<'_, T> {
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.get_mut().inner).poll(cx) {
            Poll::Ready(Ok(text)) => {
                match serde_json::from_str(&text) {
                    Ok(v) => Poll::Ready(Ok(v)),
                    Err(why) => Poll::Ready(Err(From::from(why))),
                }
            },
            Poll::Ready(Err(why)) => Poll::Ready(Err(why)),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct PendingText<'a> {
    state: PendingState<'a>,
}

impl<'a> PendingText<'a> {
    pub(crate) fn new(
        fut: Pin<Box<dyn Future<Output = ReqwestResult<Response>>>>,
        ratelimiter: &'a Ratelimiter,
        bucket: Path,
    ) -> Self {
        Self {
            state: PendingState::RatelimitRetrieval {
                fut: ratelimiter.get(bucket).boxed(),
                req: fut,
            },
        }
    }
}

impl Future for PendingText<'_> {
    type Output = Result<String>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match mem::replace(&mut self.as_mut().state, PendingState::Empty) {
                PendingState::Chunking { mut fut } => {
                    match Pin::as_mut(&mut fut).poll(cx) {
                        Poll::Ready(text) => {
                            let text = text?;
                            self.as_mut().state = PendingState::Done;

                            return Poll::Ready(Ok(text));
                        },
                        Poll::Pending => {
                            self.as_mut().state = PendingState::Chunking { fut };

                            return Poll::Pending;
                        },
                    }
                },
                PendingState::Done => panic!("polled after completion"),
                PendingState::Empty => unreachable!("state empty"),
                PendingState::RatelimitQueued { mut fut, req } => {
                    match fut.as_mut().poll(cx) {
                        Poll::Ready(Ok(tx)) => {
                            self.as_mut().state = PendingState::Request {
                                fut: req,
                                tx,
                            };
                        },
                        Poll::Ready(Err(why)) => {
                            warn!("Request canceled: {:?}", why);

                            return Poll::Ready(Err(Error::RequestCanceled {
                                source: why,
                            }));
                        },
                        Poll::Pending => {
                            self.as_mut().state = PendingState::RatelimitQueued {
                                fut,
                                req,
                            };

                            return Poll::Pending;
                        },
                    }
                },
                PendingState::RatelimitRetrieval { mut fut, req } => {
                    match fut.as_mut().poll(cx) {
                        Poll::Ready(ratelimit) => {
                            self.as_mut().state = match ratelimit {
                                Ratelimit::Queued(rx) => PendingState::RatelimitQueued {
                                    fut: rx.boxed(),
                                    req,
                                },
                                Ratelimit::Ready(tx) => PendingState::Request {
                                    fut: req,
                                    tx,
                                },
                            };
                        },
                        Poll::Pending => return Poll::Pending,
                    }
                },
                PendingState::Request { mut fut, tx } => {
                    match fut.as_mut().poll(cx) {
                        Poll::Ready(resp) => {
                            let resp = match resp {
                                Ok(v) => v,
                                Err(why) => panic!("{:?}", why),
                            };

                            match RatelimitHeaders::try_from(resp.headers()) {
                                Ok(v) => {
                                    let _ = tx.send(Some(v));
                                },
                                Err(why) => {
                                    warn!(
                                        "Err parsing headers: {:?}; {:?}",
                                        why,
                                        resp,
                                    );

                                    let _ = tx.send(None);
                                }
                            }

                            if resp.status().is_client_error() {
                                if resp.status() == StatusCode::IM_A_TEAPOT {
                                    warn!(
                                        "Discord's API now runs off of teapots -- proceed to panic: {:?}",
                                        resp,
                                    );
                                }

                                if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                                    warn!("Response got 429: {:?}", resp);
                                }

                                return Poll::Ready(Err(Error::Response {
                                    source: ResponseError::Client {
                                        response: resp,
                                    },
                                }));
                            }

                            if resp.status().is_client_error() {
                                return Poll::Ready(Err(Error::Response {
                                    source: ResponseError::Server {
                                        response: resp,
                                    },
                                }));
                            }

                            let fut = resp.text().then(|res| future::ready(match res {
                                Ok(v) => Ok(v),
                                Err(why) => Err(Error::ChunkingResponse {
                                    source: why,
                                }),
                            }));
                            self.as_mut().state = PendingState::Chunking {
                                fut: fut.boxed(),
                            };
                        },
                        Poll::Pending => {
                            self.as_mut().state = PendingState::Request { fut, tx };

                            return Poll::Pending;
                        },
                    }
                },
            }
        }
    }
}
