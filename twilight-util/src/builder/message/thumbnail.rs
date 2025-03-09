use twilight_model::channel::message::component::{Thumbnail, UnfurledMediaItem};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a thumbnail"]
pub struct ThumbnailBuilder(Thumbnail);

impl ThumbnailBuilder {
    pub fn new(media: UnfurledMediaItem) -> Self {
        Self(Thumbnail {
            id: None,
            media,
            description: None,
            spoiler: None,
        })
    }

    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    pub fn media(mut self, media: impl Into<UnfurledMediaItem>) -> Self {
        self.0.media = media.into();

        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description.replace(description.into());

        self
    }
}

impl From<Thumbnail> for ThumbnailBuilder {
    fn from(thumbnail: Thumbnail) -> Self {
        Self(thumbnail)
    }
}
