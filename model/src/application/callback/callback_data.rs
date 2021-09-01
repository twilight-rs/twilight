use crate::{
    application::component::Component,
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
};

use serde::{Deserialize, Serialize};

/// Optional extra data sent when responding to an [`Interaction`] of type
/// [`ApplicationCommand`].
///
/// This is used when intending to send a message in the response.
///
/// [`Interaction`]: crate::application::interaction::Interaction
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CallbackData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of components to include in the callback response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

impl CallbackData {
    /// Create a new builder to construct a [`CallbackData`].
    pub fn builder() -> CallbackDataBuilder {
        CallbackDataBuilder::new()
    }
}

/// Builder for [`CallbackData`].
#[derive(Debug)]
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

    /// Add a message component.
    ///
    /// Multiple component can be set by calling this method multiple times.
    pub fn component(mut self, component: Component) -> Self {
        match self.0.components {
            Some(ref mut components) => components.push(component),
            None => self.0.components = Some(vec![component]),
        };

        self
    }

    /// Set the content.
    pub fn content(self, content: impl Into<String>) -> Self {
        self._content(content.into())
    }

    #[allow(clippy::missing_const_for_fn)]
    fn _content(mut self, content: String) -> Self {
        self.0.content = Some(content);

        self
    }

    /// Add an embed.
    ///
    /// Multiple embeds can be set by calling this method multiple times.
    pub fn embed(mut self, embed: Embed) -> Self {
        self.0.embeds.push(embed);
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
    use crate::{
        application::component::{button::ButtonStyle, Button, Component},
        channel::{
            embed::Embed,
            message::{AllowedMentions, MessageFlags},
        },
    };

    use super::CallbackData;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        CallbackData: allowed_mentions,
        components,
        content,
        embeds,
        flags,
        tts
    );
    assert_impl_all!(
        CallbackData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

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

        let value = CallbackData::builder()
            .allowed_mentions(allowed_mentions.clone())
            .component(component.clone())
            .content("a content")
            .embed(embed.clone())
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
