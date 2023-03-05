use crate::{
    id::{marker::GuildMarker, Id},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

/// Partial guild object that a webhook is following.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WebhookGuild {
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::WebhookGuild;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(WebhookGuild: icon, id, name);

    assert_impl_all!(
        WebhookGuild: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );
}
