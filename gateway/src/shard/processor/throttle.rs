use futures_util::{future::FutureExt, ready, stream::Stream};
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::{sleep, Instant, Sleep};

#[derive(Debug)]
pub struct Throttle {
    delay: Pin<Box<Sleep>>,
    duration: Duration,
}

impl Throttle {
    pub fn new(duration: Duration) -> Self {
        let fut = sleep(duration);
        Self {
            delay: Box::pin(fut),
            duration,
        }
    }
}

impl Stream for Throttle {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        ready!(self.delay.poll_unpin(cx));
        let duration = self.duration;
        self.delay.as_mut().reset(Instant::now() + duration);

        Poll::Ready(Some(()))
    }
}
