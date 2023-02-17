use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

/// Create a reaction in a [`Id<ChannelMarker>`] on a [`Id<MessageMarker>`].
///
/// The reaction must be a variant of [`RequestReactionType`].
///
/// # Examples
/// ```no_run
/// use twilight_http::{request::channel::reaction::RequestReactionType, Client};
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(123);
/// let message_id = Id::new(456);
/// let emoji = RequestReactionType::Unicode { name: "🌃" };
///
/// let reaction = client
///     .create_reaction(channel_id, message_id, &emoji)
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateReaction<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> CreateReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            http,
            message_id,
        }
    }
}

impl IntoFuture for CreateReaction<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateReaction<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::CreateReaction {
            channel_id: self.channel_id.get(),
            emoji: self.emoji,
            message_id: self.message_id.get(),
        }))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::non_ascii_literal)]

    use super::CreateReaction;
    use crate::{
        request::{channel::reaction::RequestReactionType, Request, TryIntoRequest},
        routing::Route,
        Client,
    };
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());

        let emoji = RequestReactionType::Unicode { name: "🌃" };

        let builder = CreateReaction::new(&client, Id::new(123), Id::new(456), &emoji);
        let actual = builder.try_into_request()?;

        let expected = Request::from_route(&Route::CreateReaction {
            channel_id: 123,
            emoji: &RequestReactionType::Unicode { name: "🌃" },
            message_id: 456,
        });

        assert_eq!(actual.path, expected.path);

        Ok(())
    }
}
