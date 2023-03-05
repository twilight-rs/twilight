use crate::application::command::permissions::GuildCommandPermissions;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct CommandPermissionsUpdate(pub GuildCommandPermissions);

impl Deref for CommandPermissionsUpdate {
    type Target = GuildCommandPermissions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommandPermissionsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
