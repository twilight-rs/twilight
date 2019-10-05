use crate::gateway::presence::Status;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClientStatus {
    pub desktop: Option<Status>,
    pub mobile: Option<Status>,
    pub web: Option<Status>,
}
