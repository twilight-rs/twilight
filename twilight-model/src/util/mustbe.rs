//! A struct that only deserializes from one specific boolean value.
//!
//! This module is heavily based upon
//! <https://github.com/dtolnay/monostate>.

use std::fmt;

use serde::{
    Deserialize,
    de::{Error, Unexpected, Visitor},
};

/// Struct that will only serialize from the bool specified as `T`.
pub struct MustBeBool<const T: bool>;

impl<'de, const T: bool> Deserialize<'de> for MustBeBool<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MustBeBoolVisitor(bool);

        impl Visitor<'_> for MustBeBoolVisitor {
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

#[cfg(test)]
mod tests {
    use super::MustBeBool;

    use serde::Deserialize;

    #[derive(Deserialize)]
    struct MTrue {
        #[allow(unused)]
        m: MustBeBool<true>,
    }

    #[derive(Deserialize)]
    struct MFalse {
        #[allow(unused)]
        m: MustBeBool<false>,
    }

    #[derive(Deserialize)]
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
}
