use serde::{Deserialize, Serialize};

use crate::id::{
    Id,
    marker::{ApplicationMarker, SkuMarker},
};

use super::{SkuFlags, SkuType};

/// SKUs (stock-keeping units) in Discord represent premium offerings that can be made available to your application's users or guilds.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    /// ID of the parent application.
    application_id: Id<ApplicationMarker>,
    /// Flags for the SKU.
    flags: SkuFlags,
    /// ID of SKU.
    id: Id<SkuMarker>,
    /// Type of SKU.
    #[serde(rename = "type")]
    kind: SkuType,
    /// Customer-facing name of your premium offering.
    name: String,
    /// System-generated URL slug based on the SKU's name.
    slug: String,
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use crate::{
        application::monetization::{SkuFlags, SkuType},
        id::Id,
    };

    use super::Sku;

    #[test]
    fn sku() {
        let value = Sku {
            application_id: Id::new(1),
            flags: SkuFlags::GUILD_SUBSCRIPTION,
            id: Id::new(2),
            kind: SkuType::Subscription,
            name: "a name".to_owned(),
            slug: "a-slug".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Sku",
                    len: 6,
                },
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("flags"),
                Token::U64(1 << 7),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(5),
                Token::Str("name"),
                Token::Str("a name"),
                Token::Str("slug"),
                Token::Str("a-slug"),
                Token::StructEnd,
            ],
        );
    }
}
