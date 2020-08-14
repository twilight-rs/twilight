use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    #[serde(rename = "activities.read")]
    ActivitiesRead,
    #[serde(rename = "activities.write")]
    ActivitiesWrite,
    #[serde(rename = "applications.builds.read")]
    ApplicationsBuildsRead,
    #[serde(rename = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    #[serde(rename = "applications.entitlements")]
    ApplicationsEntitlements,
    #[serde(rename = "applications.store.update")]
    ApplicationsStoreUpdate,
    Bot,
    Connections,
    Email,
    #[serde(rename = "gdm.join")]
    GdmJoin,
    Guilds,
    #[serde(rename = "guilds.join")]
    GuildsJoin,
    Identify,
    #[serde(rename = "messages.read")]
    MessagesRead,
    #[serde(rename = "relationships.read")]
    RelationshipsRead,
    Rpc,
    #[serde(rename = "rpc.api")]
    RpcApi,
    #[serde(rename = "rpc.notifications.read")]
    RpcNotificationsRead,
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
