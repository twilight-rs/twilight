//! Update a message created by a webhook via execution.

use crate::{
    client::Client,
    error::Result,
    request::{
        self,
        applications::InteractionError,
        channel::allowed_mentions::AllowedMentions,
        validate::{self, EmbedValidationError},
        AuditLogReason, AuditLogReasonError, Pending, Request,
    },
    routing::Route,
};
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::embed::Embed,
    id::{ApplicationId, MessageId, WebhookId},
};

/// A webhook's message can not be updated as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UpdateWebhookMessageError {
    /// Content is over 2000 UTF-16 characters.
    ContentInvalid {
        /// Provided content.
        content: String,
    },
    /// Length of one of the embeds is over 6000 characters.
    EmbedTooLarge {
        /// Provided embeds.
        embeds: Vec<Embed>,
        /// Index of the embed that was too large.
        ///
        /// This can be used to index into [`embeds`] to retrieve the bad embed.
        ///
        /// [`embeds`]: Self::EmbedTooLarge.embeds
        index: usize,
        /// Source of the error.
        source: EmbedValidationError,
    },
    /// Too many embeds were provided.
    ///
    /// A webhook can have up to 10 embeds.
    TooManyEmbeds {
        /// Provided embeds.
        embeds: Vec<Embed>,
    },
}

impl Display for UpdateWebhookMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ContentInvalid { .. } => f.write_str("message content is invalid"),
            Self::EmbedTooLarge { .. } => f.write_str("length of one of the embeds is too large"),
            Self::TooManyEmbeds { embeds } => f.write_fmt(format_args!(
                "{} embeds were provided, but only 10 may be provided",
                embeds.len()
            )),
        }
    }
}

impl Error for UpdateWebhookMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmbedTooLarge { source, .. } => Some(source),
            Self::ContentInvalid { .. } | Self::TooManyEmbeds { .. } => None,
        }
    }
}

#[derive(Default, Serialize)]
struct UpdateWebhookMessageFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<AllowedMentions>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Option<Vec<Embed>>>,
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
/// use twilight_http::request::channel::allowed_mentions::AllowedMentions;
/// use twilight_model::id::{MessageId, WebhookId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token");
/// client.update_webhook_message(WebhookId(1), "token here", MessageId(2))
///     // By creating a default set of allowed mentions, no entity can be
///     // mentioned.
///     .allowed_mentions(AllowedMentions::default())
///     .content(Some("test <@3>".to_owned()))?
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`DeleteWebhookMessage`]: super::DeleteWebhookMessage
pub struct UpdateWebhookMessage<'a> {
    fields: UpdateWebhookMessageFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<String>,
    route: Route,
}

impl<'a> UpdateWebhookMessage<'a> {
    /// Maximum number of embeds that a webhook's message may have.
    pub const EMBED_COUNT_LIMIT: usize = 10;

    pub(crate) fn new(
        http: &'a Client,
        webhook_id: WebhookId,
        token: impl Into<String>,
        message_id: MessageId,
    ) -> Self {
        Self {
            fields: UpdateWebhookMessageFields {
                allowed_mentions: http.default_allowed_mentions(),
                ..UpdateWebhookMessageFields::default()
            },
            fut: None,
            http,
            reason: None,
            route: Route::UpdateWebhookMessage {
                message_id: message_id.0,
                token: token.into(),
                webhook_id: webhook_id.0,
            },
        }
    }

    pub(crate) fn new_interaction(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        interaction_token: impl Into<String>,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            fields: UpdateWebhookMessageFields {
                allowed_mentions: http.default_allowed_mentions(),
                ..UpdateWebhookMessageFields::default()
            },
            fut: None,
            http,
            reason: None,
            route: Route::UpdateInteractionOriginal {
                application_id: application_id.0,
                interaction_token: interaction_token.into(),
            },
        })
    }

    /// Set the allowed mentions in the message.
    pub fn allowed_mentions(mut self, allowed: AllowedMentions) -> Self {
        self.fields.allowed_mentions.replace(allowed);

        self
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
    /// Returns [`UpdateWebhookMessageError::ContentInvalid`] if the content
    /// length is too long.
    pub fn content(mut self, content: Option<String>) -> Result<Self, UpdateWebhookMessageError> {
        if let Some(content_ref) = content.as_ref() {
            if !validate::content_limit(content_ref) {
                return Err(UpdateWebhookMessageError::ContentInvalid {
                    content: content.expect("content is known to be some"),
                });
            }
        }

        self.fields.content.replace(content);

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
    /// use twilight_model::id::{MessageId, WebhookId};
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token");
    /// let embed = EmbedBuilder::new()
    ///     .description("Powerful, flexible, and scalable ecosystem of Rust libraries for the Discord API.")?
    ///     .title("Twilight")?
    ///     .url("https://twilight.rs")
    ///     .build()?;
    ///
    /// client.update_webhook_message(WebhookId(1), "token", MessageId(2))
    ///     .embeds(Some(vec![embed]))?
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`UpdateWebhookMessageError::EmbedTooLarge`] if one of the
    /// embeds are too large.
    ///
    /// Returns [`UpdateWebhookMessageError::TooManyEmbeds`] if more than 10
    /// embeds are provided.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    /// [`EMBED_COUNT_LIMIT`]: Self::EMBED_COUNT_LIMIT
    pub fn embeds(mut self, embeds: Option<Vec<Embed>>) -> Result<Self, UpdateWebhookMessageError> {
        if let Some(embeds_present) = embeds.as_deref() {
            if embeds_present.len() > Self::EMBED_COUNT_LIMIT {
                return Err(UpdateWebhookMessageError::TooManyEmbeds {
                    embeds: embeds.expect("embeds are known to be present"),
                });
            }

            for (idx, embed) in embeds_present.iter().enumerate() {
                if let Err(source) = validate::embed(&embed) {
                    return Err(UpdateWebhookMessageError::EmbedTooLarge {
                        embeds: embeds.expect("embeds are known to be present"),
                        index: idx,
                        source,
                    });
                }
            }
        }

        self.fields.embeds.replace(embeds);

        Ok(self)
    }

    fn request(&self) -> Result<Request> {
        let body = crate::json_to_vec(&self.fields)?;
        let route = self.route.clone();

        Ok(if let Some(reason) = &self.reason {
            let headers = request::audit_header(&reason)?;
            Request::from((body, headers, route))
        } else {
            Request::from((body, route))
        })
    }

    fn start(&mut self) -> Result<()> {
        let request = self.request()?;
        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for UpdateWebhookMessage<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(UpdateWebhookMessage<'_>, ());

#[cfg(test)]
mod tests {
    use super::{UpdateWebhookMessage, UpdateWebhookMessageFields};
    use crate::{
        client::Client,
        request::{AuditLogReason, Request},
        routing::Route,
    };
    use twilight_model::id::{MessageId, WebhookId};

    #[test]
    fn test_request() {
        let client = Client::new("token");
        let builder = UpdateWebhookMessage::new(&client, WebhookId(1), "token", MessageId(2))
            .content(Some("test".to_owned()))
            .expect("'test' content couldn't be set")
            .reason("reason")
            .expect("'reason' is not a valid reason");
        let actual = builder.request().expect("failed to create request");

        let body = crate::json_to_vec(&UpdateWebhookMessageFields {
            allowed_mentions: None,
            content: Some(Some("test".to_owned())),
            embeds: None,
        })
        .expect("failed to serialize fields");
        let route = Route::UpdateWebhookMessage {
            message_id: 2,
            token: "token".to_owned(),
            webhook_id: 1,
        };
        let expected = Request::from((body, route));

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}
