use super::{CategoryFields, GuildChannelFields, RoleFields, TextFields, VoiceFields};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    guild::Permissions,
    id::{ChannelId, RoleId},
};

/// Error building role fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RoleFieldsError {
    /// Color was larger than a valid RGB hexadecimal value.
    ColorNotRgb {
        /// Provided color hex value.
        color: u32,
    },
    /// Invalid id for builders.
    IdInvalid,
}

impl Display for RoleFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ColorNotRgb { color } => {
                f.write_fmt(format_args!("the color {} is invalid", color))
            }
            Self::IdInvalid => f.write_str("the given id value is 1, which is not acceptable"),
        }
    }
}

impl Error for RoleFieldsError {}

/// A builder for role fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a role"]
pub struct RoleFieldsBuilder(RoleFields);

impl RoleFieldsBuilder {
    /// The maximumn accepted color value.
    ///
    /// This is used by [`color`].
    ///
    /// [`color`]: Self::color
    pub const COLOR_MAXIMUM: u32 = 0xff_ff_ff;

    const ROLE_ID: RoleId = RoleId(1);

    /// Create a new default role field builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self(RoleFields {
            color: None,
            hoist: None,
            id: Self::ROLE_ID,
            mentionable: None,
            name: name.into(),
            permissions: None,
            position: None,
        })
    }

    /// Build the role fields.
    pub fn build(self) -> RoleFields {
        self.0
    }

    /// Set the role color.
    ///
    /// This must be a valid hexadecimal RGB value.
    ///
    /// # Errors
    ///
    /// Returns [`RoleFieldsError::ColorNotRgb`] if the color is not valid RGB.
    pub fn color(mut self, color: u32) -> Result<Self, RoleFieldsError> {
        if color > Self::COLOR_MAXIMUM {
            return Err(RoleFieldsError::ColorNotRgb { color });
        }

        self.0.color.replace(color);

        Ok(self)
    }

    /// Show the role above other roles in the user list.
    pub fn hoist(mut self) -> Self {
        self.0.hoist.replace(true);

        self
    }

    /// Set the id of the role.
    ///
    /// # Errors
    ///
    /// Returns [`RoleFieldsError::IdInvalid`] if the id is set to 1.
    pub fn id(mut self, id: RoleId) -> Result<Self, RoleFieldsError> {
        if id == Self::ROLE_ID {
            return Err(RoleFieldsError::IdInvalid);
        }

        self.0.id = id;

        Ok(self)
    }

    /// Allow the role to be @mentioned.
    pub fn mentionable(mut self) -> Self {
        self.0.mentionable.replace(true);

        self
    }

    /// Set the permissions of the role.
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.0.permissions.replace(permissions);

        self
    }

    /// Set the position of the role.
    pub fn position(mut self, position: i64) -> Self {
        self.0.position.replace(position);

        self
    }
}

/// Error building text fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TextFieldsError {
    /// The name is too short.
    NameTooShort {
        /// The invalid name.
        name: String,
    },
    /// The name is too long.
    NameTooLong {
        /// The invalid name.
        name: String,
    },
    /// The rate limit is invalid.
    RateLimitInvalid {
        /// The incorrect rate limit.
        limit: u64,
    },
    /// The topic is too long.
    TopicTooLong {
        /// The incorrect topic.
        topic: String,
    },
}

impl Display for TextFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameTooShort { name } => {
                f.write_fmt(format_args!("the name is too short: {}", name.len()))
            }
            Self::NameTooLong { name } => {
                f.write_fmt(format_args!("the name is too long: {}", name.len()))
            }
            Self::RateLimitInvalid { limit } => {
                f.write_fmt(format_args!("the rate limit {} is invalid", limit))
            }
            Self::TopicTooLong { topic } => {
                f.write_fmt(format_args!("the topic is too long: {}", topic.len()))
            }
        }
    }
}

impl Error for TextFieldsError {}

/// A builder for text fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a text channel"]
pub struct TextFieldsBuilder(TextFields);

impl TextFieldsBuilder {
    /// The minimum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MIN_NAME_LENGTH: usize = 2;

    /// The maximum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MAX_NAME_LENGTH: usize = 100;

    /// The maximum length of a rate limit.
    ///
    /// This is used by [`rate_limit_per_user`].
    ///
    /// [`rate_limit_per_user`]: Self::rate_limit_per_user
    pub const MAX_RATE_LIMIT: u64 = 21600;

    /// The maximum number of UTF-16 code points that can be in a channel topic.
    ///
    /// This is used by [`topic`].
    ///
    /// [`topic`]: Self::topic
    pub const MAX_TOPIC_LENGTH: usize = 1024;

    /// Create a new text fields builder.
    ///
    /// # Errors
    ///
    /// Returns [`TextFieldsError::NameTooShort`] if the name is too short.
    ///
    /// Returns [`TextFieldsError::NameTooLong`] if the name is too long.
    pub fn new(name: impl Into<String>) -> Result<Self, TextFieldsError> {
        Self::_new(name.into())
    }

    fn _new(name: String) -> Result<Self, TextFieldsError> {
        if name.len() < Self::MIN_NAME_LENGTH {
            return Err(TextFieldsError::NameTooShort { name });
        }

        if name.len() > Self::MAX_NAME_LENGTH {
            return Err(TextFieldsError::NameTooLong { name });
        }

        Ok(Self(TextFields {
            id: ChannelId(1),
            kind: ChannelType::GuildText,
            name,
            nsfw: None,
            permission_overwrites: None,
            parent_id: None,
            rate_limit_per_user: None,
            topic: None,
        }))
    }

    /// Build the text fields.
    pub fn build(self) -> TextFields {
        self.0
    }

    /// Make the channel NSFW.
    pub fn nsfw(mut self) -> Self {
        self.0.nsfw.replace(true);

        self
    }

    /// Set the channel's permission overwrites.
    pub fn permission_overwrites(mut self, overwrites: Vec<PermissionOverwrite>) -> Self {
        self.0.permission_overwrites.replace(overwrites);

        self
    }

    /// Set the channel's rate limit per user.
    ///
    /// # Errors
    ///
    /// Returns [`TextFieldsError::RateLimitInvalid`] if the rate limit is invalid.
    pub fn rate_limit_per_user(mut self, limit: u64) -> Result<Self, TextFieldsError> {
        if limit > Self::MAX_RATE_LIMIT {
            return Err(TextFieldsError::RateLimitInvalid { limit });
        }

        self.0.rate_limit_per_user.replace(limit);

        Ok(self)
    }

    /// Set the channel's topic.
    ///
    /// # Errors
    ///
    /// Returns [`TextFieldsError::TopicTooLong`] if the topic is too long.
    pub fn topic(self, topic: impl Into<String>) -> Result<Self, TextFieldsError> {
        self._topic(topic.into())
    }

    fn _topic(mut self, topic: String) -> Result<Self, TextFieldsError> {
        if topic.len() > Self::MAX_TOPIC_LENGTH {
            return Err(TextFieldsError::TopicTooLong { topic });
        }

        self.0.topic.replace(topic);

        Ok(self)
    }
}

/// Error building voice fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum VoiceFieldsError {
    /// The name is too short.
    NameTooShort {
        /// The invalid name.
        name: String,
    },
    /// THe name is too long.
    NameTooLong {
        /// The invalid name.
        name: String,
    },
}

impl Display for VoiceFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameTooShort { name } => {
                f.write_fmt(format_args!("the name is too short: {}", name.len()))
            }
            Self::NameTooLong { name } => {
                f.write_fmt(format_args!("the name is too long: {}", name.len()))
            }
        }
    }
}

impl Error for VoiceFieldsError {}

/// A builder for voice fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a voice channel"]
pub struct VoiceFieldsBuilder(VoiceFields);

impl VoiceFieldsBuilder {
    /// The minimum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MIN_NAME_LENGTH: usize = 2;

    /// The maximum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MAX_NAME_LENGTH: usize = 100;

    /// Create a new voice fields builder.
    ///
    /// # Errors
    ///
    /// Returns [`VoiceFieldsError::NameTooShort`] if the name is too short.
    ///
    /// Returns [`VoiceFieldsError::NameTooLong`] if the name is too long.
    pub fn new(name: impl Into<String>) -> Result<Self, VoiceFieldsError> {
        Self::_new(name.into())
    }

    fn _new(name: String) -> Result<Self, VoiceFieldsError> {
        if name.len() < Self::MIN_NAME_LENGTH {
            return Err(VoiceFieldsError::NameTooShort { name });
        }

        if name.len() > Self::MAX_NAME_LENGTH {
            return Err(VoiceFieldsError::NameTooLong { name });
        }

        Ok(Self(VoiceFields {
            bitrate: None,
            id: ChannelId(1),
            kind: ChannelType::GuildVoice,
            name,
            permission_overwrites: None,
            parent_id: None,
            user_limit: None,
        }))
    }

    /// Build the voice fields.
    pub fn build(self) -> VoiceFields {
        self.0
    }

    /// Set the voice channel's bitrate.
    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.0.bitrate.replace(bitrate);

        self
    }

    /// Set the channel's permission overwrites.
    pub fn permission_overwrites(mut self, overwrites: Vec<PermissionOverwrite>) -> Self {
        self.0.permission_overwrites.replace(overwrites);

        self
    }

    /// Set the voice channel's user limit.
    pub fn user_limit(mut self, limit: u64) -> Self {
        self.0.user_limit.replace(limit);

        self
    }
}

/// Error creating category fields.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CategoryFieldsError {
    /// The name is too short.
    NameTooShort {
        /// The invalid name.
        name: String,
    },
    /// THe name is too long.
    NameTooLong {
        /// The invalid name.
        name: String,
    },
}

impl Display for CategoryFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameTooShort { name } => {
                f.write_fmt(format_args!("the name is too short: {}", name.len()))
            }
            Self::NameTooLong { name } => {
                f.write_fmt(format_args!("the name is too long: {}", name.len()))
            }
        }
    }
}

impl Error for CategoryFieldsError {}

/// A builder for a category channel, and its children.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a category channel"]
pub struct CategoryFieldsBuilder {
    fields: CategoryFields,
    channels: Vec<GuildChannelFields>,
}

impl CategoryFieldsBuilder {
    /// The minimum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MIN_NAME_LENGTH: usize = 2;

    /// The maximum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MAX_NAME_LENGTH: usize = 100;

    /// Create a new category fields builder.
    ///
    /// # Errors
    ///
    /// Returns [`CategoryFieldsError::NameTooShort`] if the name is too short.
    ///
    /// Returns [`CategoryFieldsError::NameTooLong`] if the name is too long.
    pub fn new(name: impl Into<String>) -> Result<Self, CategoryFieldsError> {
        Self::_new(name.into())
    }

    fn _new(name: String) -> Result<Self, CategoryFieldsError> {
        if name.len() < Self::MIN_NAME_LENGTH {
            return Err(CategoryFieldsError::NameTooShort { name });
        }

        if name.len() > Self::MAX_NAME_LENGTH {
            return Err(CategoryFieldsError::NameTooLong { name });
        }

        Ok(Self {
            fields: CategoryFields {
                id: ChannelId(1),
                name,
                kind: ChannelType::GuildCategory,
                permission_overwrites: None,
            },
            channels: Vec::new(),
        })
    }

    pub(super) fn build(mut self, id: ChannelId) -> Vec<GuildChannelFields> {
        for channel in &mut self.channels {
            match channel {
                GuildChannelFields::Text(t) => t.parent_id.replace(id),
                GuildChannelFields::Voice(v) => v.parent_id.replace(id),
                GuildChannelFields::Category(_) => None,
            };
        }

        self.channels.insert(
            0,
            GuildChannelFields::Category(CategoryFields { id, ..self.fields }),
        );

        self.channels
    }

    /// Add a child text channel.
    pub fn add_text(mut self, channel: impl Into<TextFields>) -> Self {
        self.channels.push(GuildChannelFields::Text(channel.into()));

        self
    }

    /// add a child voice channel.
    pub fn add_voice(mut self, channel: impl Into<VoiceFields>) -> Self {
        self.channels
            .push(GuildChannelFields::Voice(channel.into()));

        self
    }
}

/// A builder for a list of channels.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[must_use = "must be built into a list of channnels"]
pub struct GuildChannelFieldsBuilder(Vec<GuildChannelFields>);

impl GuildChannelFieldsBuilder {
    /// Create a new channels builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the list of channels.
    pub fn build(self) -> Vec<GuildChannelFields> {
        self.0
    }

    /// Add a text channel to the builder.
    pub fn add_text(mut self, channel: impl Into<TextFields>) -> Self {
        self.0.push(GuildChannelFields::Text(channel.into()));

        self
    }

    /// Add a voice channel to the builder.
    pub fn add_voice(mut self, channel: impl Into<VoiceFields>) -> Self {
        self.0.push(GuildChannelFields::Voice(channel.into()));

        self
    }

    /// Add a category channel builder, and all its children to the builder.
    pub fn add_category_builder(mut self, channel: CategoryFieldsBuilder) -> Self {
        let last_id = self
            .0
            .iter()
            .rev()
            .find(|c| matches!(c, GuildChannelFields::Category(_)))
            .map_or(ChannelId(1), |c| c.clone().id());

        let mut channels = channel.build(ChannelId(last_id.0 + 1));

        self.0.append(&mut channels);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{CategoryFields, GuildChannelFields, RoleFields, TextFields, VoiceFields},
        CategoryFieldsBuilder, CategoryFieldsError, GuildChannelFieldsBuilder, RoleFieldsBuilder,
        RoleFieldsError, TextFieldsBuilder, TextFieldsError, VoiceFieldsBuilder, VoiceFieldsError,
    };
    use twilight_model::{
        channel::{
            permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
            ChannelType,
        },
        guild::Permissions,
        id::{ChannelId, RoleId},
    };

    fn perms() -> Permissions {
        Permissions::CONNECT | Permissions::SPEAK | Permissions::SEND_TTS_MESSAGES
    }

    fn overwrite() -> PermissionOverwrite {
        PermissionOverwrite {
            allow: perms(),
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(RoleId(2)),
        }
    }

    fn voice() -> VoiceFieldsBuilder {
        VoiceFieldsBuilder::new("voicename")
            .unwrap()
            .bitrate(96_000)
            .permission_overwrites(vec![overwrite()])
            .user_limit(40)
    }

    #[test]
    fn test_role_fields() {
        assert_eq!(
            RoleFieldsError::ColorNotRgb { color: 123_123_123 },
            RoleFieldsBuilder::new("role")
                .color(123_123_123)
                .unwrap_err()
        );

        let fields = RoleFieldsBuilder::new("rolename")
            .color(0x12_34_56)
            .unwrap()
            .hoist()
            .id(RoleId(2))
            .unwrap()
            .mentionable()
            .permissions(Permissions::empty())
            .position(1);

        assert_eq!(
            fields.build(),
            RoleFields {
                color: Some(0x12_34_56),
                hoist: Some(true),
                id: RoleId(2),
                mentionable: Some(true),
                name: String::from("rolename"),
                permissions: Some(Permissions::empty()),
                position: Some(1),
            }
        );
    }

    #[test]
    fn test_voice_fields() {
        assert_eq!(
            VoiceFieldsError::NameTooShort {
                name: String::from("c")
            },
            VoiceFieldsBuilder::new("c").unwrap_err()
        );

        let fields = voice();

        assert_eq!(
            fields.build(),
            VoiceFields {
                bitrate: Some(96_000),
                id: ChannelId(1),
                kind: ChannelType::GuildVoice,
                name: String::from("voicename"),
                permission_overwrites: Some(vec![PermissionOverwrite {
                    allow: perms(),
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(RoleId(2)),
                }]),
                parent_id: None,
                user_limit: Some(40),
            }
        );
    }

    fn text() -> TextFieldsBuilder {
        TextFieldsBuilder::new("textname")
            .unwrap()
            .nsfw()
            .permission_overwrites(vec![overwrite()])
            .rate_limit_per_user(4_000)
            .unwrap()
            .topic("a topic")
            .unwrap()
    }

    #[test]
    fn test_text_fields() {
        assert_eq!(
            TextFieldsError::NameTooShort {
                name: String::from("b")
            },
            TextFieldsBuilder::new("b").unwrap_err()
        );

        let fields = text();

        assert_eq!(
            fields.build(),
            TextFields {
                id: ChannelId(1),
                kind: ChannelType::GuildText,
                name: String::from("textname"),
                nsfw: Some(true),
                permission_overwrites: Some(vec![PermissionOverwrite {
                    allow: perms(),
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(RoleId(2)),
                }]),
                parent_id: None,
                rate_limit_per_user: Some(4_000),
                topic: Some(String::from("a topic")),
            }
        );
    }

    fn category() -> CategoryFieldsBuilder {
        CategoryFieldsBuilder::new("category")
            .unwrap()
            .add_text(text())
            .add_voice(voice())
    }

    #[test]
    fn test_category_fields() {
        assert_eq!(
            CategoryFieldsError::NameTooShort {
                name: String::from("a")
            },
            CategoryFieldsBuilder::new("a").unwrap_err()
        );

        let fields = category();
        let channels = GuildChannelFieldsBuilder::new().add_category_builder(fields);

        assert_eq!(
            channels.build(),
            vec![
                GuildChannelFields::Category(CategoryFields {
                    id: ChannelId(2),
                    kind: ChannelType::GuildCategory,
                    name: String::from("category"),
                    permission_overwrites: None,
                }),
                GuildChannelFields::Text(TextFields {
                    id: ChannelId(1),
                    kind: ChannelType::GuildText,
                    name: String::from("textname"),
                    nsfw: Some(true),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Permissions::CONNECT
                            | Permissions::SPEAK
                            | Permissions::SEND_TTS_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Role(RoleId(2)),
                    }]),
                    parent_id: Some(ChannelId(2)),
                    rate_limit_per_user: Some(4_000),
                    topic: Some(String::from("a topic")),
                }),
                GuildChannelFields::Voice(VoiceFields {
                    bitrate: Some(96_000),
                    id: ChannelId(1),
                    kind: ChannelType::GuildVoice,
                    name: String::from("voicename"),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Permissions::CONNECT
                            | Permissions::SPEAK
                            | Permissions::SEND_TTS_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Role(RoleId(2)),
                    }]),
                    parent_id: Some(ChannelId(2)),
                    user_limit: Some(40),
                }),
            ]
        );
    }

    #[test]
    fn test_channels() {
        let channels = GuildChannelFieldsBuilder::new()
            .add_text(text())
            .add_voice(voice());

        assert_eq!(
            channels.build(),
            vec![
                GuildChannelFields::Text(TextFields {
                    id: ChannelId(1),
                    kind: ChannelType::GuildText,
                    name: String::from("textname"),
                    nsfw: Some(true),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Permissions::CONNECT
                            | Permissions::SPEAK
                            | Permissions::SEND_TTS_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Role(RoleId(2)),
                    }]),
                    parent_id: None,
                    rate_limit_per_user: Some(4_000),
                    topic: Some(String::from("a topic")),
                }),
                GuildChannelFields::Voice(VoiceFields {
                    bitrate: Some(96_000),
                    id: ChannelId(1),
                    kind: ChannelType::GuildVoice,
                    name: String::from("voicename"),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Permissions::CONNECT
                            | Permissions::SPEAK
                            | Permissions::SEND_TTS_MESSAGES,
                        deny: Permissions::empty(),
                        kind: PermissionOverwriteType::Role(RoleId(2)),
                    }]),
                    parent_id: None,
                    user_limit: Some(40),
                }),
            ]
        );
    }
}
