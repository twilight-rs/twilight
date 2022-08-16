//! Representations of activity linked or textual buttons.

use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

/// Button used in an activity.
///
/// # serde
///
/// Activity buttons with a URL deserialize and serialize as a struct:
///
/// ```
/// use twilight_model::gateway::presence::activity_button::{ActivityButton, ActivityButtonLink};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// const JSON: &str = r#"{
///     "label": "a",
///     "url": "b"
/// }"#;
///
/// assert_eq!(
///     ActivityButton::Link(ActivityButtonLink {
///         label: "a".to_owned(),
///         url: "b".to_owned(),
///     }),
///     serde_json::from_str(JSON)?,
/// );
/// # Ok(()) }
/// ```
///
/// An activity button without a URL - an [`ActivityButtonText`] - will
/// deserialize and serialize as a string. This means that a textual activity
/// button with a label of "test" will serialize as simply the string "test" and
/// vice versa.
///
/// ```
/// use twilight_model::gateway::presence::activity_button::{ActivityButton, ActivityButtonText};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(
///     ActivityButton::Text(ActivityButtonText {
///         label: "test".to_owned(),
///     }),
///     serde_json::from_str(r#""test""#)?,
/// );
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ActivityButton {
    /// Activity button is a link.
    Link(ActivityButtonLink),
    /// Activity button is textual.
    Text(ActivityButtonText),
    /// Variant value is unknown to the library.
    Unknown,
}

impl ActivityButton {
    /// Whether the variant is a link button.
    pub const fn is_link(&self) -> bool {
        matches!(self, Self::Link(_))
    }

    /// Whether the variant is a text button.
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Retrieve an immutable reference to the label.
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Link(link) => Some(&link.label),
            Self::Text(text) => Some(&text.label),
            Self::Unknown => None,
        }
    }

    /// Retrieve an immutable reference to the URL if this is a link activity
    /// button.
    pub fn url(&self) -> Option<&str> {
        if let Self::Link(link) = self {
            Some(&link.url)
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ActivityButtonField {
    Label,
    Url,
}

struct ActivityButtonVisitor;

impl<'de> Visitor<'de> for ActivityButtonVisitor {
    type Value = ActivityButton;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("activity button struct or string")
    }

    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        Ok(ActivityButton::Text(ActivityButtonText { label: v }))
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        Ok(ActivityButton::Text(ActivityButtonText {
            label: v.to_owned(),
        }))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut label = None;
        let mut url = None;

        let span = tracing::trace_span!("deserializing activity button");
        let _span_enter = span.enter();

        loop {
            let span_child = tracing::trace_span!("iterating over element");
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {why:?}");

                    continue;
                }
            };

            match key {
                ActivityButtonField::Label => {
                    if label.is_some() {
                        return Err(DeError::duplicate_field("label"));
                    }

                    label = Some(map.next_value()?);
                }
                ActivityButtonField::Url => {
                    if url.is_some() {
                        return Err(DeError::duplicate_field("url"));
                    }

                    url = Some(map.next_value()?);
                }
            }
        }

        let label = label.ok_or_else(|| DeError::missing_field("label"))?;
        let url = url.ok_or_else(|| DeError::missing_field("url"))?;

        tracing::trace!(
            %label,
            ?url,
        );

        Ok(ActivityButton::Link(ActivityButtonLink { label, url }))
    }
}

impl<'de> Deserialize<'de> for ActivityButton {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ActivityButtonVisitor)
    }
}

impl Serialize for ActivityButton {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Link(link) => {
                let mut state = serializer.serialize_struct("ActivityButton", 2)?;

                state.serialize_field("label", &link.label)?;
                state.serialize_field("url", &link.url)?;

                state.end()
            }
            Self::Text(text) => serializer.serialize_str(&text.label),
            Self::Unknown => Err(serde::ser::Error::custom(
                "Can't serialize an unknown activity button type",
            )),
        }
    }
}

/// Button used in an activity with a URL.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityButtonLink {
    /// Text shown on the button.
    pub label: String,
    /// URL opened when clicking the button.
    pub url: String,
}

/// Button used in an activity without a URL.
///
/// # serde
///
/// Textual activity buttons deserialize and serialize as a string. This means
/// that a textual activity button with a label of "test" will serialize as
/// simply the string "test" and vice versa.
///
/// ```ignore
/// use twilight_model::gateway::presence::activity_button::ActivityButtonText;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(
///     ActivityButtonText { label: "test".to_owned() },
///     serde_json::from_str(r#""test""#)?,
/// );
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivityButtonText {
    /// Text shown on the button.
    pub label: String,
}
#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ActivityButtonTextField {
    Label,
}

struct ActivityButtonTextVisitor;

impl<'de> Visitor<'de> for ActivityButtonTextVisitor {
    type Value = ActivityButtonText;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("activity button string")
    }

    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        Ok(ActivityButtonText { label: v })
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        Ok(ActivityButtonText {
            label: v.to_owned(),
        })
    }
}

impl<'de> Deserialize<'de> for ActivityButtonText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ActivityButtonTextVisitor)
    }
}

impl Serialize for ActivityButtonText {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.label)
    }
}

#[cfg(test)]
mod tests {
    use super::{ActivityButton, ActivityButtonLink, ActivityButtonText};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(ActivityButtonLink: label, url);
    assert_impl_all!(
        ActivityButtonLink: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_fields!(ActivityButtonText: label);
    assert_impl_all!(
        ActivityButtonText: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        ActivityButton: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );

    fn link() -> ActivityButtonLink {
        ActivityButtonLink {
            label: "a".to_owned(),
            url: "b".to_owned(),
        }
    }

    fn text() -> ActivityButtonText {
        ActivityButtonText {
            label: "a".to_owned(),
        }
    }

    #[test]
    fn activity_button_link() {
        serde_test::assert_de_tokens(
            &link(),
            &[
                Token::Struct {
                    name: "ActivityButtonLink",
                    len: 2,
                },
                Token::Str("label"),
                Token::Str("a"),
                Token::Str("url"),
                Token::Str("b"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn activity_button_text() {
        serde_test::assert_de_tokens(&text(), &[Token::Str("a")]);
    }

    #[test]
    fn activity_button_with_url() {
        serde_test::assert_tokens(
            &ActivityButton::Link(link()),
            &[
                Token::Struct {
                    name: "ActivityButton",
                    len: 2,
                },
                Token::Str("label"),
                Token::Str("a"),
                Token::Str("url"),
                Token::Str("b"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn activity_button_without_url() {
        serde_test::assert_tokens(&ActivityButton::Text(text()), &[Token::Str("a")]);
    }
}
