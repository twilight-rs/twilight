use super::Application;
use crate::{user::User, util::Timestamp};
use serde::{Deserialize, Serialize};

/// Information about the current OAuth2 authorization.
///
/// Requires authentication with a bearer token to make the request necessary to
/// retrieve this.
///
/// Refer to [Discord Docs/Get Current Authorization Information][1] for more
/// information.
///
/// [1]: https://discord.com/developers/docs/topics/oauth2#get-current-authorization-information
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentAuthorizationInformation {
    /// Current application.
    pub application: Application,
    /// When the access token expires.
    pub expires: Timestamp,
    /// List of [scopes] the [`user`] has authorized the [`application`] for.
    ///
    /// [`application`]: Self::application
    /// [`user`]: Self::user
    /// [scopes]: crate::oauth::scope
    pub scopes: Vec<String>,
    /// User who has authorized, if the user has authorized with the
    /// [`IDENTIFY`] scope.
    ///
    /// [`IDENTIFY`]: crate::oauth::scope::IDENTIFY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use crate::{
        id::Id,
        oauth::{scope, Application},
        test::image_hash,
        util::{datetime::TimestampParseError, Timestamp},
    };

    use super::CurrentAuthorizationInformation;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        CurrentAuthorizationInformation: application,
        expires,
        scopes,
        user
    );
    assert_impl_all!(
        CurrentAuthorizationInformation: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn serde() -> Result<(), TimestampParseError> {
        const DESCRIPTION: &str =
            "Twilight Sparkle is the central main character of My Little Pony Friendship is Magic.";
        const NAME: &str = "Twilight Sparkle";

        let value = CurrentAuthorizationInformation {
            application: Application {
                approximate_guild_count: Some(2),
                bot: None,
                bot_public: true,
                bot_require_code_grant: true,
                cover_image: None,
                custom_install_url: None,
                description: DESCRIPTION.to_owned(),
                guild_id: None,
                guild: None,
                flags: None,
                icon: Some(image_hash::ICON),
                id: Id::new(100_000_000_000_000_000),
                install_params: None,
                interactions_endpoint_url: None,
                name: NAME.to_owned(),
                owner: None,
                primary_sku_id: None,
                privacy_policy_url: None,
                redirect_uris: None,
                role_connections_verification_url: None,
                rpc_origins: Vec::new(),
                slug: None,
                tags: None,
                team: None,
                terms_of_service_url: None,
                verify_key: "a".to_owned(),
                integration_types_config: None,
            },
            expires: Timestamp::parse("2023-01-09T17:19:44.000000+00:00")?,
            scopes: Vec::from([scope::APPLICATIONS_COMMANDS_PERMISSIONS_UPDATE.to_owned()]),
            user: None,
        };

        assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentAuthorizationInformation",
                    len: 3,
                },
                Token::Str("application"),
                Token::Struct {
                    name: "Application",
                    len: 16,
                },
                Token::Str("approximate_guild_count"),
                Token::Some,
                Token::U64(2),
                Token::Str("bot_public"),
                Token::Bool(true),
                Token::Str("bot_require_code_grant"),
                Token::Bool(true),
                Token::Str("cover_image"),
                Token::None,
                Token::Str("description"),
                Token::Str(DESCRIPTION),
                Token::Str("guild_id"),
                Token::None,
                Token::Str("flags"),
                Token::None,
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100000000000000000"),
                Token::Str("name"),
                Token::Str(NAME),
                Token::Str("owner"),
                Token::None,
                Token::Str("primary_sku_id"),
                Token::None,
                Token::Str("rpc_origins"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("slug"),
                Token::None,
                Token::Str("team"),
                Token::None,
                Token::Str("verify_key"),
                Token::Str("a"),
                Token::StructEnd,
                Token::Str("expires"),
                Token::Str("2023-01-09T17:19:44.000000+00:00"),
                Token::Str("scopes"),
                Token::Seq { len: Some(1) },
                Token::Str(scope::APPLICATIONS_COMMANDS_PERMISSIONS_UPDATE),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
