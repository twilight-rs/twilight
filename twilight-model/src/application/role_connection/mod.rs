//! Application role connections models.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Application Role Connection Metadata Type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum MetadataType {
    /// the metadata value (integer) is less than or equal to the guild's
    /// configured value (integer)
    IntegerLessThanOrEqual = 1,
    /// the metadata value (integer) is greater than or equal to the guild's
    /// configured value (integer)
    IntegerGreaterThanOrEqual = 2,
    /// the metadata value (integer) is equal to the guild's configured value
    /// (integer)
    IntegerEqual = 3,
    /// the metadata value (integer) is not equal to the guild's configured
    /// value (integer)
    IntegerNotEqual = 4,
    /// the metadata value (ISO8601 string) is less than or equal to
    /// the guild's configured value (integer; days before current date)
    DatetimeLessThanOrEqual = 5,
    /// the metadata value (ISO8601 string) is greater than or equal to
    /// the guild's configured value (integer; days before current date)
    DatetimeGreaterThanOrEqual = 6,
    /// the metadata value (integer) is equal to the guild's configured value
    /// (integer; 1)
    BooleanEqual = 7,
    /// the metadata value (integer) is not equal to the guild's configured
    /// value (integer; 1)
    BooleanNotEqual = 8,
}

/// Application Role Connection Metadata Structure.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Metadata {
    /// type of metadata value
    pub r#type: MetadataType,
    /// dictionary key for the metadata field
    /// (must be a-z, 0-9, or _ characters; max 50 characters)
    pub key: String,
    /// name of the metadata field (max 100 characters)
    pub name: String,
    /// translations of the name
    pub name_localizations: HashMap<String, String>,
    /// description of the metadata field (max 200 characters)
    pub description: String,
    /// translations of the description
    pub description_localizations: HashMap<String, String>,
}
