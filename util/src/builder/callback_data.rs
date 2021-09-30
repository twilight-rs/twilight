use twilight_model::{
    application::{callback::CallbackData, component::Component},
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
};

/// Builder for [`CallbackData`].
///
/// # Example
/// ```
/// use twilight_util::builder::CallbackDataBuilder;
/// use twilight_model::channel::message::MessageFlags;
///
/// let callback_data = CallbackDataBuilder::new()
///     .content("My callback message".to_string())
///     .flags(MessageFlags::EPHEMERAL)
///     .build();
/// ```
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
#[must_use = "builders have no effect if unused"]
pub struct CallbackDataBuilder(CallbackData);

impl CallbackDataBuilder {
    /// Create a new builder to construct a [`CallbackData`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set allowed mentions in the interaction response.
    #[allow(clippy::missing_const_for_fn)]
    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);

        self
    }

    /// Set message components.
    pub fn components(mut self, components: impl IntoIterator<Item = Component>) -> Self {
        self.0.components = Some(components.into_iter().collect());

        self
    }

    /// Set the content.
    #[allow(clippy::missing_const_for_fn)]
    pub fn content(mut self, content: String) -> Self {
        self.0.content = Some(content);

        self
    }

    /// Set message embeds.
    pub fn embeds(mut self, embeds: impl IntoIterator<Item = Embed>) -> Self {
        self.0.embeds = embeds.into_iter().collect();

        self
    }

    /// Set the interaction flags.
    pub const fn flags(mut self, flags: MessageFlags) -> Self {
        self.0.flags = Some(flags);

        self
    }

    /// Whether the response is TTS.
    pub const fn tts(mut self, value: bool) -> Self {
        self.0.tts = Some(value);

        self
    }

    /// Build the [`CallbackData`] struct.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> CallbackData {
        self.0
    }
}

impl Default for CallbackDataBuilder {
    fn default() -> Self {
        Self(CallbackData {
            allowed_mentions: None,
            components: None,
            content: None,
            embeds: Vec::new(),
            flags: None,
            tts: None,
        })
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
    };

    assert_impl_all!(CallbackDataBuilder: Debug, Default, Send, Sync);

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
            timestamp: Some("a timestamp".to_owned()),
            title: Some("a title".to_owned()),
            url: Some("https://example.com".to_owned()),
            video: None,
        };

        let value = CallbackDataBuilder::new()
            .allowed_mentions(allowed_mentions.clone())
            .components(vec![component.clone()])
            .content("a content".into())
            .embeds(vec![embed.clone()])
            .flags(MessageFlags::empty())
            .tts(false)
            .build();

        let expected = CallbackData {
            allowed_mentions: Some(allowed_mentions),
            components: Some(vec![component]),
            content: Some("a content".to_owned()),
            embeds: vec![embed],
            flags: Some(MessageFlags::empty()),
            tts: Some(false),
        };

        assert_eq!(value, expected);
    }
}
