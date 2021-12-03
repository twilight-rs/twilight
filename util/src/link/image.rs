use std::{
    fmt::{self, Display},
    num::NonZeroU64,
};

use twilight_model::{
    channel::message::{
        sticker::{StickerBannerAssetId, StickerFormatType, StickerId},
        Sticker,
    },
    guild::{Emoji, Guild, Member, Role},
    id::{ApplicationId, EmojiId, GuildId, RoleId, UserId},
    oauth::{team::Team, CurrentApplicationInfo},
    user::User,
};

const BASE_URL: &'static str = "https://cdn.discordapp.com/";

/// The extension used for non-animated images
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    /// `.png`
    PNG,
    /// `.jpg`
    JPEG,
    /// `.webp`
    WEBP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum _Extension {
    PNG,
    JPEG,
    WEBP,
    GIF,
    Lottie,
}

impl Extension {
    fn into(self, animated: bool, always_static: bool) -> _Extension {
        if always_static || !animated {
            match self {
                Extension::PNG => _Extension::PNG,
                Extension::JPEG => _Extension::JPEG,
                Extension::WEBP => _Extension::WEBP,
            }
        } else {
            _Extension::GIF
        }
    }
}

impl Display for _Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ext = match self {
            _Extension::PNG => ".png",
            _Extension::JPEG => ".jpg",
            _Extension::WEBP => ".webp",
            _Extension::GIF => ".gif",
            &_Extension::Lottie => ".json",
        };
        f.write_str(ext)
    }
}

/// Size querystrings the CDN Endpoints can support
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageSize {
    /// No query
    Default,
    /// `?size=16`
    Smallest = 16,
    /// `?size=32`
    Smaller = 32,
    /// `?size=64`
    Small = 64,
    /// `?size=128`
    Medium = 128,
    /// `?size=256`
    Large = 256,
    /// `?size=512`
    Larger = 512,
    /// `?size=1024`
    Largest = 1024,
    /// `?size=2048`
    Huge = 2048,
    /// `?size=4096`
    Enormous = 4096,
}

impl Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let ImageSize::Default = self {
            return Ok(());
        }
        f.write_str("?size=")?;
        Display::fmt(self, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Formatter for a CDN Endpoint that implements `std::fmt::Display`
pub struct EndpointFormat<'a, T> {
    top: &'static str,
    id: T,
    hash: Option<&'a str>,
    extension: _Extension,
    size: ImageSize,
}

impl<'a, T: Display> Display for EndpointFormat<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(BASE_URL)?;

        f.write_str(self.top)?;
        Display::fmt(&self.id, f)?;
        if let Some(hash) = self.hash {
            f.write_str("/")?;
            f.write_str(hash)?;
        }

        Display::fmt(&self.extension, f)?;
        Display::fmt(&self.size, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Formatter for longer CDN Endpoints that implements `std::fmt::Display`
pub struct LongEndpointFormat<'a, T, E> {
    top1: &'static str,
    id1: T,
    top2: &'static str,
    id2: E,
    top3: &'static str,
    hash: &'a str,
    extension: _Extension,
    size: ImageSize,
}

impl<'a, T: Display, E: Display> Display for LongEndpointFormat<'a, T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(BASE_URL)?;

        f.write_str(self.top1)?;
        Display::fmt(&self.id1, f)?;
        f.write_str(self.top2)?;
        Display::fmt(&self.id2, f)?;
        f.write_str(self.top3)?;
        f.write_str("/")?;
        f.write_str(self.hash)?;

        Display::fmt(&self.extension, f)?;
        Display::fmt(&self.size, f)
    }
}

/// Return the CDN Endpoint to an emoji's image
pub trait EmojiImage<'a> {
    /// Returns the CDN Endpoint link of the emoji's image  
    /// Uses the given extension if the emoji isn't animated or `always_static` is `true`, otherwise uses `.gif`
    fn image(
        &self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> EndpointFormat<'a, EmojiId>;
}

impl<'a> EmojiImage<'a> for Emoji {
    fn image(
        &self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> EndpointFormat<'a, EmojiId> {
        EndpointFormat {
            top: "emojis/",
            id: self.id,
            hash: None,
            extension: extension.into(self.animated, always_static),
            size,
        }
    }
}

/// Return the CDN Endpoint to a guild's resources
pub trait GuildImage<'a> {
    /// Returns the CDN Endpoint link of the guild's icon or `None` if the guild has no icon  
    /// Uses the given extension if the icon isn't animated or `always_static` is `true`, otherwise uses `.gif`
    fn icon(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>>;

    /// Returns the CDN Endpoint link of the guild's splash or `None` if the guild has no splash  
    fn splash(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>>;

    /// Returns the CDN Endpoint link of the guild's discovery splash or `None` if the guild has no discovery splash  
    fn discovery_splash(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>>;

    /// Returns the CDN Endpoint link of the guild's banner or `None` if the guild has no banner  
    fn banner(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>>;
}

impl<'a> GuildImage<'a> for Guild {
    fn icon(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>> {
        let hash = self.icon.as_ref()?;
        Some(EndpointFormat {
            top: "icons/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(is_animated(hash), always_static),
            size,
        })
    }

    fn splash(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>> {
        let hash = self.splash.as_ref()?;
        Some(EndpointFormat {
            top: "splashes/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(false, false),
            size,
        })
    }

    fn discovery_splash(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>> {
        let hash = self.discovery_splash.as_ref()?;
        Some(EndpointFormat {
            top: "discovery-splashes/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(false, false),
            size,
        })
    }

    fn banner(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, GuildId>> {
        let hash = self.banner.as_ref()?;
        Some(EndpointFormat {
            top: "banners/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(false, false),
            size,
        })
    }
}

/// Return the CDN Endpoint to a user's resources
pub trait UserImage<'a> {
    /// Returns the CDN Endpoint link of the user's banner or `None` if the user has no banner  
    /// Uses the given extension if the banner isn't animated or `always_static` is `true`, otherwise uses `.gif`
    fn banner(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, UserId>>;

    /// Returns the CDN Endpoint link of the user's avatar or their default avatar depending on their discriminator if they have no avatar  
    /// Uses the given extension if the avatar isn't animated or `always_static` is `true`, otherwise uses `.gif`
    fn avatar(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> EndpointFormat<'a, u64>;
}

impl<'a> UserImage<'a> for User {
    fn banner(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, UserId>> {
        let hash = self.banner.as_ref()?;
        Some(EndpointFormat {
            top: "banners/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(is_animated(hash), always_static),
            size,
        })
    }

    fn avatar(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> EndpointFormat<'a, u64> {
        match self.avatar.as_ref() {
            Some(hash) => EndpointFormat {
                top: "avatars/",
                id: self.id.get(),
                hash: Some(hash),
                extension: extension.into(is_animated(hash), always_static),
                size,
            },
            None => EndpointFormat {
                top: "embed/avatars/",
                id: u64::from(self.discriminator % 5),
                hash: None,
                extension: _Extension::PNG,
                size: ImageSize::Default,
            },
        }
    }
}

/// Return the CDN Endpoint to a member's avatar in a guild
pub trait MemberImage<'a> {
    /// Returns the CDN Endpoint link of the member's avatar or `None` if the member has no avatar in the guild  
    /// Uses the given extension if the avatar isn't animated or `always_static` is `true`, otherwise uses `.gif`
    fn avatar(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<LongEndpointFormat<'a, GuildId, UserId>>;
}

impl<'a> MemberImage<'a> for Member {
    fn avatar(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<LongEndpointFormat<'a, GuildId, UserId>> {
        let hash = self.avatar.as_ref()?;

        Some(LongEndpointFormat {
            top1: "guilds/",
            id1: self.guild_id,
            top2: "/users/",
            id2: self.user.id,
            top3: "/avatars/",
            hash,
            extension: extension.into(is_animated(hash), always_static),
            size,
        })
    }
}

/// Return the CDN Endpoint to an applications's resources
pub trait ApplicationImage<'a> {
    /// Returns the CDN Endpoint link of the application's icon or `None` if the application has no icon
    fn icon(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, ApplicationId>>;

    /// Returns the CDN Endpoint link of the application's cover or `None` if the application has no cover
    fn cover(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, ApplicationId>>;
}

impl<'a> ApplicationImage<'a> for CurrentApplicationInfo {
    fn icon(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, ApplicationId>> {
        let hash = self.icon.as_ref()?;

        Some(EndpointFormat {
            top: "app-icons/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(false, false),
            size,
        })
    }

    fn cover(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, ApplicationId>> {
        let hash = self.cover_image.as_ref()?;

        Some(EndpointFormat {
            top: "app-icons/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(false, false),
            size,
        })
    }
}

/// Return the CDN Endpoint to a sticker pack's banner
pub trait StickerPackImage<'a> {
    /// Returns the CDN Endpoint link of the sticker pack's banner
    fn banner(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> EndpointFormat<'a, StickerBannerAssetId>;
}

impl<'a> StickerPackImage<'a> for StickerBannerAssetId {
    fn banner(
        &self,
        extension: Extension,
        size: ImageSize,
    ) -> EndpointFormat<'a, StickerBannerAssetId> {
        EndpointFormat {
            top: "app-assets/710982414301790216/store/",
            id: *self,
            hash: None,
            extension: extension.into(false, false),
            size,
        }
    }
}

/// Return the CDN Endpoint to a team's icon
pub trait TeamImage<'a> {
    /// Returns the CDN Endpoint link of the team's icon or `None` if the team has no icon
    fn icon(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, NonZeroU64>>;
}

impl<'a> TeamImage<'a> for Team {
    fn icon(
        &'a self,
        extension: Extension,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, NonZeroU64>> {
        let hash = self.icon.as_ref()?;

        Some(EndpointFormat {
            top: "team-icons//",
            id: self.id.0,
            hash: Some(hash),
            extension: extension.into(false, false),
            size,
        })
    }
}

/// Return the CDN Endpoint to a sticker's image
pub trait StickerImage<'a> {
    /// Returns the CDN Endpoint link of the sticker's image
    /// Uses the extension the sticker supports, `.png` or `.json` (Lottie)
    fn image(&'a self) -> EndpointFormat<'a, StickerId>;
}

impl<'a> StickerImage<'a> for Sticker {
    fn image(&'a self) -> EndpointFormat<'a, StickerId> {
        let extension = if let StickerFormatType::Lottie = self.format_type {
            _Extension::Lottie
        } else {
            _Extension::PNG
        };

        EndpointFormat {
            top: "sticker/",
            id: self.id,
            hash: None,
            extension,
            size: ImageSize::Default,
        }
    }
}

/// Return the CDN Endpoint to a role's icon
pub trait RoleImage<'a> {
    /// Returns the CDN Endpoint link of the role's icon or `None` if the role has no icon  
    /// Uses the given extension if the avatar isn't animated or `always_static` is `true`, otherwise uses `.gif`
    fn icon(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, RoleId>>;
}

impl<'a> RoleImage<'a> for Role {
    fn icon(
        &'a self,
        extension: Extension,
        always_static: bool,
        size: ImageSize,
    ) -> Option<EndpointFormat<'a, RoleId>> {
        let hash = self.icon.as_ref()?;

        Some(EndpointFormat {
            top: "role-icons/",
            id: self.id,
            hash: Some(hash),
            extension: extension.into(is_animated(hash), always_static),
            size,
        })
    }
}

fn is_animated(hash: &str) -> bool {
    hash.starts_with("a_")
}
