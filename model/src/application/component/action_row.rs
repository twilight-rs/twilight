use std::fmt::{Formatter, Result as FmtResult};

use super::{Component, ComponentType};
use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};

/// A non-interactive component that acts as a container for other components.
///
/// Refer to [Discord Docs/Message Components] for additional information.
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#action-rows
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActionRow {
    /// List of components in the action row.
    pub components: Vec<Component>,
}

impl Serialize for ActionRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let field_count = 1 + usize::from(!self.components.is_empty());
        let mut state = serializer.serialize_struct("ActionRow", field_count)?;

        if !self.components.is_empty() {
            state.serialize_field("components", &self.components)?;
        }

        state.serialize_field("type", &ComponentType::ActionRow)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for ActionRow {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ActionRowVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ActionRowField {
    Type,
    Components,
}

struct ActionRowVisitor;

impl<'de> Visitor<'de> for ActionRowVisitor {
    type Value = ActionRow;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct ActionRow")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut kind: Option<ComponentType> = None;
        let mut components: Option<Vec<Component>> = None;

        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("deserializing action row");
        let _span_enter = span.enter();

        loop {
            #[cfg(feature = "tracing")]
            let span_child = tracing::trace_span!("iterating over action row");
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    #[cfg(feature = "tracing")]
                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
            };

            match key {
                ActionRowField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    let value: ComponentType = map.next_value()?;

                    if value != ComponentType::ActionRow {
                        return Err(DeError::invalid_value(
                            Unexpected::Unsigned(value as u64),
                            &"an action row type",
                        ));
                    }

                    kind = Some(value)
                }
                ActionRowField::Components => {
                    if components.is_some() {
                        return Err(DeError::duplicate_field("components"));
                    }

                    components = Some(map.next_value()?);
                }
            }
        }

        if kind.is_none() {
            return Err(DeError::missing_field("kind"));
        }

        let components = components.ok_or_else(|| DeError::missing_field("components"))?;

        #[cfg(feature = "tracing")]
        tracing::trace!(?components, ?kind, "all fields of ActionRow exist");

        Ok(ActionRow { components })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::component::{button::ButtonStyle, Button, Component};

    use super::ActionRow;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(ActionRow: components);
    assert_impl_all!(
        ActionRow: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_action_row() {
        let value = ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some("button-1".to_owned()),
                disabled: false,
                emoji: None,
                style: ButtonStyle::Primary,
                label: Some("Button".to_owned()),
                url: None,
            })]),
        };

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActionRow",
                    len: 2,
                },
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Button",
                    len: 5,
                },
                Token::String("custom_id"),
                Token::Some,
                Token::String("button-1"),
                Token::String("disabled"),
                Token::Bool(false),
                Token::String("label"),
                Token::Some,
                Token::String("Button"),
                Token::String("style"),
                Token::U8(1),
                Token::String("type"),
                Token::U8(2),
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActionRow",
                    len: 2,
                },
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Button",
                    len: 5,
                },
                Token::String("custom_id"),
                Token::String("button-1"),
                Token::String("disabled"),
                Token::Bool(false),
                Token::String("label"),
                Token::String("Button"),
                Token::String("style"),
                Token::U8(1),
                Token::String("type"),
                Token::U8(2),
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }
}
