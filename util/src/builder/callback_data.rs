use twilight_model::{
    application::{callback::CallbackData, component::Component},
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
};

/// Create a [`CallbackData`] with a builder.
///
/// # Example
/// ```
/// use twilight_util::builder::CallbackDataBuilder;
/// use twilight_model::{
///     channel::message::MessageFlags,
///     application::component::{button::ButtonStyle, Component, Button}
/// };
///
/// let component = Component::Button(Button {
///    style: ButtonStyle::Primary,
///    emoji: None,
///    label: Some("Button label".to_string()),
///    custom_id: Some("button_id".to_string()),
///    url: None,
///    disabled: false,
/// });
///
/// let callback_data = CallbackDataBuilder::new()
///     .content("Callback message".to_string())
///     .flags(MessageFlags::EPHEMERAL)
///     .components([component.clone()])
///     .build();
///
/// assert_eq!(callback_data.components, Some(vec![component]));
/// ```
#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct CallbackDataBuilder(CallbackData);

impl CallbackDataBuilder {
    /// Create a new builder to construct a [`CallbackData`].
    pub const fn new() -> Self {
        Self(CallbackData {
            allowed_mentions: None,
            components: None,
            content: None,
            embeds: None,
            flags: None,
            tts: None,
        })
    }

    /// Consume the builder, returning a [`CallbackData`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> CallbackData {
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
    #[allow(clippy::missing_const_for_fn)]
    pub fn content(mut self, content: String) -> Self {
        self.0.content = Some(content);

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
    /// The only supported flag is [`EPHEMERAL`].
    ///
    /// Defaults to [`None`].
    ///
    /// [`EPHEMERAL`]: twilight_model::channel::message::MessageFlags::EPHEMERAL
    pub const fn flags(mut self, flags: MessageFlags) -> Self {
        self.0.flags = Some(flags);

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

impl Default for CallbackDataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::CallbackDataBuilder;

    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::{
        application::{
            callback::CallbackData,
            component::{button::ButtonStyle, Button, Component},
        },
        channel::{
            embed::Embed,
            message::{AllowedMentions, MessageFlags},
        },
        datetime::Timestamp,
    };

    assert_impl_all!(CallbackDataBuilder: Clone, Debug, Default, Send, Sync);

    #[test]
    fn callback_data_builder() {
        let allowed_mentions = AllowedMentions::builder().everyone().build();

        let component = Component::Button(Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: Some("test label".into()),
            custom_id: Some("test custom id".into()),
            url: None,
            disabled: false,
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

        let value = CallbackDataBuilder::new()
            .allowed_mentions(allowed_mentions.clone())
            .components([component.clone()])
            .content("a content".into())
            .embeds([embed.clone()])
            .flags(MessageFlags::empty())
            .tts(false)
            .build();

        let expected = CallbackData {
            allowed_mentions: Some(allowed_mentions),
            components: Some(vec![component]),
            content: Some("a content".to_owned()),
            embeds: Some(vec![embed]),
            flags: Some(MessageFlags::empty()),
            tts: Some(false),
        };

        assert_eq!(value, expected);
    }
}
