use super::MessageActivityType;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageActivity {
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: MessageActivityType,
    pub party_id: Option<String>,
}
