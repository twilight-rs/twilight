use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ApiError {
    General(GeneralApiError),
    /// Request has been ratelimited.
    Ratelimited(RatelimitedApiError),
    /// Something was wrong with the input when sending a message.
    Message(MessageApiError),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::General(inner) => Display::fmt(inner, f),
            Self::Message(inner) => Display::fmt(inner, f),
            Self::Ratelimited(inner) => Display::fmt(inner, f),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct GeneralApiError {
    pub code: u64,
    pub message: String,
}

impl Display for GeneralApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Error code ")?;
        Display::fmt(&self.code, f)?;
        f.write_str(": ")?;

        f.write_str(&self.message)
    }
}

/// Sending a message failed because the provided fields contained invalid
/// input.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct MessageApiError {
    /// Fields within a provided embed were invalid.
    pub embed: Option<Vec<MessageApiErrorEmbedField>>,
}

impl Display for MessageApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("message fields invalid: ")?;

        if let Some(embed) = &self.embed {
            f.write_str("embed (")?;

            let field_count = embed.len().saturating_sub(1);

            for (idx, field) in embed.iter().enumerate() {
                Display::fmt(field, f)?;

                if idx == field_count {
                    f.write_str(", ")?;
                }
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

/// Field within a [`MessageApiError`] [embed] list.
///
/// [embed]: MessageApiError::embed
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum MessageApiErrorEmbedField {
    /// Something was wrong with the provided fields.
    Fields,
    /// The provided timestamp wasn't a valid RFC3339 string.
    Timestamp,
}

impl Display for MessageApiErrorEmbedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            Self::Fields => "fields",
            Self::Timestamp => "timestamp",
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct RatelimitedApiError {
    /// Whether the ratelimit is a global ratelimit.
    pub global: bool,
    /// Human readable message provided by the API.
    pub message: String,
    /// Amount of time to wait before retrying.
    pub retry_after: f64,
}

impl Display for RatelimitedApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Got ")?;

        if self.global {
            f.write_str("global ")?;
        }

        f.write_str("ratelimited for ")?;
        Display::fmt(&self.retry_after, f)?;

        f.write_str("s")
    }
}

impl Eq for RatelimitedApiError {}

impl PartialEq for RatelimitedApiError {
    fn eq(&self, other: &Self) -> bool {
        self.global == other.global && self.message == other.message
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApiError, GeneralApiError, MessageApiError, MessageApiErrorEmbedField, RatelimitedApiError,
    };
    use serde_test::Token;

    #[test]
    fn api_error_deser() {
        let expected = GeneralApiError {
            code: 10001,
            message: "Unknown account".to_owned(),
        };

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GeneralApiError",
                    len: 2,
                },
                Token::Str("code"),
                Token::U64(10001),
                Token::Str("message"),
                Token::Str("Unknown account"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn api_error_message() {
        let expected = ApiError::Message(MessageApiError {
            embed: Some(
                [
                    MessageApiErrorEmbedField::Fields,
                    MessageApiErrorEmbedField::Timestamp,
                ]
                .to_vec(),
            ),
        });

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "MessageApiError",
                    len: 1,
                },
                Token::Str("embed"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::UnitVariant {
                    name: "MessageApiErrorEmbedField",
                    variant: "fields",
                },
                Token::UnitVariant {
                    name: "MessageApiErrorEmbedField",
                    variant: "timestamp",
                },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn ratelimited_api_error() {
        let expected = RatelimitedApiError {
            global: true,
            message: "You are being rate limited.".to_owned(),
            retry_after: 6.457,
        };

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "RatelimitedApiError",
                    len: 3,
                },
                Token::Str("global"),
                Token::Bool(true),
                Token::Str("message"),
                Token::Str("You are being rate limited."),
                Token::Str("retry_after"),
                Token::F64(6.457),
                Token::StructEnd,
            ],
        );
    }

    /// Assert that deserializing an [`ApiError::Ratelimited`] variant uses
    /// the correct variant.
    ///
    /// Tests for [#1302], which was due to a previously ordered variant having
    /// higher priority for untagged deserialization.
    ///
    /// [#1302]: https://github.com/twilight-rs/twilight/issues/1302
    #[test]
    fn api_error_variant_ratelimited() {
        let expected = ApiError::Ratelimited(RatelimitedApiError {
            global: false,
            message: "You are being rate limited.".to_owned(),
            retry_after: 0.362,
        });

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "RatelimitedApiError",
                    len: 3,
                },
                Token::Str("global"),
                Token::Bool(false),
                Token::Str("message"),
                Token::Str("You are being rate limited."),
                Token::Str("retry_after"),
                Token::F64(0.362),
                Token::StructEnd,
            ],
        );
    }
}
