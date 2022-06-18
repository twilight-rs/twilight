use super::InteractionType;
use crate::id::{
    marker::{ApplicationMarker, InteractionMarker},
    Id,
};
use serde::Serialize;

/// Data present in an [`Interaction`] of type [`Ping`].
///
/// [`Interaction`]: super::Interaction
/// [`Ping`]: super::Interaction::Ping
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct Ping {
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Kind of the interaction.
    ///
    /// Should always be `InteractionType::Ping`.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Token of the interaction.
    pub token: String,
}
