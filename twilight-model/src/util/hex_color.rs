use std::fmt::Formatter;
use std::fmt::{Display, Result as FmtResult};
use std::num::ParseIntError;
use std::str::FromStr;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a color in the RGB format using hexadecimal notation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HexColor(
    /// Red component of the color.
    pub u8,
    /// Green component of the color.
    pub u8,
    /// Blue component of the color.
    pub u8,
);

impl Display for HexColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2))
    }
}

impl Serialize for HexColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug)]
pub enum HexColorParseError {
    InvalidLength,
    InvalidFormat,
    InvalidCharacter(ParseIntError),
}

impl From<ParseIntError> for HexColorParseError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidCharacter(err)
    }
}

impl FromStr for HexColor {
    type Err = HexColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(HexColorParseError::InvalidFormat);
        }

        let s = s.trim_start_matches('#');

        let (r, g, b) = match s.len() {
            3 => (
                u8::from_str_radix(&s[0..1], 16)?,
                u8::from_str_radix(&s[1..2], 16)?,
                u8::from_str_radix(&s[2..3], 16)?,
            ),
            6 => (
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
            ),
            _ => return Err(HexColorParseError::InvalidLength),
        };

        Ok(Self(r, g, b))
    }
}

struct HexColorVisitor;

impl Visitor<'_> for HexColorVisitor {
    type Value = HexColor;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a hex color string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        HexColor::from_str(v).map_err(|_| E::custom("invalid hex color"))
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(HexColorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::HexColor;

    #[test]
    fn hex_color_display() {
        let hex_color = HexColor(255, 255, 255);
        assert_eq!(hex_color.to_string(), "#FFFFFF");
    }

    #[test]
    fn serialize() {
        let hex_color = HexColor(252, 177, 3);
        let serialized = serde_json::to_string(&hex_color).unwrap();
        assert_eq!(serialized, "\"#FCB103\"");
    }

    #[test]
    fn serialize_2() {
        let hex_color = HexColor(255, 255, 255);
        let serialized = serde_json::to_string(&hex_color).unwrap();
        assert_eq!(serialized, "\"#FFFFFF\"");
    }

    #[test]
    fn deserialize() {
        let deserialized: HexColor = serde_json::from_str("\"#FFFFFF\"").unwrap();
        assert_eq!(deserialized, HexColor(255, 255, 255));
    }

    #[test]
    fn deserialize_invalid() {
        let deserialized: Result<HexColor, _> = serde_json::from_str("\"#GGGGGG\"");
        assert!(deserialized.is_err());
    }
}
