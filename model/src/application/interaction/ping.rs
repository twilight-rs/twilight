use super::InteractionType;
use crate::id::{
    marker::{ApplicationMarker, InteractionMarker},
    Id,
};
use serde::{ser::SerializeStruct, Serialize};

/// Data present in an [`Interaction`] of type [`Ping`].
///
/// [`Interaction`]: super::Interaction
/// [`Ping`]: super::Interaction::Ping
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ping {
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Kind of the interaction.
    pub kind: InteractionType,
    /// Token of the interaction.
    pub token: String,
}

impl Serialize for Ping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Interaction", 4)?;
        state.serialize_field("application_id", &self.application_id)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("type", &InteractionType::Ping)?;
        state.serialize_field("token", &self.token)?;
        state.end()
    }
}
