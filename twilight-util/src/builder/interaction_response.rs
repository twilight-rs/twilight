//! Create [`InteractionResponse`]s with builders.
//!
//! # Example
//!
//! ```
//! use twilight_util::builder::interaction_response::ChannelMessageBuilder;
//!
//! ChannelMessageBuilder::new().content("Congrats on sending your command!");
//! ```

use twilight_model::{
    application::command::CommandOptionChoice,
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    },
    poll::Poll,
};

/// Builder for a [`InteractionResponse`] of type [`InteractionResponseType::ApplicationCommandAutocompleteResult`].
#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct AutocompleteBuilder(InteractionResponseData);

impl AutocompleteBuilder {
    /// Creates a new default builder.
    pub fn new(choices: impl IntoIterator<Item = CommandOptionChoice>) -> Self {
        Self(InteractionResponseData {
            choices: Some(FromIterator::from_iter(choices)),
            ..Default::default()
        })
    }

    /// Builds the [`InteractionResponse`].
    pub fn build(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
            data: Some(self.0),
        }
    }
}

impl From<AutocompleteBuilder> for InteractionResponse {
    fn from(builder: AutocompleteBuilder) -> Self {
        builder.build()
    }
}

/// Builder for a [`InteractionResponse`] of type [`InteractionResponseType::ChannelMessageWithSource`].
#[derive(Clone, Debug, Default)]
#[must_use = "builders have no effect if unused"]
pub struct ChannelMessageBuilder(InteractionResponseData);

impl ChannelMessageBuilder {
    /// Creates a new default builder.
    pub fn new() -> Self {
        Self(InteractionResponseData::default())
    }

    /// Builds the [`InteractionResponse`].
    pub fn build(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(self.0),
        }
    }

    /// Sets the allowed mentions filter.
    ///
    /// Defaults to no filter.
    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);

        self
    }

    /// Sets the attachments.
    ///
    /// Defaults to no attachments.
    pub fn attachments(mut self, attachments: impl IntoIterator<Item = Attachment>) -> Self {
        self.0.attachments = Some(FromIterator::from_iter(attachments));

        self
    }

    /// Sets the components.
    ///
    /// Defaults to no components.
    pub fn components(
        mut self,
        components: impl IntoIterator<Item = impl Into<Component>>,
    ) -> Self {
        self.0.components = Some(components.into_iter().map(Into::into).collect());

        self
    }

    /// Sets the content.
    ///
    /// Defaults to no content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = Some(content.into());

        self
    }

    /// Sets the embeds.
    ///
    /// Defaults to no embeds.
    pub fn embeds(mut self, embeds: impl IntoIterator<Item = Embed>) -> Self {
        self.0.embeds = Some(FromIterator::from_iter(embeds));

        self
    }

    /// Sets the message flags.
    ///
    /// Defaults to no flags.
    pub fn flags(mut self, flags: MessageFlags) -> Self {
        self.0.flags = Some(flags);

        self
    }

    /// Sets whether TTS is used.
    ///
    /// Defaults to `false`.
    pub fn tts(mut self, tts: bool) -> Self {
        self.0.tts = Some(tts);

        self
    }

    /// Sets the poll.
    ///
    /// Defaults to no poll.
    pub fn poll(mut self, poll: Poll) -> Self {
        self.0.poll = Some(poll);

        self
    }
}

impl From<ChannelMessageBuilder> for InteractionResponse {
    fn from(builder: ChannelMessageBuilder) -> Self {
        builder.build()
    }
}

/// Builder for a [`InteractionResponse`] of type [`InteractionResponseType::Modal`].
#[derive(Clone, Debug)]
#[must_use = "builders have no effect if unused"]
pub struct ModalBuilder(InteractionResponseData);

impl ModalBuilder {
    /// Creates a new default builder.
    pub fn new(
        custom_id: impl Into<String>,
        title: impl Into<String>,
        components: impl IntoIterator<Item = impl Into<Component>>,
    ) -> Self {
        Self(InteractionResponseData {
            components: Some(components.into_iter().map(Into::into).collect()),
            custom_id: Some(custom_id.into()),
            title: Some(title.into()),
            ..Default::default()
        })
    }

    /// Builds the [`InteractionResponse`].
    pub fn build(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::Modal,
            data: Some(self.0),
        }
    }
}

impl From<ModalBuilder> for InteractionResponse {
    fn from(builder: ModalBuilder) -> Self {
        builder.build()
    }
}

/// Builder for a [`InteractionResponse`] of type [`InteractionResponseType::UpdateMessage`].
#[derive(Clone, Debug, Default)]
#[must_use = "builders have no effect if unused"]
pub struct UpdateMessageBuilder(InteractionResponseData);

impl UpdateMessageBuilder {
    /// Creates a new default builder.
    pub fn new() -> Self {
        Self(InteractionResponseData::default())
    }

    /// Builds the [`InteractionResponse`].
    pub fn build(self) -> InteractionResponse {
        InteractionResponse {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(self.0),
        }
    }

    /// Sets the allowed mentions filter.
    ///
    /// Defaults to no filter.
    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);

        self
    }

    /// Sets the attachments.
    ///
    /// Defaults to no attachments.
    pub fn attachments(mut self, attachments: impl IntoIterator<Item = Attachment>) -> Self {
        self.0.attachments = Some(FromIterator::from_iter(attachments));

        self
    }

    /// Sets the components.
    ///
    /// Defaults to no components.
    pub fn components(
        mut self,
        components: impl IntoIterator<Item = impl Into<Component>>,
    ) -> Self {
        self.0.components = Some(components.into_iter().map(Into::into).collect());

        self
    }

    /// Sets the content.
    ///
    /// Defaults to no content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = Some(content.into());

        self
    }

    /// Sets the embeds.
    ///
    /// Defaults to no embeds.
    pub fn embeds(mut self, embeds: impl IntoIterator<Item = Embed>) -> Self {
        self.0.embeds = Some(FromIterator::from_iter(embeds));

        self
    }

    /// Sets the message flags.
    ///
    /// Defaults to no flags.
    pub fn flags(mut self, flags: MessageFlags) -> Self {
        self.0.flags = Some(flags);

        self
    }

    /// Sets whether TTS is used.
    ///
    /// Defaults to `false`.
    pub fn tts(mut self, tts: bool) -> Self {
        self.0.tts = Some(tts);

        self
    }

    /// Sets the poll.
    ///
    /// Defaults to no poll.
    pub fn poll(mut self, poll: Poll) -> Self {
        self.0.poll = Some(poll);

        self
    }
}

impl From<UpdateMessageBuilder> for InteractionResponse {
    fn from(builder: UpdateMessageBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(AutocompleteBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(ChannelMessageBuilder: Clone, Debug, Default, Send, Sync);
    assert_impl_all!(ModalBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(UpdateMessageBuilder: Clone, Debug, Default, Send, Sync);
}
