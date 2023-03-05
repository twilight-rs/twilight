use super::Component;

/// Non-interactive [`Component`] container of other (non action row) components.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))
)]
pub struct ActionRow {
    /// List of components in the action row.
    #[cfg_attr(feature = "rkyv", omit_bounds)]
    pub components: Vec<Component>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(ActionRow: components);
    assert_impl_all!(ActionRow: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);
}
