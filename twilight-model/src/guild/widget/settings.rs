use crate::id::{marker::ChannelMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetSettings {
    pub channel_id: Id<ChannelMarker>,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::GuildWidgetSettings;
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(GuildWidgetSettings: channel_id, enabled);
    assert_impl_all!(
        GuildWidgetSettings: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn guild_widget_settings() {
        let value = GuildWidgetSettings {
            channel_id: Id::new(111_111_111_111_111_111),
            enabled: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildWidgetSettings",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("111111111111111111"),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
