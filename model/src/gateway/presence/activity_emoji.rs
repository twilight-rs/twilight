#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}
