use crate::application::command::permissions::GuildCommandPermission;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandPermissionsUpdate(pub GuildCommandPermission);

impl Deref for CommandPermissionsUpdate {
    type Target = GuildCommandPermission;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommandPermissionsUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
