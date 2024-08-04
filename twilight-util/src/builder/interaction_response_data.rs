use twilight_model::{
    application::command::CommandOptionChoice,
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::{attachment::Attachment, interaction::InteractionResponseData},
};

/// Create an [`InteractionResponseData`] with a builder.
///
/// # Example
/// ```
/// use twilight_model::channel::message::{
///     component::{ActionRow, Button, ButtonStyle, Component},
///     MessageFlags,
/// };
/// use twilight_util::builder::InteractionResponseDataBuilder;
///
/// let component = Component::ActionRow(ActionRow {
///     components: Vec::from([Component::Button(Button {
///         style: ButtonStyle::Primary,
///         emoji: None,
///         label: Some("Button label".to_string()),
///         custom_id: Some("button_id".to_string()),
///         url: None,
///         disabled: false,
///         sku_id: None,
///     })]),
/// });
///
/// let interaction_response_data = InteractionResponseDataBuilder::new()
///     .content("Callback message")
///     .flags(MessageFlags::EPHEMERAL)
///     .components([component.clone()])
///     .build();
///
/// assert_eq!(interaction_response_data.components, Some(vec![component]));
/// ```
#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct InteractionResponseDataBuilder(InteractionResponseData);

impl InteractionResponseDataBuilder {
    /// Create a new builder to construct an [`InteractionResponseData`].
    pub const fn new() -> Self {
        Self(InteractionResponseData {
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: None,
            custom_id: None,
            embeds: None,
            flags: None,
            title: None,
            tts: None,
        })
    }

    /// Consume the builder, returning an [`InteractionResponseData`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> InteractionResponseData {
        self.0
    }

    /// Set the [`AllowedMentions`] of the callback.
    ///
    /// Defaults to [`None`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);

        self
    }

    /// Set the attachments of the message.
    ///
    /// Defaults to [`None`].
    pub fn attachments(mut self, attachments: impl IntoIterator<Item = Attachment>) -> Self {
        self.0.attachments = Some(attachments.into_iter().collect());

        self
    }

    /// Set the autocomplete choices of the response.
    ///
    /// Only valid when the type of the interaction is
    /// [`ApplicationCommandAutocompleteResult`].
    ///
    /// [`ApplicationCommandAutocompleteResult`]: twilight_model::http::interaction::InteractionResponseType::ApplicationCommandAutocompleteResult
    pub fn choices(mut self, choices: impl IntoIterator<Item = CommandOptionChoice>) -> Self {
        self.0.choices = Some(choices.into_iter().collect());

        self
    }

    /// Set the message [`Component`]s of the callback.
    ///
    /// Defaults to [`None`].
    pub fn components(mut self, components: impl IntoIterator<Item = Component>) -> Self {
        self.0.components = Some(components.into_iter().collect());

        self
    }

    /// Set the message content of the callback.
    ///
    /// Defaults to [`None`].
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = Some(content.into());

        self
    }

    /// Set the custom ID of the callback.
    ///
    /// Defaults to [`None`].
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.0.custom_id = Some(custom_id.into());

        self
    }

    /// Set the [`Embed`]s of the callback.
    ///
    /// Defaults to an empty list.
    pub fn embeds(mut self, embeds: impl IntoIterator<Item = Embed>) -> Self {
        self.0.embeds = Some(embeds.into_iter().collect());

        self
    }

    /// Set the [`MessageFlags`].
    ///
    /// The only supported flags are [`EPHEMERAL`] and [`SUPPRESS_EMBEDS`].
    ///
    /// Defaults to [`None`].
    ///
    /// [`EPHEMERAL`]: twilight_model::channel::message::MessageFlags::EPHEMERAL
    /// [`SUPPRESS_EMBEDS`]: twilight_model::channel::message::MessageFlags::SUPPRESS_EMBEDS
    pub const fn flags(mut self, flags: MessageFlags) -> Self {
        self.0.flags = Some(flags);

        self
    }

    /// Set the title of the callback.
    ///
    /// Defaults to [`None`].
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.title = Some(title.into());

        self
    }

    /// Set whether the response has text-to-speech enabled.
    ///
    /// Defaults to [`None`].
    pub const fn tts(mut self, value: bool) -> Self {
        self.0.tts = Some(value);

        self
    }
}

impl Default for InteractionResponseDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::{
        channel::message::{
            component::{Button, ButtonStyle},
            MentionType,
        },
        util::Timestamp,
    };

    assert_impl_all!(
        InteractionResponseDataBuilder: Clone,
        Debug,
        Default,
        Send,
        Sync
    );

    #[test]
    fn callback_data_builder() {
        let allowed_mentions = AllowedMentions {
            parse: Vec::from([MentionType::Everyone]),
            ..Default::default()
        };

        let component = Component::Button(Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: Some("test label".into()),
            custom_id: Some("test custom id".into()),
            url: None,
            disabled: false,
            sku_id: None,
        });

        let embed = Embed {
            author: None,
            color: Some(123),
            description: Some("a description".to_owned()),
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: "rich".to_owned(),
            provider: None,
            thumbnail: None,
            timestamp: Some(Timestamp::from_secs(1_580_608_922).unwrap()),
            title: Some("a title".to_owned()),
            url: Some("https://example.com".to_owned()),
            video: None,
        };

        let value = InteractionResponseDataBuilder::new()
            .allowed_mentions(allowed_mentions.clone())
            .components([component.clone()])
            .content("a content")
            .embeds([embed.clone()])
            .flags(MessageFlags::empty())
            .tts(false)
            .build();

        let expected = InteractionResponseData {
            allowed_mentions: Some(allowed_mentions),
            attachments: None,
            choices: None,
            components: Some(vec![component]),
            content: Some("a content".to_owned()),
            custom_id: None,
            embeds: Some(vec![embed]),
            flags: Some(MessageFlags::empty()),
            title: None,
            tts: Some(false),
        };

        assert_eq!(value, expected);
    }
}
