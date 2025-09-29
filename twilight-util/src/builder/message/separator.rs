use twilight_model::channel::message::component::{Separator, SeparatorSpacingSize};

/// Create a separator with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a separator"]
pub struct SeparatorBuilder(Separator);

impl SeparatorBuilder {
    /// Create a new separator builder.
    pub const fn new() -> Self {
        Self(Separator {
            id: None,
            divider: None,
            spacing: None,
        })
    }

    /// Set the identifier of this separator.
    pub const fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set whether this separator is a divider.
    pub const fn divider(mut self, divider: bool) -> Self {
        self.0.divider.replace(divider);

        self
    }

    /// Set the spacing of this separator.
    pub const fn spacing(mut self, spacing: SeparatorSpacingSize) -> Self {
        self.0.spacing.replace(spacing);

        self
    }

    /// Build into a separator.
    pub const fn build(self) -> Separator {
        self.0
    }
}

impl Default for SeparatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SeparatorBuilder> for Separator {
    fn from(builder: SeparatorBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(SeparatorBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Separator: From<SeparatorBuilder>);

    #[test]
    fn builder() {
        let expected = Separator {
            id: None,
            divider: None,
            spacing: None,
        };

        let actual = SeparatorBuilder::new().build();

        assert_eq!(actual, expected);
    }
}
