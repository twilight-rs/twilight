use twilight_model::channel::message::component::{Separator, SeparatorSpacingSize};

/// Create a separator with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a separator"]
pub struct SeparatorBuilder(Separator);

impl SeparatorBuilder {
    /// Create a new separator builder.
    pub fn new() -> Self {
        Self(Separator {
            id: None,
            divider: None,
            spacing: None,
        })
    }

    /// Set the identifier of this separator.
    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set whether this separator is a divider.
    pub fn divider(mut self, divider: bool) -> Self {
        self.0.divider.replace(divider);

        self
    }

    /// Set the spacing of this separator.
    pub fn spacing(mut self, spacing: SeparatorSpacingSize) -> Self {
        self.0.spacing.replace(spacing.into());

        self
    }

    /// Build into a separator.
    pub fn build(self) -> Separator {
        self.0
    }
}

impl From<SeparatorBuilder> for Separator {
    fn from(builder: SeparatorBuilder) -> Self {
        builder.build()
    }
}
