use crate::id::{Id, marker::ChannelMarker};
use serde::{Deserialize, Serialize};

/// Partial channel object that a webhook is following.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhookChannel {
    pub id: Id<ChannelMarker>,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::WebhookChannel;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(WebhookChannel: id, name);

    assert_impl_all!(
        WebhookChannel: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );
}
