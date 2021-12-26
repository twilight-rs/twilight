//! Update a message created by a webhook via execution.

use crate::{
    client::Client,
    error::Error as HttpError,
    request::{
        self,
        validate_inner::{self, ComponentValidationError, ComponentValidationErrorType},
        AttachmentFile, AuditLogReason, AuditLogReasonError, FormBuilder, NullableField,
        PartialAttachment, Request, TryIntoRequest,
    },
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    application::component::Component,
    channel::{embed::Embed, message::AllowedMentions},
    id::{
        marker::{AttachmentMarker, ChannelMarker, MessageMarker, WebhookMarker},
        Id,
    },
};

/// A webhook's message can not be updated as configured.
#[derive(Debug)]
pub struct UpdateWebhookMessageError {
    kind: UpdateWebhookMessageErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl UpdateWebhookMessageError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &UpdateWebhookMessageErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        UpdateWebhookMessageErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl Display for UpdateWebhookMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            UpdateWebhookMessageErrorType::ComponentCount { count } => {
                Display::fmt(count, f)?;
                f.write_str(" components were provided, but only ")?;
                Display::fmt(&ComponentValidationError::COMPONENT_COUNT, f)?;

                f.write_str(" root components are allowed")
            }
            UpdateWebhookMessageErrorType::ComponentInvalid { .. } => {
                f.write_str("a provided component is invalid")
            }
            UpdateWebhookMessageErrorType::ContentInvalid => {
                f.write_str("message content is invalid")
            }
            UpdateWebhookMessageErrorType::EmbedTooLarge { .. } => {
                f.write_str("length of one of the embeds is too large")
            }
            UpdateWebhookMessageErrorType::TooManyEmbeds => {
                f.write_str("only 10 embeds may be provided")
            }
        }
    }
}

impl Error for UpdateWebhookMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`UpdateWebhookMessageError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateWebhookMessageErrorType {
    /// Content is over 2000 UTF-16 characters.
    ContentInvalid,
    /// Length of one of the embeds is over 6000 characters.
    EmbedTooLarge {
        /// Index of the embed that was too large.
        ///
        /// This can be used to index into the provided embeds to retrieve the
        /// invalid embed.
        index: usize,
    },
    /// An invalid message component was provided.
    ComponentInvalid {
        /// Additional details about the validation failure type.
        kind: ComponentValidationErrorType,
    },
    /// Too many message components were provided.
    ComponentCount {
        /// Number of components that were provided.
        count: usize,
    },
    /// Too many embeds were provided.
    ///
    /// A webhook can have up to 10 embeds.
    TooManyEmbeds,
}

#[derive(Serialize)]
struct UpdateWebhookMessageFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<AllowedMentions>,
    /// List of attachments to keep, and new attachments to add.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<PartialAttachment<'a>>,
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
/// client.update_webhook_message(Id::new(1).expect("non zero"), "token here", Id::new(2).expect("non zero"))
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
    /// List of new attachments to add to the message.
    attachments: Option<&'a [AttachmentFile<'a>]>,
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
            attachments: None,
            fields: UpdateWebhookMessageFields {
                allowed_mentions: None,
                attachments: Vec::new(),
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
            },
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

    /// Attach multiple files to the message.
    ///
    /// This no longer clears previous calls.
    pub fn attach(mut self, attachments: &'a [AttachmentFile<'a>]) -> Self {
        for (index, attachment) in attachments.iter().enumerate() {
            self.fields.attachments.push(PartialAttachment {
                description: attachment.description,
                filename: Some(attachment.filename),
                id: index as u64,
            })
        }

        // Sort and deduplicate the list of partial attachments.
        self.fields.attachments.sort_by(|a, b| a.id.cmp(&b.id));
        self.fields.attachments.dedup();

        self.attachments = Some(attachments);

        self
    }

    /// Specify multiple [`Id<AttachmentMarker>`]s already present in the target
    /// message to keep.
    ///
    /// If called, all unspecified attachments (except ones added with
    /// [`attach`]) will be removed from the message. If not called, all
    /// attachments will be kept.
    ///
    /// [`attach`]: Self::attach
    pub fn attachment_ids(mut self, attachment_ids: &[Id<AttachmentMarker>]) -> Self {
        self.fields
            .attachments
            .extend(attachment_ids.iter().map(|id| PartialAttachment {
                description: None,
                filename: None,
                id: id.get(),
            }));

        // Sort and deduplicate the list of partial attachments.
        self.fields.attachments.sort_by(|a, b| a.id.cmp(&b.id));
        self.fields.attachments.dedup();

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
    /// Returns an [`UpdateWebhookMessageErrorType::ComponentCount`] error
    /// type if too many components are provided.
    ///
    /// Returns an [`UpdateWebhookMessageErrorType::ComponentInvalid`] error
    /// type if one of the provided components is invalid.
    pub fn components(
        mut self,
        components: Option<&'a [Component]>,
    ) -> Result<Self, UpdateWebhookMessageError> {
        if let Some(components) = components.as_ref() {
            validate_inner::components(components).map_err(|source| {
                let (kind, inner_source) = source.into_parts();

                match kind {
                    ComponentValidationErrorType::ComponentCount { count } => {
                        UpdateWebhookMessageError {
                            kind: UpdateWebhookMessageErrorType::ComponentCount { count },
                            source: inner_source,
                        }
                    }
                    other => UpdateWebhookMessageError {
                        kind: UpdateWebhookMessageErrorType::ComponentInvalid { kind: other },
                        source: inner_source,
                    },
                }
            })?;
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
    /// Returns an [`UpdateWebhookMessageErrorType::ContentInvalid`] error type if
    /// the content length is too long.
    pub fn content(mut self, content: Option<&'a str>) -> Result<Self, UpdateWebhookMessageError> {
        if let Some(content_ref) = content {
            if !validate_inner::content_limit(content_ref) {
                return Err(UpdateWebhookMessageError {
                    kind: UpdateWebhookMessageErrorType::ContentInvalid,
                    source: None,
                });
            }
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
    /// client.update_webhook_message(Id::new(1).expect("non zero"), "token", Id::new(2).expect("non zero"))
    ///     .embeds(Some(&[embed]))?
    ///     .exec()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`UpdateWebhookMessageErrorType::EmbedTooLarge`] error type
    /// if one of the embeds are too large.
    ///
    /// Returns an [`UpdateWebhookMessageErrorType::TooManyEmbeds`] error type
    /// if more than 10 embeds are provided.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    /// [`EMBED_COUNT_LIMIT`]: Self::EMBED_COUNT_LIMIT
    pub fn embeds(
        mut self,
        embeds: Option<&'a [Embed]>,
    ) -> Result<Self, UpdateWebhookMessageError> {
        if let Some(embeds_present) = embeds {
            if embeds_present.len() > Self::EMBED_COUNT_LIMIT {
                return Err(UpdateWebhookMessageError {
                    kind: UpdateWebhookMessageErrorType::TooManyEmbeds,
                    source: None,
                });
            }

            for (idx, embed) in embeds_present.iter().enumerate() {
                if let Err(source) = validate_inner::embed(embed) {
                    return Err(UpdateWebhookMessageError {
                        kind: UpdateWebhookMessageErrorType::EmbedTooLarge { index: idx },
                        source: Some(Box::new(source)),
                    });
                }
            }
        }

        self.fields.embeds = Some(NullableField(embeds));

        Ok(self)
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`attach`]. See [Discord Docs/Create Message] and
    /// [`ExecuteWebhook::payload_json`].
    ///
    /// [`attach`]: Self::attach
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

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if self.attachments.is_some() || self.fields.payload_json.is_some() {
            let mut form_builder = if let Some(payload_json) = self.fields.payload_json {
                FormBuilder::new(Cow::Borrowed(payload_json))
            } else {
                crate::json::to_vec(&self.fields)
                    .map(Cow::Owned)
                    .map(FormBuilder::new)
                    .map_err(HttpError::json)?
            };

            if let Some(attachments) = self.attachments {
                form_builder = form_builder.attachments(attachments);
            }

            request = request.form(form_builder.build());
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
        let builder = UpdateWebhookMessage::new(
            &client,
            Id::new(1).expect("non zero"),
            "token",
            Id::new(2).expect("non zero"),
        )
        .content(Some("test"))
        .expect("'test' content couldn't be set")
        .thread_id(Id::new(3).expect("non zero"))
        .reason("reason")
        .expect("'reason' is not a valid reason");
        let actual = builder
            .try_into_request()
            .expect("failed to create request");

        let body = UpdateWebhookMessageFields {
            allowed_mentions: None,
            attachments: Vec::new(),
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
