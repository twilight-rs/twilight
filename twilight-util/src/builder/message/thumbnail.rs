use twilight_model::channel::message::component::{Thumbnail, UnfurledMediaItem};

/// Create a thumbnail with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a thumbnail"]
pub struct ThumbnailBuilder(Thumbnail);

impl ThumbnailBuilder {
    /// Create a new thumbnail builder.
    pub fn new(media: impl Into<UnfurledMediaItem>) -> Self {
        Self(Thumbnail {
            id: None,
            media: media.into(),
            description: None,
            spoiler: None,
        })
    }

    /// Set the identifier of this thumbnail.
    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set the media of this thumbnail.
    pub fn media(mut self, media: impl Into<UnfurledMediaItem>) -> Self {
        self.0.media = media.into();

        self
    }

    /// Set the description of this thumbnail.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description.replace(Some(description.into()));

        self
    }

    /// Build into a thumbnail.
    pub fn build(self) -> Thumbnail {
        self.0
    }
}

impl From<ThumbnailBuilder> for Thumbnail {
    fn from(builder: ThumbnailBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ThumbnailBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Thumbnail: From<ThumbnailBuilder>);

    #[test]
    fn builder() {
        let url = "http://www.example.com/example.png";
        let expected = Thumbnail {
            id: None,
            media: UnfurledMediaItem {
                url: url.to_string(),
                proxy_url: None,
                height: None,
                width: None,
                content_type: None,
            },
            description: None,
            spoiler: None,
        };

        let actual = ThumbnailBuilder::new(url).build();

        assert_eq!(actual, expected);
    }
}
