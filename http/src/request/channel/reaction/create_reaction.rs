use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ChannelId, MessageId};

/// Create a reaction in a [`ChannelId`] on a [`MessageId`].
///
/// The reaction must be a variant of [`RequestReactionType`].
///
/// # Examples
/// ```rust,no_run
/// use twilight_http::{Client, request::channel::reaction::RequestReactionType};
/// use twilight_model::{
///     id::{ChannelId, MessageId},
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = ChannelId::new(123).expect("non zero");
/// let message_id = MessageId::new(456).expect("non zero");
/// let emoji = RequestReactionType::Unicode { name: "🌃" };
///
/// let reaction = client
///     .create_reaction(channel_id, message_id, &emoji)
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateReaction<'a> {
    channel_id: ChannelId,
    emoji: &'a RequestReactionType<'a>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> CreateReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            http,
            message_id,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
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
    use twilight_model::id::{ChannelId, MessageId};

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("foo".to_owned());

        let emoji = RequestReactionType::Unicode { name: "🌃" };

        let builder = CreateReaction::new(
            &client,
            ChannelId::new(123).expect("non zero"),
            MessageId::new(456).expect("non zero"),
            &emoji,
        );
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
