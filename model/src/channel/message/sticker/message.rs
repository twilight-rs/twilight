use super::{StickerFormatType, StickerId};
use serde::{Deserialize, Serialize};

/// Smallest amount of data required to render a sticker.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageSticker {
    pub format_type: StickerFormatType,
    pub id: StickerId,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{MessageSticker, StickerFormatType, StickerId};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(MessageSticker: format_type, id, name);

    assert_impl_all!(
        MessageSticker: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    #[test]
    fn test_full() {
        let value = MessageSticker {
            format_type: StickerFormatType::Lottie,
            id: StickerId::new(1).expect("non zero"),
            name: "sticker".into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageSticker",
                    len: 3,
                },
                Token::Str("format_type"),
                Token::U8(StickerFormatType::Lottie as u8),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker"),
                Token::StructEnd,
            ],
        );
    }
}
