use crate::guild::Permissions;
use serde::{Deserialize, Serialize};

/// Parameters for in-app authorization links.
///
/// Refer to [Discord Docs/Install Params Object].
///
/// [Discord Docs/Install Params Object]: https://discord.com/developers/docs/resources/application#install-params-object
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InstallParams {
    /// Permissions to request for the bot role.
    pub permissions: Permissions,
    /// Scopes to add the application to the guild with.
    pub scopes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::InstallParams;
    use crate::guild::Permissions;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        InstallParams: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_serde() {
        let value = InstallParams {
            permissions: Permissions::empty(),
            scopes: Vec::from([
                "applications.commands".to_owned(),
                "applications.commands.permissions.update".to_owned(),
                "identify".to_owned(),
            ]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InstallParams",
                    len: 2,
                },
                Token::String("permissions"),
                Token::String("0"),
                Token::String("scopes"),
                Token::Seq { len: Some(3) },
                Token::String("applications.commands"),
                Token::String("applications.commands.permissions.update"),
                Token::String("identify"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
