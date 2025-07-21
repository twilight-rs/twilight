use super::{
    application_integration_type::{ApplicationIntegrationMap, ApplicationIntegrationTypeConfig},
    team::Team,
    ApplicationFlags, InstallParams,
};
use crate::{
    guild::Guild,
    id::{
        marker::{ApplicationMarker, GuildMarker, OauthSkuMarker},
        Id,
    },
    user::User,
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Deserializer, Serialize};

// Custom deserializer for redirect_uris that filters out null values
fn deserialize_redirect_uris<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RedirectUris {
        Strings(Vec<String>),
        OptionalStrings(Vec<Option<String>>),
    }

    let opt_redirect_uris: Option<RedirectUris> = Option::deserialize(deserializer)?;

    Ok(opt_redirect_uris.map(|redirect_uris| match redirect_uris {
        RedirectUris::Strings(vec) => vec,
        RedirectUris::OptionalStrings(vec) => vec.into_iter().filter_map(|item| item).collect(),
    }))
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application {
    /// Approximate count of guilds this app has been added to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_guild_count: Option<u64>,
    /// Approximate count of users that have installed the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_user_install_count: Option<u64>,
    /// Partial user object for the bot user associated with the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<User>,
    /// When `false`, only the app owner can add the app to guilds
    pub bot_public: bool,
    /// When `true`, the app's bot will only join upon completion of the
    /// full OAuth2 code grant flow
    pub bot_require_code_grant: bool,
    /// Default rich presence invite cover image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<ImageHash>,
    /// Application's default custom authorization link, if enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<String>,
    /// Description of the application.
    pub description: String,
    /// Event webhooks URL for the app to receive webhook events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_url: Option<String>,
    /// If webhook events are enabled for the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_status: Option<u64>,
    /// List of Webhook event types the app subscribes to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_types: Option<Vec<String>>,
    /// Public flags of the application.
    pub flags: Option<ApplicationFlags>,
    /// Partial object of the associated guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Guild>,
    /// Guild associated with the app. For example, a developer support server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Icon of the application.
    pub icon: Option<ImageHash>,
    /// ID of the application.
    pub id: Id<ApplicationMarker>,
    /// Settings for the application's default in-app authorization, if enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_params: Option<InstallParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types_config:
        Option<ApplicationIntegrationMap<ApplicationIntegrationTypeConfig>>,
    /// Interactions endpoint URL for the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<String>,
    /// Name of the application.
    pub name: String,
    /// Partial user object for the owner of the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    /// If this app is a game sold on Discord, this field will be the
    /// id of the "Game SKU" that is created, if exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_sku_id: Option<Id<OauthSkuMarker>>,
    /// URL of the application's privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    /// Redirect URIs for the application.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_redirect_uris",
        default
    )]
    pub redirect_uris: Option<Vec<String>>,
    /// Role connection verification URL for the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<String>,
    #[serde(default)]
    pub rpc_origins: Vec<String>,
    /// If this app is a game sold on Discord, this field will be the
    /// URL slug that links to the store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Tags describing the content and functionality of the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// If the app belongs to a team, this will be a list of the
    /// members of that team.
    pub team: Option<Team>,
    /// URL of the application's terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
    pub verify_key: String,
}

#[cfg(test)]
mod tests {
    use super::{Application, ApplicationFlags, Team, User};
    use crate::{
        id::Id,
        oauth::{
            application_integration_type::{
                ApplicationIntegrationMap, ApplicationIntegrationTypeConfig,
            },
            team::{TeamMember, TeamMembershipState},
        },
        test::image_hash,
        util::image_hash::ImageHash,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        Application: approximate_guild_count,
        approximate_user_install_count,
        bot,
        bot_public,
        bot_require_code_grant,
        cover_image,
        custom_install_url,
        description,
        event_webhooks_url,
        event_webhooks_status,
        event_webhooks_types,
        flags,
        guild,
        guild_id,
        icon,
        id,
        install_params,
        integration_types_config,
        interactions_endpoint_url,
        name,
        owner,
        primary_sku_id,
        privacy_policy_url,
        redirect_uris,
        role_connections_verification_url,
        rpc_origins,
        slug,
        tags,
        team,
        terms_of_service_url,
        verify_key
    );

    assert_impl_all!(
        Application: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );

    #[allow(clippy::too_many_lines)]
    #[test]
    fn current_application_info() {
        let value = Application {
            approximate_guild_count: Some(2),
            approximate_user_install_count: Some(5),
            bot: None,
            bot_public: true,
            bot_require_code_grant: false,
            cover_image: Some(image_hash::COVER),
            custom_install_url: None,
            description: "a pretty cool application".to_owned(),
            event_webhooks_url: None,
            event_webhooks_status: None,
            event_webhooks_types: None,
            flags: Some(ApplicationFlags::EMBEDDED),
            guild: None,
            guild_id: Some(Id::new(1)),
            icon: Some(image_hash::ICON),
            id: Id::new(2),
            install_params: None,
            integration_types_config: Some(ApplicationIntegrationMap {
                guild: Some(ApplicationIntegrationTypeConfig {
                    oauth2_install_params: None,
                }),
                user: None,
            }),
            interactions_endpoint_url: Some("https://interactions".into()),
            name: "cool application".to_owned(),
            owner: Some(User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(3),
                locale: None,
                mfa_enabled: None,
                name: "app dev".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
            primary_sku_id: Some(Id::new(4)),
            privacy_policy_url: Some("https://privacypolicy".into()),
            redirect_uris: Some(vec![
                "https://localhost:3000/api/auth".to_owned(),
                "https://example.com/dashboard".to_owned(),
                "https://example.com/api/auth/callback/discord".to_owned(),
            ]),
            role_connections_verification_url: Some("https://roleconnections".into()),
            rpc_origins: vec!["one".to_owned()],
            slug: Some("app slug".to_owned()),
            tags: Some(Vec::from([
                "ponies".to_owned(),
                "horses".to_owned(),
                "friendship".to_owned(),
                "magic".to_owned(),
            ])),
            team: Some(Team {
                icon: None,
                id: Id::new(5),
                members: Vec::new(),
                name: "team name".into(),
                owner_user_id: Id::new(6),
            }),
            terms_of_service_url: Some("https://termsofservice".into()),
            verify_key: "key".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Application",
                    len: 24,
                },
                Token::Str("approximate_guild_count"),
                Token::Some,
                Token::U64(2),
                Token::Str("approximate_user_install_count"),
                Token::Some,
                Token::U64(5),
                Token::Str("bot_public"),
                Token::Bool(true),
                Token::Str("bot_require_code_grant"),
                Token::Bool(false),
                Token::Str("cover_image"),
                Token::Some,
                Token::Str(image_hash::COVER_INPUT),
                Token::Str("description"),
                Token::Str("a pretty cool application"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(131_072),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("integration_types_config"),
                Token::Some,
                Token::Struct {
                    name: "ApplicationIntegrationMap",
                    len: 1,
                },
                Token::Str("0"),
                Token::Some,
                Token::Struct {
                    name: "ApplicationIntegrationTypeConfig",
                    len: 0,
                },
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("interactions_endpoint_url"),
                Token::Some,
                Token::Str("https://interactions"),
                Token::Str("name"),
                Token::Str("cool application"),
                Token::Str("owner"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("app dev"),
                Token::StructEnd,
                Token::Str("primary_sku_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("privacy_policy_url"),
                Token::Some,
                Token::Str("https://privacypolicy"),
                Token::Str("redirect_uris"),
                Token::Some,
                Token::Seq { len: Some(3) },
                Token::Str("https://localhost:3000/api/auth"),
                Token::Str("https://example.com/dashboard"),
                Token::Str("https://example.com/api/auth/callback/discord"),
                Token::SeqEnd,
                Token::Str("role_connections_verification_url"),
                Token::Some,
                Token::Str("https://roleconnections"),
                Token::Str("rpc_origins"),
                Token::Seq { len: Some(1) },
                Token::Str("one"),
                Token::SeqEnd,
                Token::Str("slug"),
                Token::Some,
                Token::Str("app slug"),
                Token::Str("tags"),
                Token::Some,
                Token::Seq { len: Some(4) },
                Token::Str("ponies"),
                Token::Str("horses"),
                Token::Str("friendship"),
                Token::Str("magic"),
                Token::SeqEnd,
                Token::Str("team"),
                Token::Some,
                Token::Struct {
                    name: "Team",
                    len: 5,
                },
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("name"),
                Token::Str("team name"),
                Token::Str("owner_user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::StructEnd,
                Token::Str("terms_of_service_url"),
                Token::Some,
                Token::Str("https://termsofservice"),
                Token::Str("verify_key"),
                Token::Str("key"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn deserialize_real_application_response() {
        let json_str = r#"{
            "approximate_guild_count": 1040,
            "approximate_user_install_count": 160,
            "bot": {
                "accent_color": null,
                "avatar": "c273213790e64f8230f7ea035817cbbf",
                "avatar_decoration_data": null,
                "banner": "c273213790e64f8230f7ea035817cbbf",
                "banner_color": null,
                "bot": true,
                "clan": null,
                "collectibles": null,
                "discriminator": "8114",
                "flags": 65536,
                "global_name": null,
                "id": "1123143036763906048",
                "primary_guild": null,
                "public_flags": 65536,
                "username": "Test Bot"
            },
            "bot_public": true,
            "bot_require_code_grant": false,
            "description": "Test application description",
            "flags": 28049408,
            "guild_id": "1149418873507033169",
            "icon": "c273213790e64f8230f7ea035817cbbf",
            "id": "1123143036763906048",
            "integration_types_config": {
                "0": {}
            },
            "name": "Test Application",
            "owner": {
                "accent_color": null,
                "avatar": null,
                "avatar_decoration_data": null,
                "banner": null,
                "banner_color": null,
                "clan": null,
                "collectibles": null,
                "discriminator": "0000",
                "flags": 1024,
                "global_name": null,
                "id": "1271810892136452150",
                "primary_guild": null,
                "public_flags": 1024,
                "username": "test_team_user"
            },
            "redirect_uris": [
                "https://localhost:3000/api/auth",
                "https://example.com/dashboard",
                "https://example.com/api/auth/callback/discord",
                null
            ],
            "tags": [
                "test tag 1",
                "test tag 2",
                "moderation",
                "test tag 3"
            ],
            "team": {
                "icon": "c273213790e64f8230f7ea035817cbbf",
                "id": "1271810892136452150",
                "members": [
                    {
                        "membership_state": 2,
                        "permissions": ["*"],
                        "role": "admin",
                        "team_id": "1271810892136452150",
                        "user": {
                            "accent_color": null,
                            "avatar": "c273213790e64f8230f7ea035817cbbf",
                            "avatar_decoration_data": null,
                            "banner": null,
                            "banner_color": null,
                            "clan": null,
                            "collectibles": null,
                            "discriminator": "0",
                            "flags": 0,
                            "global_name": "Test User 1",
                            "id": "303950242229911562",
                            "primary_guild": null,
                            "public_flags": 0,
                            "username": "testuser1"
                        }
                    },
                    {
                        "membership_state": 2,
                        "permissions": ["*"],
                        "role": "admin",
                        "team_id": "1271810892136452150",
                        "user": {
                            "accent_color": null,
                            "avatar": "c273213790e64f8230f7ea035817cbbf",
                            "avatar_decoration_data": null,
                            "banner": null,
                            "banner_color": null,
                            "clan": null,
                            "collectibles": null,
                            "discriminator": "0",
                            "flags": 4194304,
                            "global_name": "Test User 2",
                            "id": "117401110326673416",
                            "primary_guild": null,
                            "public_flags": 4194304,
                            "username": "testuser2"
                        }
                    },
                    {
                        "membership_state": 2,
                        "permissions": ["*"],
                        "role": "developer",
                        "team_id": "1271810892136452150",
                        "user": {
                            "accent_color": null,
                            "avatar": "c273213790e64f8230f7ea035817cbbf",
                            "avatar_decoration_data": null,
                            "banner": null,
                            "banner_color": null,
                            "clan": null,
                            "collectibles": null,
                            "discriminator": "0",
                            "flags": 0,
                            "global_name": "Test User 3",
                            "id": "460037808476782602",
                            "primary_guild": null,
                            "public_flags": 0,
                            "username": "testuser3"
                        }
                    }
                ],
                "name": "test team",
                "owner_user_id": "117401110326673416"
            },
            "verify_key": "mock_verify_key_hash",
            "rpc_origins": []
        }"#;

        let result: Application = serde_json::from_str(json_str).expect("json to be valid");
        let expected = Application {
            approximate_guild_count: Some(1040),
            approximate_user_install_count: Some(160),
            bot: Some(User {
                accent_color: None,
                avatar: Some(ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap()),
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: Some(ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap()),
                bot: true,
                discriminator: 8114,
                email: None,
                flags: Some(crate::user::UserFlags::from_bits_truncate(65536)),
                global_name: None,
                id: Id::new(1123143036763906048),
                locale: None,
                mfa_enabled: None,
                name: "Test Bot".to_owned(),
                premium_type: None,
                public_flags: Some(crate::user::UserFlags::from_bits_truncate(65536)),
                system: None,
                verified: None,
            }),
            bot_public: true,
            bot_require_code_grant: false,
            cover_image: None,
            custom_install_url: None,
            description: "Test application description".to_owned(),
            event_webhooks_url: None,
            event_webhooks_status: None,
            event_webhooks_types: None,
            flags: Some(ApplicationFlags::from_bits_truncate(28049408)),
            guild: None,
            guild_id: Some(Id::new(1149418873507033169)),
            icon: Some(ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap()),
            id: Id::new(1123143036763906048),
            install_params: None,
            integration_types_config: Some(ApplicationIntegrationMap {
                guild: Some(ApplicationIntegrationTypeConfig {
                    oauth2_install_params: None,
                }),
                user: None,
            }),
            interactions_endpoint_url: None,
            name: "Test Application".to_owned(),
            owner: Some(User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 0,
                email: None,
                flags: Some(crate::user::UserFlags::from_bits_truncate(1024)),
                global_name: None,
                id: Id::new(1271810892136452150),
                locale: None,
                mfa_enabled: None,
                name: "test_team_user".to_owned(),
                premium_type: None,
                public_flags: Some(crate::user::UserFlags::from_bits_truncate(1024)),
                system: None,
                verified: None,
            }),
            primary_sku_id: None,
            privacy_policy_url: None,
            redirect_uris: Some(vec![
                "https://localhost:3000/api/auth".to_owned(),
                "https://example.com/dashboard".to_owned(),
                "https://example.com/api/auth/callback/discord".to_owned(),
            ]),
            role_connections_verification_url: None,
            rpc_origins: vec![],
            slug: None,
            tags: Some(vec![
                "test tag 1".to_owned(),
                "test tag 2".to_owned(),
                "moderation".to_owned(),
                "test tag 3".to_owned(),
            ]),
            team: Some(Team {
                icon: Some(ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap()),
                id: Id::new(1271810892136452150),
                members: vec![
                    TeamMember {
                        membership_state: TeamMembershipState::Accepted,
                        permissions: vec!["*".to_owned()],
                        role: "admin".to_owned(),
                        team_id: Id::new(1271810892136452150),
                        user: User {
                            accent_color: None,
                            avatar: Some(
                                ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap(),
                            ),
                            avatar_decoration: None,
                            avatar_decoration_data: None,
                            banner: None,
                            bot: false,
                            discriminator: 0,
                            email: None,
                            flags: Some(crate::user::UserFlags::from_bits_truncate(0)),
                            global_name: Some("Test User 1".to_owned()),
                            id: Id::new(303950242229911562),
                            locale: None,
                            mfa_enabled: None,
                            name: "testuser1".to_owned(),
                            premium_type: None,
                            public_flags: Some(crate::user::UserFlags::from_bits_truncate(0)),
                            system: None,
                            verified: None,
                        },
                    },
                    TeamMember {
                        membership_state: TeamMembershipState::Accepted,
                        permissions: vec!["*".to_owned()],
                        role: "admin".to_owned(),
                        team_id: Id::new(1271810892136452150),
                        user: User {
                            accent_color: None,
                            avatar: Some(
                                ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap(),
                            ),
                            avatar_decoration: None,
                            avatar_decoration_data: None,
                            banner: None,
                            bot: false,
                            discriminator: 0,
                            email: None,
                            flags: Some(crate::user::UserFlags::from_bits_truncate(4194304)),
                            global_name: Some("Test User 2".to_owned()),
                            id: Id::new(117401110326673416),
                            locale: None,
                            mfa_enabled: None,
                            name: "testuser2".to_owned(),
                            premium_type: None,
                            public_flags: Some(crate::user::UserFlags::from_bits_truncate(4194304)),
                            system: None,
                            verified: None,
                        },
                    },
                    TeamMember {
                        membership_state: TeamMembershipState::Accepted,
                        permissions: vec!["*".to_owned()],
                        role: "developer".to_owned(),
                        team_id: Id::new(1271810892136452150),
                        user: User {
                            accent_color: None,
                            avatar: Some(
                                ImageHash::parse(b"c273213790e64f8230f7ea035817cbbf").unwrap(),
                            ),
                            avatar_decoration: None,
                            avatar_decoration_data: None,
                            banner: None,
                            bot: false,
                            discriminator: 0,
                            email: None,
                            flags: Some(crate::user::UserFlags::from_bits_truncate(0)),
                            global_name: Some("Test User 3".to_owned()),
                            id: Id::new(460037808476782602),
                            locale: None,
                            mfa_enabled: None,
                            name: "testuser3".to_owned(),
                            premium_type: None,
                            public_flags: Some(crate::user::UserFlags::from_bits_truncate(0)),
                            system: None,
                            verified: None,
                        },
                    },
                ],
                name: "test team".to_owned(),
                owner_user_id: Id::new(117401110326673416),
            }),
            terms_of_service_url: None,
            verify_key: "mock_verify_key_hash".to_owned(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn deserialize_redirect_uris_with_nulls() {
        let json_str = r#"{
            "redirect_uris": [
                "https://localhost:3000/api/auth",
                null,
                "https://example.com/dashboard",
                null,
                "https://example.com/api/auth/callback/discord"
            ]
        }"#;

        #[derive(Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "super::deserialize_redirect_uris")]
            redirect_uris: Option<Vec<String>>,
        }

        let result: TestStruct = serde_json::from_str(json_str).expect("json to be valid");
        let expected = vec![
            "https://localhost:3000/api/auth".to_owned(),
            "https://example.com/dashboard".to_owned(),
            "https://example.com/api/auth/callback/discord".to_owned(),
        ];

        assert_eq!(result.redirect_uris, Some(expected));
    }
}
