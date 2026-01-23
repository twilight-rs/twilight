//! Futures and streams returned by [`Standby`].
//!
//! [`Standby`]: super::Standby

use futures_core::Stream;
use std::{
    error::Error,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::{mpsc, oneshot};

/// Future canceled due to Standby being dropped.
#[derive(Debug)]
pub struct Canceled(());

impl fmt::Display for Canceled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("standby dropped")
    }
}

impl Error for Canceled {}

/// Future returned from `Standby::wait_for_X`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForFuture<T> {
    /// Receiver half of the oneshot channel.
    pub(crate) rx: oneshot::Receiver<T>,
}

impl<T> Future for WaitForFuture<T> {
    type Output = Result<T, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.rx).poll(cx).map_err(|_| Canceled(()))
    }
}

/// Future returned from `Standby::wait_for_X_stream`.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct WaitForStream<T> {
    /// Receiver half of the MPSC channel
    pub(crate) rx: mpsc::UnboundedReceiver<T>,
}

impl<T> Stream for WaitForStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::{WaitForFuture, WaitForStream};
    use futures_core::Stream;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, future::Future};

    assert_impl_all!(WaitForFuture<()>: Debug, Future, Send, Sync);
    assert_impl_all!(WaitForStream<()>: Debug, Stream, Send, Sync);
}
