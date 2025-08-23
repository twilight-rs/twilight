use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, marker::AvatarDecorationDataSkuMarker},
    util::ImageHash,
};

/// The data for the user's avatar decoration.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AvatarDecorationData {
    /// The avatar decoration hash.
    pub asset: ImageHash,
    /// ID of the avatar decoration's SKU.
    pub sku_id: Id<AvatarDecorationDataSkuMarker>,
}

#[cfg(test)]
mod tests {
    use super::{AvatarDecorationData, Id, ImageHash};
    use serde_test::Token;

    #[test]
    fn test_avatar_decoration_data() {
        let hash = "b2a6536641da91a0b59bd66557c56c36";
        let value = AvatarDecorationData {
            asset: ImageHash::parse(hash.as_bytes()).unwrap(),
            sku_id: Id::new(1),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AvatarDecorationData",
                    len: 2,
                },
                Token::Str("asset"),
                Token::Str(hash),
                Token::Str("sku_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
