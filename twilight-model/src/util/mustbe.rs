//! A struct that only deserializes from one specific boolean value.
//!
//! This module is heavily based upon
//! <https://github.com/dtolnay/monostate>.

use std::{
    fmt::{self, Debug},
    hash::Hash,
};

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Serialize,
};

/// Struct that will only serialize from the bool specified as `T`.
#[derive(Clone, Copy, Default)]
pub struct MustBeBool<const T: bool>;

impl<const T: bool> MustBeBool<T> {
    /// Get the expected boolean
    pub const fn get(self) -> bool {
        T
    }
}

impl<const T: bool, const U: bool> PartialEq<MustBeBool<U>> for MustBeBool<T> {
    fn eq(&self, _: &MustBeBool<U>) -> bool {
        T.eq(&U)
    }
}

impl<const T: bool> Eq for MustBeBool<T> {}

impl<const T: bool> Debug for MustBeBool<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MustBeBool").field(&T).finish()
    }
}

impl<const T: bool> Hash for MustBeBool<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        T.hash(state)
    }
}

impl<'de, const T: bool> Deserialize<'de> for MustBeBool<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MustBeBoolVisitor(bool);

        impl<'de> Visitor<'de> for MustBeBoolVisitor {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "boolean `{}`", self.0)
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if v == self.0 {
                    Ok(())
                } else {
                    Err(E::invalid_value(Unexpected::Bool(v), &self))
                }
            }
        }

        deserializer
            .deserialize_any(MustBeBoolVisitor(T))
            .map(|()| MustBeBool)
    }
}

impl<const T: bool> Serialize for MustBeBool<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(T)
    }
}

#[cfg(test)]
mod tests {
    use super::MustBeBool;

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct MTrue {
        m: MustBeBool<true>,
    }

    #[derive(Deserialize, Serialize)]
    struct MFalse {
        m: MustBeBool<false>,
    }

    #[derive(Deserialize, Serialize)]
    #[serde(untagged)]
    enum TestEnum {
        VariantTrue(MTrue),
        VariantFalse(MFalse),
    }

    #[test]
    #[allow(unused)]
    fn true_false_enum() {
        let json_1 = r#"{ "m": false }"#;
        let result_1 = serde_json::from_str::<TestEnum>(json_1).unwrap();
        assert!(matches!(result_1, TestEnum::VariantFalse(_)));

        let json_2 = r#"{ "m": true }"#;
        let result_2 = serde_json::from_str::<TestEnum>(json_2).unwrap();
        assert!(matches!(result_2, TestEnum::VariantTrue(_)));
    }

    #[test]
    fn default_value() {
        #[derive(Deserialize, Serialize)]
        struct MFalse {
            #[serde(default)]
            m: MustBeBool<false>,
        }

        let json_1 = r#"{}"#;
        serde_json::from_str::<MFalse>(json_1).unwrap();
    }

    #[test]
    fn ser() {
        let val = TestEnum::VariantTrue(MTrue { m: MustBeBool });
        let result = serde_json::to_string(&val).unwrap();
        assert_eq!(r#"{"m":true}"#, result);

        let val = TestEnum::VariantFalse(MFalse { m: MustBeBool });
        let result = serde_json::to_string(&val).unwrap();
        assert_eq!(r#"{"m":false}"#, result);
    }

    #[test]
    fn equality() {
        assert_ne!(MustBeBool::<false>, MustBeBool::<true>);
        assert_eq!(MustBeBool::<false>, MustBeBool::<false>);
        assert_eq!(MustBeBool::<true>, MustBeBool::<true>);
    }
}
