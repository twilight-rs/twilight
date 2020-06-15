use futures_channel::oneshot::{Canceled, Receiver};
use futures_util::future::FutureExt;
use std::{future::Future, pin::Pin, task::{Context, Poll}};
use twilight_model::gateway::{
    event::Event,
    payload::{MessageCreate, ReactionAdd},
};

/// The future returned from [`Standby::wait_for_event`].
///
/// [`Standby::wait_for_event`]: struct.Standby.html#method.wait_for_event
#[derive(Debug)]
pub struct WaitForEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The future returned from [`Standby::wait_for`].
///
/// [`Standby::wait_for`]: struct.Standby.html#method.wait_for
#[derive(Debug)]
pub struct WaitForGuildEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForGuildEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The future returned from [`Standby::wait_for_message`].
///
/// [`Standby::wait_for_message`]: struct.Standby.html#method.wait_for_message
#[derive(Debug)]
pub struct WaitForMessageFuture {
    pub(crate) rx: Receiver<MessageCreate>,
}

impl Future for WaitForMessageFuture {
    type Output = Result<MessageCreate, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}

/// The future returned from [`Standby::wait_for_reaction`].
///
/// [`Standby::wait_for_reaction`]: struct.Standby.html#method.wait_for_reaction
#[derive(Debug)]
pub struct WaitForReactionFuture {
    pub(crate) rx: Receiver<ReactionAdd>,
}

impl Future for WaitForReactionFuture {
    type Output = Result<ReactionAdd, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx)
    }
}
