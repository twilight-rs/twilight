use crate::util::known_string::KnownString;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

/// Localization setting.
///
/// Locales are used to present information in and customize the experience for
/// people who speak a language in a particular region.
///
/// Locales are configured per-[user]; can be configured as a [preferred locale]
/// on [guilds] with the [`COMMUNITY`] feature enabled; is
/// [present on interactions] with the locale of the invoking user; is used for
/// [localizing commands] and their [command options]; and so on.
///
/// # Examples
///
/// Print the user's locale:
///
/// ```no_run
/// use twilight_model::user::Locale;
///
/// let locale = Locale::CROATIAN;
/// let english_name = locale.english_name().unwrap_or("unknown");
/// let native_name = locale.native_name().unwrap_or("unknown");
///
/// println!("your locale is {english_name} ({native_name})");
/// ```
///
/// [`COMMUNITY`]: crate::guild::GuildFeature::COMMUNITY
/// [command options]: crate::application::command::CommandOption::name_localizations
/// [guilds]: crate::guild::Guild
/// [localizing commands]: crate::application::command::Command::name_localizations
/// [preferred locale]: crate::guild::Guild::preferred_locale
/// [user]: crate::user::User
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Locale(KnownString<5>);

impl Locale {
    /// Bulgarian locale.
    ///
    /// The native name of the locale is български.
    pub const BULGARIAN: Self = Self::from_bytes(b"bg");

    /// Chinese, China locale.
    ///
    /// The native name of the locale is 中文.
    pub const CHINESE_CHINA: Self = Self::from_bytes(b"zh-CN");

    /// Chinese, Taiwan locale.
    ///
    /// The native name of the locale is 繁體中文.
    pub const CHINESE_TAIWAN: Self = Self::from_bytes(b"zh-TW");

    /// Croatian locale.
    ///
    /// The native name of the locale is Hrvatski.
    pub const CROATIAN: Self = Self::from_bytes(b"hr");

    /// Czech locale.
    ///
    /// The native name of the locale is Čeština.
    pub const CZECH: Self = Self::from_bytes(b"cs");

    /// Danish locale.
    ///
    /// The native name of the locale is Dansk.
    pub const DANISH: Self = Self::from_bytes(b"da");

    /// English, UK locale.
    ///
    /// The native name of the locale is English, UK.
    pub const ENGLISH_UK: Self = Self::from_bytes(b"en-GB");

    /// English, US locale.
    ///
    /// The native name of the locale is English, US.
    pub const ENGLISH_US: Self = Self::from_bytes(b"en-US");

    /// Finnish locale.
    ///
    /// The native name of the locale is Suomi.
    pub const FINNISH: Self = Self::from_bytes(b"fi");

    /// French locale.
    ///
    /// The native name of the locale is Français.
    pub const FRENCH: Self = Self::from_bytes(b"fr");

    /// German locale.
    ///
    /// The native name of the locale is Deutsch.
    pub const GERMAN: Self = Self::from_bytes(b"de");

    /// Greek locale.
    ///
    /// The native name of the locale is Ελληνικά.
    pub const GREEK: Self = Self::from_bytes(b"el");

    /// Hindi locale.
    ///
    /// The native name of the locale is हिन्दी.
    pub const HINDI: Self = Self::from_bytes(b"hi");

    /// Hungarian locale.
    ///
    /// The native name of the locale is Magyar.
    pub const HUNGARIAN: Self = Self::from_bytes(b"hu");

    /// Indonesian locale.
    ///
    /// The native name of the locale is Bahasa Indonesia.
    pub const INDONESIAN: Self = Self::from_bytes(b"id");

    /// Italian locale.
    ///
    /// The native name of the locale is Italiano.
    pub const ITALIAN: Self = Self::from_bytes(b"it");

    /// Japanese locale.
    ///
    /// The native name of the locale is 日本語.
    pub const JAPANESE: Self = Self::from_bytes(b"ja");

    /// Korean locale.
    ///
    /// The native name of the locale is 한국어.
    pub const KOREAN: Self = Self::from_bytes(b"ko");

    /// Lithuanian locale.
    ///
    /// The native name of the locale is Lietuviškai.
    pub const LITHUANIAN: Self = Self::from_bytes(b"lt");

    /// Dutch locale.
    ///
    /// The native name of the locale is Nederlands.
    pub const NETHERLANDS: Self = Self::from_bytes(b"nl");

    /// Norwegian locale.
    ///
    /// The native name of the locale is Norsk.
    pub const NORWEGIAN: Self = Self::from_bytes(b"no");

    /// Polish locale.
    ///
    /// The native name of the locale is Polski.
    pub const POLISH: Self = Self::from_bytes(b"pl");

    /// Portuguese, Brazilian locale.
    ///
    /// The native name of the locale is Português do Brasil.
    pub const PORTUGUESE_BRAZILIAN: Self = Self::from_bytes(b"pt-BR");

    /// Romanian, Romania locale.
    ///
    /// The native name of the locale is Română.
    pub const ROMANIAN_ROMANIA: Self = Self::from_bytes(b"ro");

    /// Russian locale.
    ///
    /// The native name of the locale is Pусский.
    pub const RUSSIAN: Self = Self::from_bytes(b"ru");

    /// Spanish locale.
    ///
    /// The native name of the locale is Español.
    pub const SPANISH_SPAIN: Self = Self::from_bytes(b"es-ES");

    /// Swedish locale.
    ///
    /// The native name of the locale is Svenska.
    pub const SWEDISH_SWEDEN: Self = Self::from_bytes(b"sv-SE");

    /// Thai locale.
    ///
    /// The native name of the locale is ไทย.
    pub const THAI: Self = Self::from_bytes(b"th");

    /// Turkish locale.
    ///
    /// The native name of the locale is Türkçe.
    pub const TURKISH: Self = Self::from_bytes(b"tr");

    /// Vietnamese locale.
    ///
    /// The native name of the locale is Tiếng Việt.
    pub const VIETNAMESE: Self = Self::from_bytes(b"vi");

    /// Ukrainian locale.
    ///
    /// The native name of the locale is Українська.
    pub const UKRAINIAN: Self = Self::from_bytes(b"uk");

    /// Create a locale from a dynamic value.
    ///
    /// The provided locale must be 5 bytes or smaller.
    pub fn new(locale: &str) -> Option<Self> {
        KnownString::from_str(locale).map(Self)
    }

    /// Get the value of the locale.
    ///
    /// # Panics
    ///
    /// Panics if the locale isn't valid UTF-8.
    pub fn get(&self) -> &str {
        self.0.get()
    }

    /// English name of the locale.
    ///
    /// Values not mapping to a known locale return None.
    ///
    /// # Examples
    ///
    /// Check the English name of the [`BULGARIAN`][`Self::BULGARIAN`] and
    /// [`SWEDISH_SWEDEN`][`Self::SWEDISH_SWEDEN`] locales:
    ///
    /// ```
    /// use twilight_model::user::Locale;
    ///
    /// assert_eq!(Some("Bulgarian"), Locale::BULGARIAN.english_name());
    /// assert_eq!(Some("Swedish"), Locale::SWEDISH_SWEDEN.english_name());
    ///
    /// // Discord doesn't support the Mexican dialect of Spanish, so it
    /// // doesn't have a known representation for a display name.
    /// assert!(Locale::new("es-MX").unwrap().english_name().is_none());
    /// ```
    pub const fn english_name(self) -> Option<&'static str> {
        Some(match self {
            Self::BULGARIAN => "Bulgarian",
            Self::CHINESE_CHINA => "Chinese, China",
            Self::CHINESE_TAIWAN => "Chinese, Taiwan",
            Self::CROATIAN => "Croatian",
            Self::CZECH => "Czech",
            Self::DANISH => "Danish",
            Self::ENGLISH_UK => "English, UK",
            Self::ENGLISH_US => "English, US",
            Self::FINNISH => "Finnish",
            Self::FRENCH => "French",
            Self::GERMAN => "German",
            Self::GREEK => "Greek",
            Self::HINDI => "Hindi",
            Self::HUNGARIAN => "Hungarian",
            Self::INDONESIAN => "Indonesian",
            Self::ITALIAN => "Italian",
            Self::JAPANESE => "Japanese",
            Self::KOREAN => "Korean",
            Self::LITHUANIAN => "Lithuanian",
            Self::NETHERLANDS => "Dutch",
            Self::NORWEGIAN => "Norwegian",
            Self::POLISH => "Polish",
            Self::PORTUGUESE_BRAZILIAN => "Portuguese, Brazilian",
            Self::ROMANIAN_ROMANIA => "Romanian, Romania",
            Self::RUSSIAN => "Russian",
            Self::SPANISH_SPAIN => "Spanish",
            Self::SWEDISH_SWEDEN => "Swedish",
            Self::THAI => "Thai",
            Self::TURKISH => "Turkish",
            Self::VIETNAMESE => "Vietnamese",
            Self::UKRAINIAN => "Ukrainian",
            _ => return None,
        })
    }

    /// Native name of the locale.
    ///
    /// Values not mapping to a known locale return None.
    ///
    /// # Examples
    ///
    /// Check the Native name of the [`THAI`][`Self::THAI`] and
    /// [`VIETNAMESE`][`Self::VIETNAMESE`] locales:
    ///
    /// ```
    /// use twilight_model::user::Locale;
    ///
    /// assert_eq!(Some("ไทย"), Locale::THAI.native_name());
    /// assert_eq!(Some("Tiếng Việt"), Locale::VIETNAMESE.native_name());
    ///
    /// // Discord doesn't support Irish, so it doesn't have a known
    /// // representation for a display name.
    /// assert!(Locale::new("ga").unwrap().native_name().is_none());
    /// ```
    pub const fn native_name(self) -> Option<&'static str> {
        Some(match self {
            Self::BULGARIAN => "български",
            Self::CHINESE_CHINA => "中文",
            Self::CHINESE_TAIWAN => "繁體中文",
            Self::CROATIAN => "Hrvatski",
            Self::CZECH => "Čeština",
            Self::DANISH => "Dansk",
            Self::ENGLISH_UK => "English, UK",
            Self::ENGLISH_US => "English, US",
            Self::FINNISH => "Suomi",
            Self::FRENCH => "Français",
            Self::GERMAN => "Deutsch",
            Self::GREEK => "Ελληνικά",
            Self::HINDI => "हिन्दी",
            Self::HUNGARIAN => "Magyar",
            Self::INDONESIAN => "Bahasa Indonesia",
            Self::ITALIAN => "Italiano",
            Self::JAPANESE => "日本語",
            Self::KOREAN => "한국어",
            Self::LITHUANIAN => "Lietuviškai",
            Self::NETHERLANDS => "Nederlands",
            Self::NORWEGIAN => "Norsk",
            Self::POLISH => "Polski",
            Self::PORTUGUESE_BRAZILIAN => "Português do Brasil",
            Self::ROMANIAN_ROMANIA => "Română",
            Self::RUSSIAN => "Pусский",
            Self::SPANISH_SPAIN => "Español",
            Self::SWEDISH_SWEDEN => "Svenska",
            Self::THAI => "ไทย",
            Self::TURKISH => "Türkçe",
            Self::VIETNAMESE => "Tiếng Việt",
            Self::UKRAINIAN => "Українська",
            _ => return None,
        })
    }

    /// Create a locale from a set of bytes.
    const fn from_bytes(input: &[u8]) -> Self {
        Self(KnownString::from_bytes(input))
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Debug for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.get())
    }
}

impl Deref for Locale {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl FromStr for Locale {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl ToString for Locale {
    fn to_string(&self) -> String {
        KnownString::to_string(&self.0)
    }
}

impl TryFrom<&str> for Locale {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::Locale;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr, string::ToString};

    assert_impl_all!(
        Locale: AsRef<str>,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
        ToString,
        TryFrom<&'static str>,
    );

    const MAP: &[(Locale, &str, &str, &str)] = &[
        (Locale::BULGARIAN, "bg", "Bulgarian", "български"),
        (Locale::CHINESE_CHINA, "zh-CN", "Chinese, China", "中文"),
        (
            Locale::CHINESE_TAIWAN,
            "zh-TW",
            "Chinese, Taiwan",
            "繁體中文",
        ),
        (Locale::CROATIAN, "hr", "Croatian", "Hrvatski"),
        (Locale::CZECH, "cs", "Czech", "Čeština"),
        (Locale::DANISH, "da", "Danish", "Dansk"),
        (Locale::ENGLISH_UK, "en-GB", "English, UK", "English, UK"),
        (Locale::ENGLISH_US, "en-US", "English, US", "English, US"),
        (Locale::FINNISH, "fi", "Finnish", "Suomi"),
        (Locale::FRENCH, "fr", "French", "Français"),
        (Locale::GERMAN, "de", "German", "Deutsch"),
        (Locale::GREEK, "el", "Greek", "Ελληνικά"),
        (Locale::HINDI, "hi", "Hindi", "हिन्दी"),
        (Locale::HUNGARIAN, "hu", "Hungarian", "Magyar"),
        (Locale::INDONESIAN, "id", "Indonesian", "Bahasa Indonesia"),
        (Locale::ITALIAN, "it", "Italian", "Italiano"),
        (Locale::JAPANESE, "ja", "Japanese", "日本語"),
        (Locale::KOREAN, "ko", "Korean", "한국어"),
        (Locale::LITHUANIAN, "lt", "Lithuanian", "Lietuviškai"),
        (Locale::NETHERLANDS, "nl", "Dutch", "Nederlands"),
        (Locale::NORWEGIAN, "no", "Norwegian", "Norsk"),
        (Locale::POLISH, "pl", "Polish", "Polski"),
        (
            Locale::PORTUGUESE_BRAZILIAN,
            "pt-BR",
            "Portuguese, Brazilian",
            "Português do Brasil",
        ),
        (
            Locale::ROMANIAN_ROMANIA,
            "ro",
            "Romanian, Romania",
            "Română",
        ),
        (Locale::RUSSIAN, "ru", "Russian", "Pусский"),
        (Locale::SPANISH_SPAIN, "es-ES", "Spanish", "Español"),
        (Locale::SWEDISH_SWEDEN, "sv-SE", "Swedish", "Svenska"),
        (Locale::THAI, "th", "Thai", "ไทย"),
        (Locale::TURKISH, "tr", "Turkish", "Türkçe"),
        (Locale::VIETNAMESE, "vi", "Vietnamese", "Tiếng Việt"),
        (Locale::UKRAINIAN, "uk", "Ukrainian", "Українська"),
    ];

    #[test]
    fn variants() {
        for (locale, value, english_name, native_name) in MAP {
            serde_test::assert_tokens(
                locale,
                &[Token::NewtypeStruct { name: "Locale" }, Token::Str(value)],
            );
            assert_eq!(Some(*locale), Locale::new(value));
            assert_eq!(*value, locale.as_ref());
            assert_eq!(Ok(*locale), Locale::from_str(value));
            assert_eq!(Ok(*locale), Locale::try_from(*value));
            assert_eq!(value, &locale.to_string());
            assert_eq!(*value, locale.get());
            assert_eq!(Some(*english_name), locale.english_name());
            assert_eq!(Some(*native_name), locale.native_name());
        }
    }
}
