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
/// [`EmbedBuilder::field`]: struct.EmbedBuilder.html#method.field
/// [`inline`]: #method.inline
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed field"]
pub struct EmbedFieldBuilder(EmbedField);

impl EmbedFieldBuilder {
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

        if name.chars().count() > 256 {
            return Err(EmbedFieldError::NameTooLong { name, value });
        }

        if value.is_empty() {
            return Err(EmbedFieldError::ValueEmpty { name, value });
        }

        if value.chars().count() > 1024 {
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
