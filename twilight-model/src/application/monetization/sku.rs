use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ApplicationMarker, SKUMarker},
    Id,
};

use super::{SKUFlags, SKUType};

/// SKUs (stock-keeping units) in Discord represent premium offerings that can be made available to your application's users or guilds.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SKU {
    /// ID of SKU.
    id: Id<SKUMarker>,
    /// Type of SKU.
    #[serde(rename = "type")]
    kind: SKUType,
    /// ID of the parent application.
    application_id: Id<ApplicationMarker>,
    /// Customer-facing name of your premium offering.
    name: String,
    /// System-generated URL slug based on the SKU's name.
    slug: String,
    /// Flags for the SKU.
    flags: SKUFlags,
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use crate::{
        application::monetization::{SKUFlags, SKUType},
        id::Id,
    };

    use super::SKU;

    #[test]
    fn sku() {
        serde_test::assert_tokens(
            &SKU {
                id: Id::new(1),
                kind: SKUType::Subscription,
                application_id: Id::new(2),
                name: "a name".to_owned(),
                slug: "a-slug".to_owned(),
                flags: SKUFlags::GUILD_SUBSCRIPTION,
            },
            &[
                Token::Struct {
                    name: "SKU",
                    len: 6,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("name"),
                Token::Str("a name"),
                Token::Str("slug"),
                Token::Str("a-slug"),
                Token::Str("flags"),
                Token::U64(1 << 7),
                Token::StructEnd,
            ],
        );
    }
}
