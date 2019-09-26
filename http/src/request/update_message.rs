use super::prelude::*;
use dawn_model::{
    channel::{embed::Embed, Message},
    id::{ChannelId, MessageId},
};

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
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
/// client.update_message(ChannelId(1), MessageId(2))
///     .content("test update".to_owned())
///     .await?;
/// # Ok(()) } fn main() {}
/// ```
///
/// Remove the message's content:
///
/// ```rust,no_run
/// use dawn_http::Client;
/// use dawn_model::id::{ChannelId, MessageId};
///
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
/// client.update_message(ChannelId(1), MessageId(2))
///     .content(None)
///     .await?;
/// # Ok(()) } fn main() {}
/// ```
///
/// [embed]: #method.embed
#[derive(Serialize)]
pub struct UpdateMessage<'a> {
    // We don't serialize if this is Option::None, to avoid overwriting the
    // field without meaning to.
    //
    // So we use a nested Option, representing the following states:
    //
    // - Some(Some(String)): Modifying the "content" from one state to a string;
    // - Some(None): Removing the "content" by giving the Discord API a written
    //   `"content": null` in the JSON;
    // - None: Don't serialize the field at all, not modifying the state.
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<Option<Embed>>,
    #[serde(skip)]
    channel_id: ChannelId,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Message>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    message_id: MessageId,
}

impl<'a> UpdateMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            content: None,
            embed: None,
            fut: None,
            http,
            message_id: message_id.into(),
        }
    }

    /// Set the content of the message.
    ///
    /// Pass `None` if you want to remove the message content.
    pub fn content(mut self, content: impl Into<Option<String>>) -> Self {
        self.content.replace(content.into());

        self
    }

    /// Set the embed of the message.
    ///
    /// Pass `None` if you want to remove the message embed.
    pub fn embed(mut self, embed: impl Into<Option<Embed>>) -> Self {
        self.embed.replace(embed.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdateMessage {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateMessage<'_>, Message);
