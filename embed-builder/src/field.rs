//! Create embed fields.

use twilight_model::channel::embed::EmbedField;

/// Create an embed field with a builder.
///
/// This can be passed into [`EmbedBuilder::field`].
///
/// Fields are not inlined by default. Use [`inline`] to inline a field.
///
/// [`EmbedBuilder::field`]: crate::EmbedBuilder::field
/// [`inline`]: Self::inline
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an embed field"]
pub struct EmbedFieldBuilder(EmbedField);

impl EmbedFieldBuilder {
    /// Create a new default embed field builder.
    ///
    /// Refer to [`EmbedBuilder::FIELD_NAME_LENGTH_LIMIT`] for the maximum
    /// number of UTF-16 code points that can be in a field name.
    ///
    /// Refer to [`EmbedBuilder::FIELD_VALUE_LENGTH_LIMIT`] for the maximum
    /// number of UTF-16 code points that can be in a field value.
    ///
    /// [`EmbedBuilder::FIELD_NAME_LENGTH_LIMIT`]: crate::EmbedBuilder::FIELD_NAME_LENGTH_LIMIT
    /// [`EmbedBuilder::FIELD_VALUE_LENGTH_LIMIT`]: crate::EmbedBuilder::FIELD_VALUE_LENGTH_LIMIT
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self::_new(name.into(), value.into())
    }

    const fn _new(name: String, value: String) -> Self {
        Self(EmbedField {
            inline: false,
            name,
            value,
        })
    }

    /// Build into an embed field.
    #[allow(clippy::missing_const_for_fn)]
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
    /// ```
    /// use twilight_embed_builder::EmbedFieldBuilder;
    ///
    /// let field = EmbedFieldBuilder::new("twilight", "is cool")
    ///     .inline()
    ///     .build();
    /// ```
    pub const fn inline(mut self) -> Self {
        self.0.inline = true;

        self
    }
}

impl From<EmbedFieldBuilder> for EmbedField {
    /// Convert an embed field builder into an embed field.
    ///
    /// This is equivalent to calling [`EmbedFieldBuilder::build`].
    fn from(builder: EmbedFieldBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::EmbedFieldBuilder;
    use crate::{EmbedBuilder, EmbedErrorType};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::channel::embed::EmbedField;

    assert_impl_all!(EmbedFieldBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(EmbedField: From<EmbedFieldBuilder>);

    #[test]
    fn test_new_errors() {
        assert!(matches!(
            EmbedBuilder::new().field(EmbedFieldBuilder::new("", "a")).build().unwrap_err().kind(),
            EmbedErrorType::FieldNameEmpty { name, value }
            if name.is_empty() && value.len() == 1
        ));
        assert!(matches!(
            EmbedBuilder::new().field(EmbedFieldBuilder::new("a".repeat(257), "a")).build().unwrap_err().kind(),
            EmbedErrorType::FieldNameTooLong { name, value }
            if name.len() == 257 && value.len() == 1
        ));
        assert!(matches!(
            EmbedBuilder::new().field(EmbedFieldBuilder::new("a", "")).build().unwrap_err().kind(),
            EmbedErrorType::FieldValueEmpty { name, value }
            if name.len() == 1 && value.is_empty()
        ));
        assert!(matches!(
            EmbedBuilder::new().field(EmbedFieldBuilder::new("a", "a".repeat(1025))).build().unwrap_err().kind(),
            EmbedErrorType::FieldValueTooLong { name, value }
            if name.len() == 1 && value.len() == 1025
        ));
    }

    #[test]
    fn test_builder_inline() {
        let expected = EmbedField {
            inline: true,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };
        let actual = EmbedFieldBuilder::new("name", "value").inline().build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_builder_no_inline() {
        let expected = EmbedField {
            inline: false,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };
        let actual = EmbedFieldBuilder::new("name", "value").build();

        assert_eq!(actual, expected);
    }
}
