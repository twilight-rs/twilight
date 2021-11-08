use super::RequestReactionType;
use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

/// Create a reaction in a [`Id<ChannelMarker>`] on a [`Id<MessageMarker>`].
///
/// The reaction must be a variant of [`RequestReactionType`].
///
/// # Examples
/// ```rust,no_run
/// use twilight_http::{Client, request::channel::reaction::RequestReactionType};
/// use twilight_model::{
///     id::Id,
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(123).expect("non zero");
/// let message_id = Id::new(456).expect("non zero");
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

    fn request(&self) -> Request {
        Request::from_route(&Route::CreateReaction {
            channel_id: self.channel_id.get(),
            emoji: self.emoji,
            message_id: self.message_id.get(),
        })
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.http.request(self.request())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::non_ascii_literal)]

    use super::CreateReaction;
    use crate::{
        request::{channel::reaction::RequestReactionType, Request},
        routing::Route,
        Client,
    };
    use twilight_model::id::Id;

    #[test]
    fn test_request() {
        let client = Client::new("foo".to_owned());

        let emoji = RequestReactionType::Unicode { name: "🌃" };

        let builder = CreateReaction::new(
            &client,
            Id::new(123).expect("non zero"),
            Id::new(456).expect("non zero"),
            &emoji,
        );
        let actual = builder.request();

        let expected = Request::from_route(&Route::CreateReaction {
            channel_id: 123,
            emoji: &RequestReactionType::Unicode { name: "🌃" },
            message_id: 456,
        });

        assert_eq!(actual.path, expected.path);
    }
}
