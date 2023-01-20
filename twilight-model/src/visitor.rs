use serde::de::{Error as DeError, Visitor};
use std::{
    fmt::{Formatter, Result as FmtResult},
    marker::PhantomData,
};

/// Deserializers for optional nullable fields.
///
/// Some booleans in the Discord API are null when true, and not present when
/// false. `serde` doesn't have a way of natively handling this, so we need some
/// custom (de)serialization magic. [`RoleTags`] in particular has these fields.
///
/// [`RoleTags`]: crate::guild::RoleTags
pub mod null_boolean {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::fmt::{Formatter, Result as FmtResult};

    struct NullBooleanVisitor;

    impl<'de> Visitor<'de> for NullBooleanVisitor {
        type Value = bool;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("null")
        }

        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }

        // `visit_none` is used by `serde_json` when a present `null` value is
        // encountered, but other implementations - such as `simd_json` - may
        // use `visit_unit` instead.
        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }
    }

    // Clippy will say this bool can be taken by value, but we need it to be
    // passed by reference because that's what serde does.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_option(NullBooleanVisitor)
    }
}

/// (De)serializers for IDs that can be "zero", parsing as None in the case of
/// being zero.
///
/// This is a bug on Discord's end, but has been rather consistent for some
/// model fields such as [`ForumTag::emoji_id`].
///
/// [`ForumTag::emoji_id`]: crate::channel::forum::ForumTag
pub mod zeroable_id {
    use crate::id::Id;
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
        Deserialize,
    };
    use std::{
        fmt::{Formatter, Result as FmtResult},
        marker::PhantomData,
        str::FromStr,
    };

    struct ZeroableIdVisitor<T> {
        phantom: PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for ZeroableIdVisitor<T> {
        type Value = Option<Id<T>>;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str(r#"ID, 0, "0", or null"#)
        }

        fn visit_newtype_struct<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            let stringified_number = String::deserialize(deserializer)?;

            self.visit_str(&stringified_number)
        }

        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            deserializer.deserialize_any(self)
        }

        fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
            Id::from_str(v).map(Some).map_err(DeError::custom)
        }

        fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Id::new_checked(v))
        }

        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    // Clippy will say this bool can be taken by value, but we need it to be
    // passed by reference because that's what serde does.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer, T>(
        value: &Option<Id<T>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if let Some(id) = value {
            serializer.serialize_some(id)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>, T>(
        deserializer: D,
    ) -> Result<Option<Id<T>>, D::Error> {
        deserializer.deserialize_any(ZeroableIdVisitor::<T> {
            phantom: PhantomData,
        })
    }
}

pub struct U16EnumVisitor<'a> {
    description: &'a str,
    phantom: PhantomData<u16>,
}

impl<'a> U16EnumVisitor<'a> {
    pub const fn new(description: &'a str) -> Self {
        Self {
            description,
            phantom: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for U16EnumVisitor<'_> {
    type Value = u16;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.description)
    }

    fn visit_u16<E: DeError>(self, value: u16) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        let smaller = u16::try_from(value).map_err(E::custom)?;

        self.visit_u16(smaller)
    }
}
