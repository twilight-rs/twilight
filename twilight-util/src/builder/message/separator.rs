use twilight_model::channel::message::component::{Separator, SeparatorSpacingSize};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a separator"]
pub struct SeparatorBuilder(Separator);

impl SeparatorBuilder {
    pub fn new() -> Self {
        Self(Separator {
            id: None,
            divider: None,
            spacing: None,
        })
    }

    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.0.divider.replace(divider);

        self
    }

    pub fn spacing(mut self, spacing: SeparatorSpacingSize) -> Self {
        self.0.spacing.replace(spacing.into());

        self
    }
}
