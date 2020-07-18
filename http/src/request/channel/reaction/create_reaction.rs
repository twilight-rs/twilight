use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{
    channel::ReactionType,
    id::{ChannelId, MessageId},
};

/// Create a reaction in a [`ChannelId`] on a [`MessageId`].
///
/// The reaction must be a variant of [`ReactionType`].
///
/// # Examples
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::{
///     channel::ReactionType,
///     id::{ChannelId, MessageId},
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let message_id = MessageId(456);
/// let emoji = ReactionType::Unicode { name: String::from("🌃") };
///
/// let reaction = client
///     .create_reaction(channel_id, message_id, emoji)
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`ChannelId`]: ../../../../twilight_model/id/struct.ChannelId.html
/// [`MessageId`]: ../../../../twilight_model/id/struct.MessageId.html
/// [`ReactionType`]: ../../../../twilight_model/channel/enum.ReactionType.html
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
        emoji: ReactionType,
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
                emoji: Cow::Owned(self.emoji.clone()),
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(CreateReaction<'_>, ());
