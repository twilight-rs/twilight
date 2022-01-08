//! Update a message created by a webhook via execution.

use crate::{
    client::Client,
    error::Error as HttpError,
    request::{
        self, AttachmentFile, AuditLogReason, AuditLogReasonError, Form, NullableField, Request,
        TryIntoRequest,
    },
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::borrow::Cow;
use twilight_model::{
    application::component::Component,
    channel::{embed::Embed, message::AllowedMentions, Attachment},
    id::{
        marker::{ChannelMarker, MessageMarker, WebhookMarker},
        Id,
    },
};
use twilight_validate::message::{
    components as validate_components, content as validate_content, embeds as validate_embeds,
    MessageValidationError,
};

#[derive(Serialize)]
struct UpdateWebhookMessageFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "request::slice_is_empty")]
    attachments: &'a [Attachment],
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<NullableField<&'a [Component]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<NullableField<&'a [Embed]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<&'a [u8]>,
}

/// Update a message created by a webhook.
///
/// A webhook's message must always have at least one embed or some amount of
/// content. If you wish to delete a webhook's message refer to
/// [`DeleteWebhookMessage`].
///
/// # Examples
///
/// Update a webhook's message by setting the content to `test <@3>` -
/// attempting to mention user ID 3 - and specifying that only that the user may
/// not be mentioned.
///
/// ```no_run
/// # use twilight_http::Client;
/// use twilight_model::{
///     channel::message::AllowedMentions,
///     id::Id,
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token".to_owned());
/// client.update_webhook_message(Id::new(1), "token here", Id::new(2))
///     // By creating a default set of allowed mentions, no entity can be
///     // mentioned.
///     .allowed_mentions(AllowedMentions::default())
///     .content(Some("test <@3>"))?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`DeleteWebhookMessage`]: super::DeleteWebhookMessage
#[must_use = "requests must be configured and executed"]
pub struct UpdateWebhookMessage<'a> {
    attachments: Cow<'a, [AttachmentFile<'a>]>,
    fields: UpdateWebhookMessageFields<'a>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
    reason: Option<&'a str>,
    thread_id: Option<Id<ChannelMarker>>,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
}

impl<'a> UpdateWebhookMessage<'a> {
    /// Maximum number of embeds that a webhook's message may have.
    pub const EMBED_COUNT_LIMIT: usize = 10;

    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            fields: UpdateWebhookMessageFields {
                allowed_mentions: None,
                attachments: &[],
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
            },
            attachments: Cow::Borrowed(&[]),
            http,
            message_id,
            reason: None,
            thread_id: None,
            token,
            webhook_id,
        }
    }

    /// Set the allowed mentions in the message.
    pub fn allowed_mentions(mut self, allowed: AllowedMentions) -> Self {
        self.fields.allowed_mentions.replace(allowed);

        self
    }

    /// Specify multiple attachments already present in the target message to keep.
    ///
    /// If called, all unspecified attachments will be removed from the message.
    /// If not called, all attachments will be kept.
    pub const fn attachments(mut self, attachments: &'a [Attachment]) -> Self {
        self.fields.attachments = attachments;

        self
    }

    /// Add multiple [`Component`]s to a message.
    ///
    /// Calling this method multiple times will clear previous calls.
    ///
    /// Pass `None` to clear existing components.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(
        mut self,
        components: Option<&'a [Component]>,
    ) -> Result<Self, MessageValidationError> {
        if let Some(components) = components {
            validate_components(components)?;
        }

        self.fields.components = Some(NullableField(components));

        Ok(self)
    }

    /// Set the content of the message.
    ///
    /// Pass `None` if you want to remove the message content.
    ///
    /// Note that if there is are no embeds then you will not be able to remove
    /// the content of the message.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ContentInvalid`] if the content length is too
    /// long.
    ///
    /// [`ContentInvalid`]: twilight_validate::message::MessageValidationErrorType::ContentInvalid
    pub fn content(mut self, content: Option<&'a str>) -> Result<Self, MessageValidationError> {
        if let Some(content_ref) = content {
            validate_content(content_ref)?;
        }

        self.fields.content = Some(NullableField(content));

        Ok(self)
    }

    /// Set the list of embeds of the webhook's message.
    ///
    /// Pass `None` to remove all of the embeds.
    ///
    /// The maximum number of allowed embeds is defined by
    /// [`EMBED_COUNT_LIMIT`].
    ///
    /// The total character length of each embed must not exceed 6000
    /// characters. Additionally, the internal fields also have character
    /// limits. Refer to [the discord docs] for more information.
    ///
    /// # Examples
    ///
    /// Create an embed and update the message with the new embed. The content
    /// of the original message is unaffected and only the embed(s) are
    /// modified.
    ///
    /// ```no_run
    /// # use twilight_http::Client;
    /// use twilight_embed_builder::EmbedBuilder;
    /// use twilight_model::id::Id;
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token".to_owned());
    /// let embed = EmbedBuilder::new()
    ///     .description("Powerful, flexible, and scalable ecosystem of Rust libraries for the Discord API.")
    ///     .title("Twilight")
    ///     .url("https://twilight.rs")
    ///     .build()?;
    ///
    /// client.update_webhook_message(Id::new(1), "token", Id::new(2))
    ///     .embeds(Some(&[embed]))?
    ///     .exec()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TooManyEmbeds`] if there are too many embeds.
    ///
    /// Otherwise, refer to the errors section of [`embed`] for a list of errors
    /// that may occur.
    ///
    /// [`EMBED_COUNT_LIMIT`]: twilight_validate::message::EMBED_COUNT_LIMIT
    /// [`TooManyEmbeds`]: twilight_validate::message::MessageValidationErrorType::TooManyEmbeds
    /// [`embed`]: twilight_validate::embed::embed
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    pub fn embeds(mut self, embeds: Option<&'a [Embed]>) -> Result<Self, MessageValidationError> {
        if let Some(embeds) = embeds {
            validate_embeds(embeds)?;
        }

        self.fields.embeds = Some(NullableField(embeds));

        Ok(self)
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear any previous calls.
    #[allow(clippy::missing_const_for_fn)] // False positive
    pub fn attach(mut self, attachments: &'a [AttachmentFile<'a>]) -> Self {
        self.attachments = Cow::Borrowed(attachments);

        self
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear any previous calls.
    #[deprecated(since = "0.7.2", note = "Use attach instead")]
    pub fn files(mut self, files: &'a [(&'a str, &'a [u8])]) -> Self {
        self.attachments = Cow::Owned(AttachmentFile::from_pairs(files));

        self
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`files`]. See [Discord Docs/Create Message] and
    /// [`ExecuteWebhook::payload_json`].
    ///
    /// [`files`]: Self::files
    /// [`ExecuteWebhook::payload_json`]: super::ExecuteWebhook::payload_json
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub const fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        self.fields.payload_json = Some(payload_json);

        self
    }

    /// Update in a thread belonging to the channel instead of the channel
    /// itself.
    pub fn thread_id(mut self, thread_id: Id<ChannelMarker>) -> Self {
        self.thread_id.replace(thread_id);

        self
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

impl<'a> AuditLogReason<'a> for UpdateWebhookMessage<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateWebhookMessage<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: self.thread_id.map(Id::get),
            token: self.token,
            webhook_id: self.webhook_id.get(),
        })
        .use_authorization_token(false);

        if !self.attachments.is_empty() || self.fields.payload_json.is_some() {
            let mut form = Form::new();

            if !self.attachments.is_empty() {
                for (index, attachment) in self.attachments.iter().enumerate() {
                    form.attach(
                        index as u64,
                        attachment.filename.as_bytes(),
                        attachment.file,
                    );
                }
            }

            if let Some(payload_json) = &self.fields.payload_json {
                form.payload_json(payload_json);
            } else {
                let mut fields = self.fields;

                if fields.allowed_mentions.is_none() {
                    fields.allowed_mentions = self.http.default_allowed_mentions();
                }

                let body = crate::json::to_vec(&fields).map_err(HttpError::json)?;
                form.payload_json(&body);
            }

            request = request.form(form);
        } else {
            let mut fields = self.fields;

            if fields.allowed_mentions.is_none() {
                fields.allowed_mentions = self.http.default_allowed_mentions();
            }

            request = request.json(&fields)?;
        }

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        Ok(request.build())
    }
}

#[cfg(test)]
mod tests {
    use super::{UpdateWebhookMessage, UpdateWebhookMessageFields};
    use crate::{
        client::Client,
        request::{AuditLogReason, NullableField, Request, TryIntoRequest},
        routing::Route,
    };
    use twilight_model::id::Id;

    #[test]
    fn test_request() {
        let client = Client::new("token".to_owned());
        let builder = UpdateWebhookMessage::new(&client, Id::new(1), "token", Id::new(2))
            .content(Some("test"))
            .expect("'test' content couldn't be set")
            .thread_id(Id::new(3))
            .reason("reason")
            .expect("'reason' is not a valid reason");
        let actual = builder
            .try_into_request()
            .expect("failed to create request");

        let body = UpdateWebhookMessageFields {
            allowed_mentions: None,
            attachments: &[],
            components: None,
            content: Some(NullableField(Some("test"))),
            embeds: None,
            payload_json: None,
        };
        let route = Route::UpdateWebhookMessage {
            message_id: 2,
            thread_id: Some(3),
            token: "token",
            webhook_id: 1,
        };
        let expected = Request::builder(&route)
            .json(&body)
            .expect("failed to serialize body")
            .build();

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}
