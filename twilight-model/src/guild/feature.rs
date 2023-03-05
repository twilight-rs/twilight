#![allow(deprecated)]
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Special and optional guild features.
///
/// See [Discord Docs/Guild Features].
///
/// [Discord Docs/Guild Features]: https://discord.com/developers/docs/resources/guild#guild-object-guild-features
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "String", into = "Cow<'static, str>")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub enum GuildFeature {
    /// Has access to set an animated guild banner image.
    AnimatedBanner,
    /// Has access to set an animated guild icon.
    AnimatedIcon,
    /// Has set up auto moderation rules.
    AutoModeration,
    /// Has access to set a guild banner image.
    Banner,
    /// Has access to use commerce features (create store channels).
    #[deprecated]
    Commerce,
    /// Can enable welcome screen, membership screening, stage channels,
    /// discovery, and receives community updates.
    Community,
    /// Guild has enabled monetization.
    CreatorMonetizableProvisional,
    /// Guild has enabled the role subscription promotional page.
    CreatorStorePage,
    /// Guild has been set as a support server on the App Directory.
    DeveloperSupportServer,
    /// Is able to be discovered in the directory.
    Discoverable,
    /// Is able to be featured in the directory.
    Featurable,
    /// Invites have been paused, this prevents new users from joining.
    InvitesDisabled,
    /// Has access to set an invite splash background.
    InviteSplash,
    /// Has enabled membership screening.
    MemberVerificationGateEnabled,
    /// Has enabled monetization.
    #[deprecated(since = "0.14.1", note = "not in active use by discord")]
    MonetizationEnabled,
    /// Has increased custom sticker slots.
    MoreStickers,
    /// Has access to create news channels.
    News,
    /// Is partnered.
    Partnered,
    /// Can be previewed before joining via membership screening or the directory.
    PreviewEnabled,
    /// Has access to create private threads.
    PrivateThreads,
    /// Is able to set role icons.
    RoleIcons,
    /// Guild has role subscriptions that can be purchased.
    RoleSubscriptionsAvailableForPurchase,
    /// Guild has enabled role subscriptions.
    RoleSubscriptionsEnabled,
    /// Has enabled ticketed events.
    TicketedEventsEnabled,
    /// Has access to set a vanity URL.
    VanityUrl,
    /// Is verified.
    Verified,
    /// Has access to set 384kps bitrate in voice (previously VIP voice servers).
    VipRegions,
    /// Has enabled the welcome screen.
    WelcomeScreenEnabled,
    /// Variant value is unknown to the library.
    Unknown(String),
}

impl From<GuildFeature> for Cow<'static, str> {
    fn from(value: GuildFeature) -> Self {
        match value {
            GuildFeature::AnimatedBanner => "ANIMATED_BANNER".into(),
            GuildFeature::AnimatedIcon => "ANIMATED_ICON".into(),
            GuildFeature::AutoModeration => "AUTO_MODERATION".into(),
            GuildFeature::Banner => "BANNER".into(),
            GuildFeature::Commerce => "COMMERCE".into(),
            GuildFeature::Community => "COMMUNITY".into(),
            GuildFeature::CreatorMonetizableProvisional => "CREATOR_MONETIZABLE_PROVISIONAL".into(),
            GuildFeature::CreatorStorePage => "CREATOR_STORE_PAGE".into(),
            GuildFeature::DeveloperSupportServer => "DEVELOPER_SUPPORT_SERVER".into(),
            GuildFeature::Discoverable => "DISCOVERABLE".into(),
            GuildFeature::Featurable => "FEATURABLE".into(),
            GuildFeature::InvitesDisabled => "INVITES_DISABLED".into(),
            GuildFeature::InviteSplash => "INVITE_SPLASH".into(),
            GuildFeature::MemberVerificationGateEnabled => {
                "MEMBER_VERIFICATION_GATE_ENABLED".into()
            }
            GuildFeature::MonetizationEnabled => "MONETIZATION_ENABLED".into(),
            GuildFeature::MoreStickers => "MORE_STICKERS".into(),
            GuildFeature::News => "NEWS".into(),
            GuildFeature::Partnered => "PARTNERED".into(),
            GuildFeature::PreviewEnabled => "PREVIEW_ENABLED".into(),
            GuildFeature::PrivateThreads => "PRIVATE_THREADS".into(),
            GuildFeature::RoleIcons => "ROLE_ICONS".into(),
            GuildFeature::RoleSubscriptionsAvailableForPurchase => {
                "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE".into()
            }
            GuildFeature::RoleSubscriptionsEnabled => "ROLE_SUBSCRIPTIONS_ENABLED".into(),
            GuildFeature::TicketedEventsEnabled => "TICKETED_EVENTS_ENABLED".into(),
            GuildFeature::VanityUrl => "VANITY_URL".into(),
            GuildFeature::Verified => "VERIFIED".into(),
            GuildFeature::VipRegions => "VIP_REGIONS".into(),
            GuildFeature::WelcomeScreenEnabled => "WELCOME_SCREEN_ENABLED".into(),
            GuildFeature::Unknown(unknown) => unknown.into(),
        }
    }
}

impl From<String> for GuildFeature {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ANIMATED_BANNER" => Self::AnimatedBanner,
            "ANIMATED_ICON" => Self::AnimatedIcon,
            "AUTO_MODERATION" => Self::AutoModeration,
            "BANNER" => Self::Banner,
            "COMMERCE" => Self::Commerce,
            "COMMUNITY" => Self::Community,
            "CREATOR_MONETIZABLE_PROVISIONAL" => GuildFeature::CreatorMonetizableProvisional,
            "CREATOR_STORE_PAGE" => GuildFeature::CreatorStorePage,
            "DEVELOPER_SUPPORT_SERVER" => Self::DeveloperSupportServer,
            "DISCOVERABLE" => Self::Discoverable,
            "FEATURABLE" => Self::Featurable,
            "INVITES_DISABLED" => Self::InvitesDisabled,
            "INVITE_SPLASH" => Self::InviteSplash,
            "MEMBER_VERIFICATION_GATE_ENABLED" => Self::MemberVerificationGateEnabled,
            "MONETIZATION_ENABLED" => Self::MonetizationEnabled,
            "MORE_STICKERS" => Self::MoreStickers,
            "NEWS" => Self::News,
            "PARTNERED" => Self::Partnered,
            "PREVIEW_ENABLED" => Self::PreviewEnabled,
            "PRIVATE_THREADS" => Self::PrivateThreads,
            "ROLE_ICONS" => Self::RoleIcons,
            "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE" => {
                GuildFeature::RoleSubscriptionsAvailableForPurchase
            }
            "ROLE_SUBSCRIPTIONS_ENABLED" => GuildFeature::RoleSubscriptionsEnabled,
            "TICKETED_EVENTS_ENABLED" => Self::TicketedEventsEnabled,
            "VANITY_URL" => Self::VanityUrl,
            "VERIFIED" => Self::Verified,
            "VIP_REGIONS" => Self::VipRegions,
            "WELCOME_SCREEN_ENABLED" => Self::WelcomeScreenEnabled,
            _ => Self::Unknown(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GuildFeature;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(
            &GuildFeature::AnimatedBanner,
            &[Token::Str("ANIMATED_BANNER")],
        );
        serde_test::assert_tokens(&GuildFeature::AnimatedIcon, &[Token::Str("ANIMATED_ICON")]);
        serde_test::assert_tokens(
            &GuildFeature::AutoModeration,
            &[Token::Str("AUTO_MODERATION")],
        );
        serde_test::assert_tokens(&GuildFeature::Banner, &[Token::Str("BANNER")]);
        serde_test::assert_tokens(&GuildFeature::Commerce, &[Token::Str("COMMERCE")]);
        serde_test::assert_tokens(&GuildFeature::Community, &[Token::Str("COMMUNITY")]);
        serde_test::assert_tokens(
            &GuildFeature::CreatorMonetizableProvisional,
            &[Token::Str("CREATOR_MONETIZABLE_PROVISIONAL")],
        );
        serde_test::assert_tokens(
            &GuildFeature::CreatorStorePage,
            &[Token::Str("CREATOR_STORE_PAGE")],
        );
        serde_test::assert_tokens(
            &GuildFeature::DeveloperSupportServer,
            &[Token::Str("DEVELOPER_SUPPORT_SERVER")],
        );
        serde_test::assert_tokens(&GuildFeature::Discoverable, &[Token::Str("DISCOVERABLE")]);
        serde_test::assert_tokens(&GuildFeature::Featurable, &[Token::Str("FEATURABLE")]);
        serde_test::assert_tokens(
            &GuildFeature::InvitesDisabled,
            &[Token::Str("INVITES_DISABLED")],
        );
        serde_test::assert_tokens(&GuildFeature::InviteSplash, &[Token::Str("INVITE_SPLASH")]);
        serde_test::assert_tokens(
            &GuildFeature::MemberVerificationGateEnabled,
            &[Token::Str("MEMBER_VERIFICATION_GATE_ENABLED")],
        );
        serde_test::assert_tokens(
            &GuildFeature::MonetizationEnabled,
            &[Token::Str("MONETIZATION_ENABLED")],
        );
        serde_test::assert_tokens(&GuildFeature::MoreStickers, &[Token::Str("MORE_STICKERS")]);
        serde_test::assert_tokens(&GuildFeature::News, &[Token::Str("NEWS")]);
        serde_test::assert_tokens(&GuildFeature::Partnered, &[Token::Str("PARTNERED")]);
        serde_test::assert_tokens(
            &GuildFeature::PreviewEnabled,
            &[Token::Str("PREVIEW_ENABLED")],
        );
        serde_test::assert_tokens(
            &GuildFeature::PrivateThreads,
            &[Token::Str("PRIVATE_THREADS")],
        );
        serde_test::assert_tokens(&GuildFeature::RoleIcons, &[Token::Str("ROLE_ICONS")]);
        serde_test::assert_tokens(
            &GuildFeature::RoleSubscriptionsAvailableForPurchase,
            &[Token::Str("ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE")],
        );
        serde_test::assert_tokens(
            &GuildFeature::RoleSubscriptionsEnabled,
            &[Token::Str("ROLE_SUBSCRIPTIONS_ENABLED")],
        );
        serde_test::assert_tokens(
            &GuildFeature::TicketedEventsEnabled,
            &[Token::Str("TICKETED_EVENTS_ENABLED")],
        );
        serde_test::assert_tokens(&GuildFeature::VanityUrl, &[Token::Str("VANITY_URL")]);
        serde_test::assert_tokens(&GuildFeature::Verified, &[Token::Str("VERIFIED")]);
        serde_test::assert_tokens(&GuildFeature::VipRegions, &[Token::Str("VIP_REGIONS")]);
        serde_test::assert_tokens(
            &GuildFeature::WelcomeScreenEnabled,
            &[Token::Str("WELCOME_SCREEN_ENABLED")],
        );
        serde_test::assert_tokens(
            &GuildFeature::Unknown("UNKNOWN".to_owned()),
            &[Token::Str("UNKNOWN")],
        );
    }
}
