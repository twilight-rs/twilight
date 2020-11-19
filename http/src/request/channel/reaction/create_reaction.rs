use crate::request::prelude::*;
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
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    emoji: String,
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
            emoji: super::format_emoji(emoji),
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::CreateReaction {
                channel_id: self.channel_id.0,
                emoji: self.emoji.clone(),
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(CreateReaction<'_>, ());
