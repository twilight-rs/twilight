//! Create embed fields.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::embed::EmbedField;

/// Error creating an embed field.
///
/// This is returned from [`EmbedFieldBuilder::new`].
///
/// [`EmbedFieldBuilder::new`]: struct.EmbedFieldBuilder.html#method.new
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EmbedFieldError {
    /// Name is empty.
    NameEmpty {
        /// Provided name. Although empty, the same owned allocation is
        /// included.
        name: String,
        /// Provided value.
        value: String,
    },
    /// Name is longer than 256 UTF-16 code points.
    NameTooLong {
        /// Provided name.
        name: String,
        /// Provided value.
        value: String,
    },
    /// Value is empty.
    ValueEmpty {
        /// Provided name.
        name: String,
        /// Provided value. Although empty, the same owned allocation is
        /// included.
        value: String,
    },
    /// Value is longer than 1024 UTF-16 code points.
    ValueTooLong {
        /// Provided name.
        name: String,
        /// Provided value.
        value: String,
    },
}

impl Display for EmbedFieldError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameEmpty { .. } => f.write_str("the field name is empty"),
            Self::NameTooLong { .. } => f.write_str("the field name is too long"),
            Self::ValueEmpty { .. } => f.write_str("the field value is empty"),
            Self::ValueTooLong { .. } => f.write_str("the field value is too long"),
        }
    }
}

impl Error for EmbedFieldError {}

/// Create an embed field with a builder.
///
/// This can be passed into [`EmbedBuilder::field`].
///
/// Fields are not inlined by default. Use [`inline`] to inline a field.
///
/// [`EmbedBuilder::field`]: ../builder/struct.EmbedBuilder.html#method.field
/// [`inline`]: #method.inline
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed field"]
pub struct EmbedFieldBuilder(EmbedField);

impl EmbedFieldBuilder {
    /// The maximum number of UTF-16 code points that can be in a field name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: #method.new
    pub const NAME_LENGTH_LIMIT: usize = 256;

    /// The maximum number of UTF-16 code points that can be in a field value.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: #method.new
    pub const VALUE_LENGTH_LIMIT: usize = 1024;

    /// Create a new default embed field builder.
    ///
    /// The name is limited to 256 UTF-16 code points, and the value is limited
    /// to 1024.
    ///
    /// # Errors
    ///
    /// Returns [`EmbedFieldError::NameEmpty`] if the provided name is
    /// empty.
    ///
    /// Returns [`EmbedFieldError::NameTooLong`] if the provided name is
    /// longer than 256 UTF-16 code points.
    ///
    /// Returns [`EmbedFieldError::ValueEmpty`] if the provided value is
    /// empty.
    ///
    /// Returns [`EmbedFieldError::ValueTooLong`] if the provided value
    /// is longer than 1024 UTF-16 code points.
    ///
    /// [`EmbedFieldError::NameEmpty`]: enum.EmbedFieldError.html#variant.NameEmpty
    /// [`EmbedFieldError::NameTooLong`]: enum.EmbedFieldError.html#variant.NameTooLong
    /// [`EmbedFieldError::ValueEmpty`]: enum.EmbedFieldError.html#variant.ValueEmpty
    /// [`EmbedFieldError::ValueTooLong`]: enum.EmbedFieldError.html#variant.ValueTooLong
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Result<Self, EmbedFieldError> {
        Self::_new(name.into(), value.into())
    }

    fn _new(name: String, value: String) -> Result<Self, EmbedFieldError> {
        if name.is_empty() {
            return Err(EmbedFieldError::NameEmpty { name, value });
        }

        if name.chars().count() > Self::NAME_LENGTH_LIMIT {
            return Err(EmbedFieldError::NameTooLong { name, value });
        }

        if value.is_empty() {
            return Err(EmbedFieldError::ValueEmpty { name, value });
        }

        if value.chars().count() > Self::VALUE_LENGTH_LIMIT {
            return Err(EmbedFieldError::ValueTooLong { name, value });
        }

        Ok(Self(EmbedField {
            inline: false,
            name,
            value,
        }))
    }

    /// Build into an embed field.
    #[must_use = "should be used as part of an embed builder"]
    pub fn build(self) -> EmbedField {
        self.0
    }

    /// Inline the field.
    ///
    /// # Examples
    ///
    /// Create an inlined field:
    ///
    /// ```rust
    /// use twilight_embed_builder::EmbedFieldBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let field = EmbedFieldBuilder::new("twilight", "is cool")?
    ///     .inline()
    ///     .build();
    /// # Ok(()) }
    /// ```
    pub fn inline(mut self) -> Self {
        self.0.inline = true;

        self
    }
}

impl From<EmbedFieldBuilder> for EmbedField {
    /// Convert an embed field builder into an embed field.
    ///
    /// This is equivalent to calling [`EmbedFieldBuilder::build`].
    ///
    /// [`EmbedFieldBuilder::build`]: #method.build
    fn from(builder: EmbedFieldBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{EmbedFieldBuilder, EmbedFieldError};
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::{error::Error, fmt::Debug};
    use twilight_model::channel::embed::EmbedField;

    assert_impl_all!(
        EmbedFieldError: Clone,
        Debug,
        Error,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_fields!(EmbedFieldError::NameEmpty: name, value);
    assert_fields!(EmbedFieldError::NameTooLong: name, value);
    assert_fields!(EmbedFieldError::ValueEmpty: name, value);
    assert_fields!(EmbedFieldError::ValueTooLong: name, value);
    assert_impl_all!(EmbedFieldBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    const_assert!(EmbedFieldBuilder::NAME_LENGTH_LIMIT == 256);
    const_assert!(EmbedFieldBuilder::VALUE_LENGTH_LIMIT == 1024);
    assert_impl_all!(EmbedField: From<EmbedFieldBuilder>);

    #[test]
    fn test_new_errors() {
        assert!(matches!(
            EmbedFieldBuilder::new("", "a"),
            Err(EmbedFieldError::NameEmpty { name, value })
            if name.is_empty() && value.len() == 1
        ));
        assert!(matches!(
            EmbedFieldBuilder::new("a".repeat(257), "a"),
            Err(EmbedFieldError::NameTooLong { name, value })
            if name.len() == 257 && value.len() == 1
        ));
        assert!(matches!(
            EmbedFieldBuilder::new("a", ""),
            Err(EmbedFieldError::ValueEmpty { name, value })
            if name.len() == 1 && value.is_empty()
        ));
        assert!(matches!(
            EmbedFieldBuilder::new("a", "a".repeat(1025)),
            Err(EmbedFieldError::ValueTooLong { name, value })
            if name.len() == 1 && value.len() == 1025
        ));
    }

    #[test]
    fn test_builder_inline() -> Result<(), Box<dyn Error>> {
        let expected = EmbedField {
            inline: true,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };
        let actual = EmbedFieldBuilder::new("name", "value")?.inline().build();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_builder_no_inline() -> Result<(), Box<dyn Error>> {
        let expected = EmbedField {
            inline: false,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };
        let actual = EmbedFieldBuilder::new("name", "value")?.build();

        assert_eq!(actual, expected);

        Ok(())
    }
}
