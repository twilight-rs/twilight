//! Utilities for efficiently parsing and representing data from Discord's API.

pub mod datetime;
pub mod image_hash;

pub(crate) mod known_string;

pub use self::{datetime::Timestamp, image_hash::ImageHash};

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn is_false(value: &bool) -> bool {
    !value
}

macro_rules! impl_typed {
    ($type: ty, u8) => {
        impl_typed!($type, integer, u8);
    };
    ($type: ty, u16) => {
        impl_typed!($type, integer, u16);
    };
    ($type: ty, integer, $raw: ty) => {
        impl $type {
            /// Create a new value from a dynamic raw value.
            ///
            /// The provided value isn't validated. Known valid values are
            /// associated constants on this type.
            pub const fn new(raw_value: $raw) -> Self {
                Self(raw_value)
            }

            /// Retrieve the raw value.
            pub const fn get(&self) -> $raw {
                self.0
            }
        }

        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(name) = self.name() {
                    f.debug_struct(stringify!($type))
                        .field("name", &name)
                        .field("value", &self.0)
                        .finish()
                } else {
                    f.debug_tuple(stringify!($type)).field(&self.0).finish()
                }
            }
        }

        impl From<$raw> for $type {
            fn from(value: $raw) -> Self {
                Self(value)
            }
        }

        impl From<$type> for $raw {
            fn from(value: $type) -> Self {
                value.get()
            }
        }
    };
    ($type: ty, String) => {
        impl $type {
            /// Create a mention type from a dynamic value.
            ///
            /// The provided mention type must be 64 bytes or smaller.
            pub fn new(mention_type: &str) -> Option<Self> {
                $crate::util::known_string::KnownString::from_str(mention_type).map(Self)
            }

            /// Get the value of the mention type.
            ///
            /// # Panics
            ///
            /// Panics if the mention type isn't valid UTF-8.
            pub fn get(&self) -> &str {
                self.0.get()
            }

            /// Create a event type from a set of bytes.
            const fn from_bytes(input: &[u8]) -> Self {
                Self(KnownString::from_bytes(input))
            }
        }

        impl AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                self.get()
            }
        }

        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.name().unwrap_or_else(|| self.get()))
            }
        }

        impl std::ops::Deref for $type {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }

        impl std::str::FromStr for $type {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl ToString for $type {
            fn to_string(&self) -> String {
                KnownString::to_string(&self.0)
            }
        }

        impl TryFrom<&str> for $type {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value).ok_or(())
            }
        }
    };
}
