use serde::de::{Error as DeError, Visitor};
use std::{
    convert::TryFrom,
    fmt::{Formatter, Result as FmtResult},
    marker::PhantomData,
};

pub struct U16EnumVisitor<'a> {
    description: &'a str,
    phantom: PhantomData<u16>,
}

impl<'a> U16EnumVisitor<'a> {
    pub const fn new(description: &'a str) -> Self {
        Self {
            description,
            phantom: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for U16EnumVisitor<'_> {
    type Value = u16;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.description)
    }

    fn visit_u16<E: DeError>(self, value: u16) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        let smaller = u16::try_from(value).map_err(E::custom)?;

        self.visit_u16(smaller)
    }
}
