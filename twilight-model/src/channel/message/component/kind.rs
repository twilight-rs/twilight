use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Type of [`Component`].
///
/// [`Component`]: super::Component
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ComponentType(u8);

impl ComponentType {
    /// Component is an [`ActionRow`].
    ///
    /// [`ActionRow`]: super::ActionRow
    pub const ACTION_ROW: Self = Self::new(1);

    /// Component is an [`Button`].
    ///
    /// [`Button`]: super::Button
    pub const BUTTON: Self = Self::new(2);

    /// Component is an [`SelectMenu`].
    ///
    /// [`SelectMenu`]: super::SelectMenu
    pub const SELECT_MENU: Self = Self::new(3);

    /// Component is an [`TextInput`].
    ///
    /// [`TextInput`]: super::TextInput
    pub const TEXT_INPUT: Self = Self::new(4);

    /// Create a new command type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`ACTION_ROW`][`Self::ACTION_ROW`].
    pub const fn new(command_type: u8) -> Self {
        Self(command_type)
    }

    /// Retrieve the value of the command type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::message::component::ComponentType;
    ///
    /// assert_eq!(2, ComponentType::BUTTON.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    ///
    /// # Examples
    ///
    /// Check the [`ACTION_ROW`] constant's name:
    ///
    /// ```
    /// use twilight_model::channel::message::component::ComponentType;
    ///
    /// assert_eq!(Some("ACTION_ROW"), ComponentType::ACTION_ROW.name());
    /// ```
    ///
    /// [`ACTION_ROW`]: Self::ACTION_ROW
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::ACTION_ROW => "ACTION_ROW",
            Self::BUTTON => "BUTTON",
            Self::SELECT_MENU => "SELECT_MENU",
            Self::TEXT_INPUT => "TEXT_INPUT",
            _ => return None,
        })
    }
}

impl Debug for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("ComponentType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("ComponentType").field(&self.0).finish()
        }
    }
}

impl From<u8> for ComponentType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ComponentType> for u8 {
    fn from(value: ComponentType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentType;
    use serde_test::Token;

    const MAP: &[(ComponentType, u8)] = &[
        (ComponentType::ACTION_ROW, 1),
        (ComponentType::BUTTON, 2),
        (ComponentType::SELECT_MENU, 3),
        (ComponentType::TEXT_INPUT, 4),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "ComponentType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, ComponentType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
