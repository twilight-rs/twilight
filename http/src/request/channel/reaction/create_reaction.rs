use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Pending, Request},
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
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let message_id = MessageId(456);
/// let emoji = RequestReactionType::Unicode { name: String::from("🌃") };
///
/// let reaction = client
///     .create_reaction(channel_id, message_id, emoji)
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateReaction<'a> {
    channel_id: ChannelId,
    emoji: RequestReactionType,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> CreateReaction<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            fut: None,
            http,
            message_id,
        }
    }

    fn request(&self) -> Request {
        Request::from_route(Route::CreateReaction {
            channel_id: self.channel_id.0,
            emoji: self.emoji.display().to_string(),
            message_id: self.message_id.0,
        })
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = self.request();

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(CreateReaction<'_>, ());

#[cfg(test)]
mod tests {
    use super::CreateReaction;
    use crate::{
        request::{channel::reaction::RequestReactionType, Request},
        routing::Route,
        Client,
    };
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    use twilight_model::id::{ChannelId, MessageId};

    #[test]
    fn test_request() {
        let client = Client::new("foo");

        let emoji = RequestReactionType::Unicode {
            name: String::from("\u{1f303}"),
        };

        let builder = CreateReaction::new(&client, ChannelId(123), MessageId(456), emoji);
        let actual = builder.request();

        let expected = Request::from_route(Route::CreateReaction {
            channel_id: 123,
            emoji: utf8_percent_encode("\u{1f303}", NON_ALPHANUMERIC).to_string(),
            message_id: 456,
        });

        assert_eq!(actual.path_str, expected.path_str);
    }
}
