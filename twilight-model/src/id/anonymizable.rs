use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Id;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum AnonymizableId<T> {
    Anonymized,
    Id(Id<T>),
}

impl<T> Clone for AnonymizableId<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Anonymized => Self::Anonymized,
            Self::Id(id) => Self::Id(*id),
        }
    }
}

impl<T> Copy for AnonymizableId<T> {}

impl<'de, T> Deserialize<'de> for AnonymizableId<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Id::deserialize(deserializer).map_or(Self::Anonymized, Self::Id))
    }
}

impl<T> Eq for AnonymizableId<T> {}

impl<T> PartialEq for AnonymizableId<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Anonymized, Self::Anonymized) => true,
            (Self::Id(id), Self::Id(other_id)) => id == other_id,
            (_, _) => false,
        }
    }
}

impl<T> Hash for AnonymizableId<T> {
    fn hash<U: Hasher>(&self, state: &mut U) {
        match self {
            Self::Anonymized => state.write_u64(0),
            Self::Id(id) => state.write_u64(id.value.get()),
        }
    }
}

impl<T> Serialize for AnonymizableId<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Anonymized => serializer.serialize_newtype_struct("AnonymizableId", "0"),
            Self::Id(id) => serializer.serialize_newtype_struct("AnonymizableId", &id.to_string()),
        }
    }
}
