use serde::{Deserialize, Serialize};

use super::InstallParams;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ApplicationIntegrationType {
    GuildInstall,
    UserInstall,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl ApplicationIntegrationType {
    pub const fn kind(self) -> &'static str {
        match self {
            Self::GuildInstall => "GUILD_INSTALL",
            Self::UserInstall => "USER_INSTALL",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl From<u8> for ApplicationIntegrationType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::GuildInstall,
            1 => Self::UserInstall,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ApplicationIntegrationType> for u8 {
    fn from(value: ApplicationIntegrationType) -> Self {
        match value {
            ApplicationIntegrationType::GuildInstall => 0,
            ApplicationIntegrationType::UserInstall => 1,
            ApplicationIntegrationType::Unknown(unknown) => unknown,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ApplicationIntegrationMap<Guild, User = Guild> {
    #[serde(rename = "0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Guild>,
    #[serde(rename = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ApplicationIntegrationTypeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_install_params: Option<InstallParams>,
}
