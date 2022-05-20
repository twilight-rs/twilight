//! Module containing all the markers used to specify validation
//! rules.

/// Module containing validation markers for messages.
pub mod message {
    use twilight_model::{
        application::component::Component,
        channel::embed::Embed,
        http::attachment::Attachment,
        id::{marker::StickerMarker, Id},
    };

    use crate::{
        message::{
            attachment_filename, components, content, embeds, sticker_ids, MessageValidationError,
        },
        Validate, Validated,
    };

    /// Marker for `CreateMessage::attachments`.
    pub struct AttachmentMarker;

    impl<'a> Validate<AttachmentMarker> for &'a [Attachment] {
        type Error = MessageValidationError;

        fn validate(self) -> Result<crate::Validated<Self, AttachmentMarker>, Self::Error> {
            self.iter()
                .try_for_each(|attachment| attachment_filename(&attachment.filename))?;

            Ok(Validated::new(self))
        }
    }

    /// Marker for `CreateMessage::components`.
    pub struct ComponentsMarker;

    impl<'a> Validate<ComponentsMarker> for &'a [Component] {
        type Error = MessageValidationError;

        fn validate(self) -> Result<Validated<Self, ComponentsMarker>, Self::Error> {
            components(self)?;

            Ok(Validated::new(self))
        }
    }

    /// Marker for `CreateMessage::content`.
    pub struct ContentMarker;

    impl<'a> Validate<ContentMarker> for &'a str {
        type Error = MessageValidationError;

        fn validate(self) -> Result<Validated<Self, ContentMarker>, Self::Error> {
            content(self)?;

            Ok(Validated::new(self))
        }
    }

    /// Marker for `CreateMessage::embeds`.
    pub struct EmbedsMarker;

    impl<'a> Validate<EmbedsMarker> for &'a [Embed] {
        type Error = MessageValidationError;

        fn validate(self) -> Result<Validated<Self, EmbedsMarker>, Self::Error> {
            embeds(self)?;

            Ok(Validated::new(self))
        }
    }

    /// Marker for `CreateMessage::sticker_ids`.
    pub struct StickerIdsMarker;

    impl<'a> Validate<StickerIdsMarker> for &'a [Id<StickerMarker>] {
        type Error = MessageValidationError;

        fn validate(self) -> Result<Validated<Self, StickerIdsMarker>, Self::Error> {
            sticker_ids(self)?;

            Ok(Validated::new(self))
        }
    }
}
