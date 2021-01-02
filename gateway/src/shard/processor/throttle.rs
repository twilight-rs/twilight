use futures_timer::Delay;
use futures_util::{future::FutureExt, ready, stream::Stream};
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub struct Throttle {
    delay: Delay,
    duration: Duration,
}

impl Throttle {
    pub fn new(duration: Duration) -> Self {
        Self {
            delay: Delay::new(duration),
            duration,
        }
    }
}

impl Stream for Throttle {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        ready!(self.delay.poll_unpin(cx));
        let duration = self.duration;
        self.delay.reset(duration);

        Poll::Ready(Some(()))
    }
}
