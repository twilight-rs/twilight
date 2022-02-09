use crate::{
    application::{command::CommandOptionChoice, component::Component},
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum CallbackDataEnvelope {
    Autocomplete(Autocomplete),
    Messages(CallbackData),
}

/// Optional extra data sent when responding to an [`Interaction`] of type
/// [`ApplicationCommand`].
///
/// This is used when intending to send a message in the response.
///
/// This struct has an [associated builder] in the [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/struct.CallbackDataBuilder.html
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

/// Response to an autocomplete [`Interaction`].
///
/// [`Interaction`]: crate::application::interaction::Interaction
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Autocomplete {
    /// List of autocomplete alternatives.
    pub choices: Vec<CommandOptionChoice>,
}

#[cfg(test)]
mod tests {
    use super::{Autocomplete, CallbackData};
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

    assert_fields!(Autocomplete: choices);

    assert_impl_all!(
        Autocomplete: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
}
