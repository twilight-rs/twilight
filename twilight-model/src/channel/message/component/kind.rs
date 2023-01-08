use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type of [`Component`].
///
/// [`Component`]: super::Component
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

    /// Name of the component type.
    ///
    /// Variants have a name equivalent to the variant name itself.
    ///
    /// # Examples
    ///
    /// Check the [`ACTION_ROW`] variant's name:
    ///
    /// ```
    /// use twilight_model::channel::message::component::ComponentType;
    ///
    /// assert_eq!("ACTION_ROW", ComponentType::ACTION_ROW.name());
    /// ```
    ///
    /// [`ACTION_ROW`]: Self::ACTION_ROW
    pub const fn name(&self) -> &'static str {
        match *self {
            Self::ACTION_ROW => "ACTION_ROW",
            Self::BUTTON => "BUTTON",
            Self::SELECT_MENU => "SELECT_MENU",
            Self::TEXT_INPUT => "TEXT_INPUT",
            _ => "UNKNOWN",
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

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
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
