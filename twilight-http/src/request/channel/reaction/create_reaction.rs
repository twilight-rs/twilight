use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

struct CreateReactionPath<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    message_id: Id<MessageMarker>,
}

/// Create a reaction in a [`Id<ChannelMarker>`] on a [`Id<MessageMarker>`].
///
/// The reaction must be a variant of [`RequestReactionType`].
///
/// # Examples
/// ```no_run
/// use twilight_http::{Client, request::channel::reaction::RequestReactionType};
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
    http: &'a Client,
    path: CreateReactionPath<'a>,
}

impl<'a> CreateReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            http,
            path: CreateReactionPath {
                channel_id,
                emoji,
                message_id,
            },
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
        let CreateReactionPath {
            channel_id,
            emoji,
            message_id,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("messages")
            .id(message_id)
            .resource("reactions")
            .id(emoji)
            .no_resource()
            .me()
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Put, path))
    }
}
