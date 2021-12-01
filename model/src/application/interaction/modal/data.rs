use crate::application::component::ComponentType;
use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Formatter, Result as FmtResult};

/// Data received when an [`ModalSubmit`] interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [`ModalSubmit`]: crate::application::interaction::Interaction::ModalSubmit
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionData {
    /// User defined identifier for the input text.
    pub custom_id: String,
    /// List of parsed user inputs.
    pub components: Vec<ModalInteractionDataActionRow>,
}

/// The parsed [`ActionRow`] of the users input.
///
/// Refer to [the discord docs] for more information.
///
/// [`ActionRow`]: crate::application::interaction::modal::ModalInteractionDataActionRow
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ModalInteractionDataActionRow {
    /// The parsed components.
    pub components: Vec<ModalInteractionDataComponent>,
}

impl Serialize for ModalInteractionDataActionRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Reserved for `type`
        let len = 2;

        let mut state = serializer.serialize_struct("ModalInteractionDataActionRow", len)?;

        state.serialize_field("type", &ComponentType::ActionRow)?;
        state.serialize_field("components", &self.components)?;

        state.end()
    }
}

/// Data received when a user fills in a modal component.
///
/// Refer to [the discord docs] for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionDataComponent {
    pub custom_id: String,
    pub value: ModalComponentValue,
}

impl Serialize for ModalInteractionDataComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Reserved for `type`, `custom_id` and `value`
        let len = 3;

        let mut state = serializer.serialize_struct("ModalInteractionDataComponent", len)?;

        state.serialize_field("custom_id", &self.custom_id)?;
        state.serialize_field("type", &self.value.kind())?;

        match &self.value {
            ModalComponentValue::InputText(i) => state.serialize_field("value", i)?,
        }

        state.end()
    }
}

impl<'de> Deserialize<'de> for ModalInteractionDataComponent {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Fields {
            Type,
            CustomId,
            Value,
        }

        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum ValueEnvelope {
            String(String),
        }

        struct ModalInteractionDataComponentVisitor;

        impl<'de> Visitor<'de> for ModalInteractionDataComponentVisitor {
            type Value = ModalInteractionDataComponent;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                formatter.write_str("ModalInteractionDataComponent")
            }

            #[allow(clippy::too_many_lines)]
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut custom_id_opt = None;
                let mut kind_opt = None;
                let mut value_opt = None;

                loop {
                    let key = match map.next_key() {
                        Ok(Some(key)) => key,
                        Ok(None) => break,
                        Err(why) => {
                            map.next_value::<IgnoredAny>()?;

                            tracing::trace!("ran into an unknown key: {:?}", why);

                            continue;
                        }
                    };

                    match key {
                        Fields::CustomId => {
                            if custom_id_opt.is_some() {
                                return Err(DeError::duplicate_field("custom_id"));
                            }

                            custom_id_opt = Some(map.next_value()?);
                        }
                        Fields::Type => {
                            if kind_opt.is_some() {
                                return Err(DeError::duplicate_field("type"));
                            }

                            kind_opt = Some(map.next_value()?);
                        }
                        Fields::Value => {
                            if value_opt.is_some() {
                                return Err(DeError::duplicate_field("value"));
                            }

                            value_opt = Some(map.next_value()?);
                        }
                    }
                }

                let custom_id = custom_id_opt.ok_or_else(|| DeError::missing_field("custom_id"))?;
                let kind = kind_opt.ok_or_else(|| DeError::missing_field("type"))?;

                let value = match kind {
                    ComponentType::ActionRow => {
                        return Err(DeError::unknown_variant("ActionRow", &["InputText"]))
                    }
                    ComponentType::Button => {
                        return Err(DeError::unknown_variant("Button", &["InputText"]))
                    }
                    ComponentType::SelectMenu => {
                        return Err(DeError::unknown_variant("SelectMenu", &["InputText"]))
                    }
                    ComponentType::InputText => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        ModalComponentValue::InputText(val)
                    }
                };

                Ok(ModalInteractionDataComponent { custom_id, value })
            }
        }

        deserializer.deserialize_map(ModalInteractionDataComponentVisitor)
    }
}

/// Value of a [`ModalInteractionDataComponent`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModalComponentValue {
    InputText(String),
}

impl ModalComponentValue {
    pub const fn kind(&self) -> ComponentType {
        match self {
            ModalComponentValue::InputText(_) => ComponentType::InputText,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    use crate::application::component::ComponentType;

    use super::{
        ModalComponentValue, ModalInteractionData, ModalInteractionDataActionRow,
        ModalInteractionDataComponent,
    };

    assert_fields!(ModalInteractionData: custom_id, components);
    assert_impl_all!(
        ModalInteractionData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Serialize,
    );

    assert_fields!(ModalInteractionDataComponent: custom_id, value);
    assert_impl_all!(
        ModalInteractionDataComponent: Clone,
        Debug,
        Eq,
        PartialEq,
        Deserialize<'static>,
        Serialize
    );

    #[test]
    fn test_modal_data() {
        let value = ModalInteractionData {
            custom_id: "test-modal".to_owned(),
            components: Vec::from([ModalInteractionDataActionRow {
                components: Vec::from([
                ModalInteractionDataComponent {
                    custom_id: "the-data-id".to_owned(),
                    value: ModalComponentValue::InputText("Twilight is a powerful, flexible and scalable ecosystem of Rust libraries for the Discord API.".to_owned())
                }
            ])
        },  ])
        };

        serde_test::assert_tokens(&value, &[
            Token::Struct {
                name: "ModalInteractionData",
                len: 2
            },
            Token::String("custom_id"),
            Token::String("test-modal"),
            Token::String("components"),
            Token::Seq { len: Some(1) },
            Token::Struct { name: "ModalInteractionDataActionRow", len: 2},
            Token::String("type"),
            Token::U8(ComponentType::ActionRow as u8),
            Token::String("components"),
            Token::Seq { len: Some(1) },
            Token::Struct {
                name: "ModalInteractionDataComponent",
                len: 3
            },
            Token::String("custom_id"),
            Token::String("the-data-id"),
            Token::String("type"),
            Token::U8(ComponentType::InputText as u8),
            Token::String("value"),
            Token::String("Twilight is a powerful, flexible and scalable ecosystem of Rust libraries for the Discord API."),
            Token::StructEnd,
            Token::SeqEnd,
            Token::StructEnd,
            Token::SeqEnd,
            Token::StructEnd
        ]);
    }
}
