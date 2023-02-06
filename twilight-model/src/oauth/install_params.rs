use super::Scope;
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
    /// List of [scopes] to add the application to the guild with.
    ///
    /// [scopes]: crate::oauth::Scope
    pub scopes: Vec<Scope>,
}

#[cfg(test)]
mod tests {
    use super::InstallParams;
    use crate::{guild::Permissions, oauth::Scope};
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
    fn serde() {
        let value = InstallParams {
            permissions: Permissions::empty(),
            scopes: Vec::from([
                Scope::APPLICATIONS_COMMANDS,
                Scope::APPLICATIONS_COMMANDS_PERMISSIONS_UPDATE,
                Scope::IDENTIFY,
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
                Token::NewtypeStruct { name: "Scope" },
                Token::String(Scope::APPLICATIONS_COMMANDS.get()),
                Token::NewtypeStruct { name: "Scope" },
                Token::String(Scope::APPLICATIONS_COMMANDS_PERMISSIONS_UPDATE.get()),
                Token::NewtypeStruct { name: "Scope" },
                Token::String(Scope::IDENTIFY.get()),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
