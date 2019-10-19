use super::prelude::*;
use dawn_model::{
    channel::{embed::Embed, Message},
    id::{ChannelId, MessageId},
};

#[derive(Default, Serialize)]
struct UpdateMessageFields {
    // We don't serialize if this is Option::None, to avoid overwriting the
    // field without meaning to.
    //
    // So we use a nested Option, representing the following states:
    //
    // - Some(Some(String)): Modifying the "content" from one state to a string;
    // - Some(None): Removing the "content" by giving the Discord API a written
    //   `"content": null` in the JSON;
    // - None: Don't serialize the field at all, not modifying the state.
    #[allow(clippy::option_option)]
    content: Option<Option<String>>,
    #[allow(clippy::option_option)]
    embed: Option<Option<Embed>>,
}

/// Futures request to update a message.
///
/// You can pass `None` to any of the methods to remove the associated field.
/// For example, if you have a message with an embed you want to remove, you can
/// use `.[embed](None)` to remove the embed.
///
/// # Examples
///
/// Replace the content with `"test update"`:
///
/// ```rust,no_run
/// use dawn_http::Client;
/// use dawn_model::id::{ChannelId, MessageId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
/// client.update_message(ChannelId(1), MessageId(2))
///     .content("test update".to_owned())
///     .await?;
/// # Ok(()) }
/// ```
///
/// Remove the message's content:
///
/// ```rust,no_run
/// use dawn_http::Client;
/// use dawn_model::id::{ChannelId, MessageId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
/// client.update_message(ChannelId(1), MessageId(2))
///     .content(None)
///     .await?;
/// # Ok(()) }
/// ```
///
/// [embed]: #method.embed
pub struct UpdateMessage<'a> {
    channel_id: ChannelId,
    fields: UpdateMessageFields,
    fut: Option<Pending<'a, Message>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> UpdateMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fields: UpdateMessageFields::default(),
            fut: None,
            http,
            message_id,
        }
    }

    /// Set the content of the message.
    ///
    /// Pass `None` if you want to remove the message content.
    pub fn content(mut self, content: impl Into<Option<String>>) -> Self {
        self.fields.content.replace(content.into());

        self
    }

    /// Set the embed of the message.
    ///
    /// Pass `None` if you want to remove the message embed.
    pub fn embed(mut self, embed: impl Into<Option<Embed>>) -> Self {
        self.fields.embed.replace(embed.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateMessage {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateMessage<'_>, Message);
