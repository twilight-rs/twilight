//! Update a original response create for a interaction.

use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AttachmentFile, Form, NullableField, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use std::borrow::Cow;
use twilight_model::{
    application::component::Component,
    channel::{embed::Embed, message::AllowedMentions, Attachment, Message},
    id::{marker::ApplicationMarker, Id},
};
use twilight_validate::message::{
    components as validate_components, content as validate_content, embeds as validate_embeds,
    MessageValidationError,
};

#[derive(Serialize)]
struct UpdateOriginalResponseFields<'a> {
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

/// Update the original response created by a interaction.
///
/// A response must always have at least one embed or some amount of
/// content. If you wish to delete a original response refer to
/// [`DeleteOriginalResponse`].
///
/// # Examples
///
/// Update the original response by setting the content to `test <@3>` -
/// attempting to mention user ID 3 - and specifying that only that the user may
/// not be mentioned.
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::{
///     channel::message::AllowedMentions,
///     id::Id,
/// };
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(1);
///
/// client
///     .interaction(application_id)
///     .update_interaction_original("token here")
///     // By creating a default set of allowed mentions, no entity can be
///     // mentioned.
///     .allowed_mentions(AllowedMentions::default())
///     .content(Some("test <@3>"))?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`DeleteOriginalResponse`]: super::DeleteOriginalResponse
#[must_use = "requests must be configured and executed"]
pub struct UpdateOriginalResponse<'a> {
    application_id: Id<ApplicationMarker>,
    attachments: Cow<'a, [AttachmentFile<'a>]>,
    fields: UpdateOriginalResponseFields<'a>,
    http: &'a Client,
    token: &'a str,
}

impl<'a> UpdateOriginalResponse<'a> {
    /// Maximum number of embeds that a original response may have.
    pub const EMBED_COUNT_LIMIT: usize = 10;

    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        interaction_token: &'a str,
    ) -> Self {
        Self {
            application_id,
            fields: UpdateOriginalResponseFields {
                allowed_mentions: None,
                attachments: &[],
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
            },
            attachments: Cow::Borrowed(&[]),
            http,
            token: interaction_token,
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

    /// Set the list of embeds of the original response.
    ///
    /// Pass `None` to remove all of the embeds.
    ///
    /// The maximum number of allowed embeds is defined by
    /// [`EMBED_COUNT_LIMIT`].
    ///
    /// The total character length of each embed must not exceed 6000
    /// characters. Additionally, the internal fields also have character
    /// limits. See [the Discord Docs/Embed Limits].
    ///
    /// # Examples
    ///
    /// Create an embed and update the message with the new embed. The content
    /// of the original message is unaffected and only the embed(s) are
    /// modified.
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_http::Client;
    /// use twilight_embed_builder::EmbedBuilder;
    /// use twilight_model::id::Id;
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let application_id = Id::new(1);
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description("Powerful, flexible, and scalable ecosystem of Rust libraries for the Discord API.")
    ///     .title("Twilight")
    ///     .url("https://twilight.rs")
    ///     .build()?;
    ///
    /// client
    ///     .interaction(application_id)
    ///     .update_interaction_original("token")
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
    /// [the Discord Docs/Embed Limits]: https://discord.com/developers/docs/resources/channel#embed-limits
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
    /// [`attach`]. See [Discord Docs/Create Message] and
    /// [`CreateFollowupMessage::payload_json`].
    ///
    /// [`attach`]: Self::attach
    /// [`CreateFollowupMessage::payload_json`]: super::CreateFollowupMessage::payload_json
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub const fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        self.fields.payload_json = Some(payload_json);

        self
    }

    pub fn exec(self) -> ResponseFuture<Message> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateOriginalResponse<'_> {
    fn try_into_request(mut self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateInteractionOriginal {
            application_id: self.application_id.get(),
            interaction_token: self.token,
        });

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
                if self.fields.allowed_mentions.is_none() {
                    self.fields.allowed_mentions = self.http.default_allowed_mentions();
                }

                let body = crate::json::to_vec(&self.fields).map_err(HttpError::json)?;
                form.payload_json(&body);
            }

            request = request.form(form);
        } else {
            if self.fields.allowed_mentions.is_none() {
                self.fields.allowed_mentions = self.http.default_allowed_mentions();
            }

            request = request.json(&self.fields)?;
        }

        Ok(request.use_authorization_token(false).build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::Client, request::TryIntoRequest};
    use std::error::Error;
    use twilight_http_ratelimiting::Path;
    use twilight_model::id::Id;

    #[test]
    fn test_delete_followup_message() -> Result<(), Box<dyn Error>> {
        let application_id = Id::new(1);
        let token = "foo".to_owned();

        let client = Client::new(String::new());
        let req = client
            .interaction(application_id)
            .update_interaction_original(&token)
            .content(Some("test"))?
            .try_into_request()?;

        assert!(!req.use_authorization_token());
        assert_eq!(
            &Path::WebhooksIdTokenMessagesId(application_id.get(), token),
            req.ratelimit_path()
        );

        Ok(())
    }
}
