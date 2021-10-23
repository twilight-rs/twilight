//! Flow for managing ratelimit tickets.
//!
//! Tickets are the [`Ratelimiter`]'s method of managing approval for a consumer
//! to be able to send a request.
//!
//! # Ratelimit Consumer
//!
//! ## 1. Requesting a ticket
//!
//! Consumers of a ratelimiter will call [`Ratelimiter::ticket`].
//!
//! ## 2. Waiting for approval
//!
//! In return consumers will receive a [`TicketReceiver`]. This must be polled
//! in order to know when the ratelimiter has approved a ticket.
//!
//! ## 3. Receiving approval
//!
//! When a ticket is approved and the future resolves, a [`TicketSender`] is
//! provided. This must be used to provide the ratelimiter with the response's
//! ratelimit headers.
//!
//! ## 4. Performing the request
//!
//! Consumers may now execute the HTTP request associated with the ticket. Once
//! a response (or lack of one) is received, the headers [must be parsed] and
//! sent to the ratelimiter via [`TicketSender::headers`]. This completes the
//! cycle.
//!
//! # Ratelimiter
//!
//! ## 1. Initializing a ticket's channels
//!
//! Ratelimiters will accept a request for a ticket when [`Ratelimiter::ticket`]
//! is called. You must call [`channel`] to create a channel between the
//! ratelimiter and the consumer.
//!
//! ## 2. Keeping the consumer waiting
//!
//! [`channel`] will return two halves: [`TicketNotifier`] and
//! [`TicketReceiver`]. Ratelimiters must keep the notifier and give the user
//! the receiver in return.
//!
//! ## 3. Notifying the consumer of ticket approval
//!
//! When any ratelimits have passed and a user is free to perform their request,
//! call [`TicketNotifier::available`]. If the user hasn't canceled their
//! request for a ticket, you will receive a [`TicketHeaders`].
//!
//! ## 4. Receiving the response's headers
//!
//! The consumer will perform their HTTP request and parse the response's
//! headers. Once the headers (or lack of headers) are available the user will
//! send them along the channel. Poll the provided [`TicketHeaders`] for those
//! headers to complete the cycle.
//!
//! [`Ratelimiter::ticket`]: super::Ratelimiter::ticket
//! [`Ratelimiter`]: super::Ratelimiter
//! [must be parsed]: super::headers

use crate::headers::RatelimitHeaders;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::oneshot::{self, error::RecvError, Receiver, Sender};

/// Receiver to wait for the headers sent by the API consumer.
///
/// You must poll the future in order to process the headers. If the future
/// results to an error, then the API consumer dropped the sernding half of the
/// channel. You should treat this as if the request happened.
#[derive(Debug)]
pub struct TicketHeaders(Receiver<Option<RatelimitHeaders>>);

impl Future for TicketHeaders {
    type Output = Result<Option<RatelimitHeaders>, RecvError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

/// Indicate to the ratelimit consumer that their ticket has been granted and
/// they may now send a request.
#[derive(Debug)]
pub struct TicketNotifier(Sender<Sender<Option<RatelimitHeaders>>>);

impl TicketNotifier {
    /// Signal to the ratelimiter consumer (an HTTP client) that a request may
    /// now be performed.
    ///
    /// A receiver is returned. This must be stored and awaited so that
    /// ratelimiting backends can handle the headers that the API consumer will
    /// send back, thus completing the cycle.
    ///
    /// Returns a `None` if the consumer has dropped their
    /// [`TicketReceiver`] half. The ticket is considered canceled.
    #[must_use]
    pub fn available(self) -> Option<TicketHeaders> {
        let (tx, rx) = oneshot::channel();

        self.0.send(tx).ok()?;

        Some(TicketHeaders(rx))
    }
}

/// Channel receiver to wait for availability of a ratelimit ticket.
///
/// This is used by the ratelimiter consumer (such as an API client) to wait for
/// an available ratelimit ticket.
///
/// Once one is available, a [`TicketSender`] will be produced which can be used to
/// send the associated HTTP response's ratelimit headers.
#[derive(Debug)]
pub struct TicketReceiver(Receiver<Sender<Option<RatelimitHeaders>>>);

impl Future for TicketReceiver {
    type Output = Result<TicketSender, RecvError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx).map_ok(TicketSender)
    }
}

/// Channel sender to send response ratelimit information to the ratelimiter.
///
/// This is used by the ratelimiter consumer (such as an API client) once a
/// request has been granted via [`TicketReceiver`].
///
/// If a response results in available ratelimit headers, send them via
/// [`headers`] to the ratelimiter backend. If a response results in an
/// error - such as a server error or request cancellation - send `None`.
///
/// [`headers`]: Self::headers
#[derive(Debug)]
pub struct TicketSender(Sender<Option<RatelimitHeaders>>);

impl TicketSender {
    /// Send the response's ratelimit headers to the ratelimiter.
    ///
    /// This will allow the ratelimiter to complete the cycle and acknowledge
    /// that the request has been completed. This must be done so that the
    /// ratelimiter can process information such as whether there's a global
    /// ratelimit.
    ///
    /// # Errors
    ///
    /// Returns the input headers if the ratelimiter has dropped the receiver
    /// half. This may happen if the ratelimiter is dropped or if a timeout has
    /// occurred.
    pub fn headers(
        self,
        headers: Option<RatelimitHeaders>,
    ) -> Result<(), Option<RatelimitHeaders>> {
        self.0.send(headers)
    }
}

/// Produce a new channel consisting of a sender and receiver.
///
/// The notifier is to be used by the ratelimiter while the receiver is to be
/// provided to the consumer.
///
/// Refer to the [module-level] documentation for more information.
///
/// [module-level]: self
#[must_use]
pub fn channel() -> (TicketNotifier, TicketReceiver) {
    let (tx, rx) = oneshot::channel();

    (TicketNotifier(tx), TicketReceiver(rx))
}
