use serde::{Deserialize, Serialize};

/// OAuth 2 scopes that must be requested for access to different resources.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Fetch data from the user's "Now Playing/Recently Played" list.
    ///
    /// This is only available to allowlisted applications.
    #[serde(rename = "activities.read")]
    ActivitiesRead,
    /// Update the user's activity.
    ///
    /// This is only available to allowlisted applications.
    #[serde(rename = "activities.write")]
    ActivitiesWrite,
    /// Read build data for a user's applications.
    #[serde(rename = "applications.builds.read")]
    ApplicationsBuildsRead,
    /// Upload/update builds for a user's applications.
    ///
    /// This is only available to allowlisted applications.
    #[serde(rename = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    /// Read entitlements for a user's applications.
    #[serde(rename = "applications.entitlements")]
    ApplicationsEntitlements,
    /// Read and update store data for a user's applications.
    ///
    /// This includes things like SKUs, store listings, achievements, etc.
    #[serde(rename = "applications.store.update")]
    ApplicationsStoreUpdate,
    /// Put the bot in the user's selected guild, or a provided guild by ID.
    Bot,
    /// Read linked third-party account connections.
    Connections,
    /// Read the user's email address.
    Email,
    /// Add users to group DMs.
    #[serde(rename = "gdm.join")]
    GdmJoin,
    /// Retrieve basic information about a user's guilds.
    Guilds,
    /// Add users to guilds.
    #[serde(rename = "guilds.join")]
    GuildsJoin,
    /// Retrieve basic user information without an email address.
    Identify,
    /// Read messages from all client channels.
    ///
    /// For non-local RPC server API access, this will be restricted to channels
    /// and guilds the application creates.
    #[serde(rename = "messages.read")]
    MessagesRead,
    /// Read a user's friends and implicit relationships.
    ///
    /// This is only available to allowlisted applications.
    #[serde(rename = "relationships.read")]
    RelationshipsRead,
    /// Control a user's local Discord client via local RPC API access.
    ///
    /// This is only available to allowlisted applications.
    Rpc,
    /// Access the API as the local user via local RPC API access.
    ///
    /// This is only available to allowlisted applications.
    #[serde(rename = "rpc.api")]
    RpcApi,
    /// Receive notifications pushed out to a user via local RPC API access.
    ///
    /// This is only available to allowlisted applications.
    #[serde(rename = "rpc.notifications.read")]
    RpcNotificationsRead,
    /// Generate a webhook for a selected guild and channel.
    ///
    /// The webhook is returned in the OAuth token response for authorization
    /// code grants.
    #[serde(rename = "webhook.incoming")]
    WebhookIncoming,
}

impl Scope {
    /// Return the name of the scope.
    ///
    /// This is equivalent to what you would get when serializing it.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_oauth2::Scope;
    ///
    /// assert_eq!("bot", Scope::Bot.name());
    /// assert_eq!("identify", Scope::Identify.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::ActivitiesRead => "activities.read",
            Self::ActivitiesWrite => "activities.write",
            Self::ApplicationsBuildsRead => "applications.builds.read",
            Self::ApplicationsBuildsUpload => "applications.builds.upload",
            Self::ApplicationsEntitlements => "applications.entitlements",
            Self::ApplicationsStoreUpdate => "applications.store.update",
            Self::Bot => "bot",
            Self::Connections => "connections",
            Self::Email => "email",
            Self::GdmJoin => "gdm.join",
            Self::Guilds => "guilds",
            Self::GuildsJoin => "guilds.join",
            Self::Identify => "identify",
            Self::MessagesRead => "messages.read",
            Self::RelationshipsRead => "relationships.read",
            Self::Rpc => "rpc",
            Self::RpcApi => "rpc.api",
            Self::RpcNotificationsRead => "rpc.notifications.read",
            Self::WebhookIncoming => "webhook.incoming",
        }
    }
}

pub fn join(scopes: &[Scope]) -> String {
    let mut buf = String::new();

    let scope_count = scopes.len().saturating_sub(1);

    for (idx, scope) in scopes.iter().enumerate() {
        buf.push_str(scope.name());

        if idx < scope_count {
            buf.push(' ');
        }
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::Scope;

    #[test]
    fn test_join() {
        assert!(super::join(&[]).is_empty());
        assert_eq!("bot", super::join(&[Scope::Bot]));
        assert_eq!("bot identify", super::join(&[Scope::Bot, Scope::Identify]));
    }

    #[test]
    fn test_scopes() {
        assert_eq!("activities.read", Scope::ActivitiesRead.name());
        assert_eq!("activities.write", Scope::ActivitiesWrite.name());
        assert_eq!(
            "applications.builds.read",
            Scope::ApplicationsBuildsRead.name()
        );
        assert_eq!(
            "applications.builds.upload",
            Scope::ApplicationsBuildsUpload.name()
        );
        assert_eq!(
            "applications.entitlements",
            Scope::ApplicationsEntitlements.name()
        );
        assert_eq!(
            "applications.store.update",
            Scope::ApplicationsStoreUpdate.name()
        );
        assert_eq!("bot", Scope::Bot.name());
        assert_eq!("connections", Scope::Connections.name());
        assert_eq!("email", Scope::Email.name());
        assert_eq!("gdm.join", Scope::GdmJoin.name());
        assert_eq!("guilds", Scope::Guilds.name());
        assert_eq!("guilds.join", Scope::GuildsJoin.name());
        assert_eq!("identify", Scope::Identify.name());
        assert_eq!("messages.read", Scope::MessagesRead.name());
        assert_eq!("relationships.read", Scope::RelationshipsRead.name());
        assert_eq!("rpc", Scope::Rpc.name());
        assert_eq!("rpc.api", Scope::RpcApi.name());
        assert_eq!("rpc.notifications.read", Scope::RpcNotificationsRead.name());
        assert_eq!("webhook.incoming", Scope::WebhookIncoming.name());
    }
}
