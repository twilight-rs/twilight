use crate::{
    request::application::role_connections::{GetMetadata, SetMetadata},
    Client,
};
use twilight_model::{
    application::role_connection::Metadata,
    id::{marker::ApplicationMarker, Id},
};

/// Client interface for application role connections.
#[derive(Debug)]
pub struct RoleConnectionsClient<'a> {
    application_id: Id<ApplicationMarker>,
    client: &'a Client,
}

impl<'a> RoleConnectionsClient<'a> {
    /// Create a new interface for using interactions.
    pub(super) const fn new(client: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            client,
        }
    }

    /// Get application role connections metadata.
    pub const fn metadata(&'a self) -> GetMetadata<'a> {
        GetMetadata::new(self.client, self.application_id)
    }

    /// Set the application role connections metadata.
    pub const fn set_metadata(&'a self, records: &'a [Metadata]) -> SetMetadata<'a> {
        SetMetadata::new(self.client, self.application_id, records)
    }
}

#[cfg(test)]
mod tests {
    use super::RoleConnectionsClient;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(RoleConnectionsClient<'_>: Debug, Send, Sync);
}
