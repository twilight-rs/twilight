use super::{CategoryFields, GuildChannelFields, RoleFields, TextFields, VoiceFields};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::ChannelType,
    guild::Permissions,
    http::permission_overwrite::PermissionOverwrite,
    id::{
        Id,
        marker::{ChannelMarker, RoleMarker},
    },
};

/// Error building role fields.
#[derive(Debug)]
pub struct RoleFieldsError {
    kind: RoleFieldsErrorType,
}

impl RoleFieldsError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &RoleFieldsErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (RoleFieldsErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for RoleFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            RoleFieldsErrorType::ColorNotRgb { color } => {
                f.write_str("the color ")?;
                Display::fmt(color, f)?;

                f.write_str(" is invalid")
            }
            RoleFieldsErrorType::IdInvalid => {
                f.write_str("the given id value is 1, which is not acceptable")
            }
        }
    }
}

impl Error for RoleFieldsError {}

/// Type of [`RoleFieldsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum RoleFieldsErrorType {
    /// Color was larger than a valid RGB hexadecimal value.
    ColorNotRgb {
        /// Provided color hex value.
        color: u32,
    },
    /// Invalid id for builders.
    IdInvalid,
}

/// A builder for role fields.
#[derive(Debug)]
#[must_use = "must be built into a role"]
pub struct RoleFieldsBuilder(Result<RoleFields, RoleFieldsError>);

impl RoleFieldsBuilder {
    /// The maximum accepted color value.
    ///
    /// This is used by [`color`].
    ///
    /// [`color`]: Self::color
    pub const COLOR_MAXIMUM: u32 = 0xff_ff_ff;

    const ROLE_ID: Id<RoleMarker> = Id::new(1);

    /// Create a new default role field builder.
    pub const fn new(name: String) -> Self {
        Self(Ok(RoleFields {
            color: None,
            hoist: None,
            id: Self::ROLE_ID,
            mentionable: None,
            name,
            permissions: None,
            position: None,
        }))
    }

    /// Build the role fields.
    ///
    /// # Errors
    ///
    /// Returns a [`RoleFieldsErrorType::ColorNotRgb`] error type if the color
    /// is not valid RGB.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Result<RoleFields, RoleFieldsError> {
        self.0
    }

    /// Set the role color.
    ///
    /// This must be a valid hexadecimal RGB value. `0x000000` is ignored
    /// and doesn't count towards the final computed color in the user list.
    /// Refer to [`Self::COLOR_MAXIMUM`] for the maximum
    /// acceptable value.
    pub fn color(mut self, color: u32) -> Self {
        self.0 = self.0.and_then(|mut fields| {
            if color > Self::COLOR_MAXIMUM {
                return Err(RoleFieldsError {
                    kind: RoleFieldsErrorType::ColorNotRgb { color },
                });
            }

            fields.color.replace(color);

            Ok(fields)
        });

        self
    }

    /// Show the role above other roles in the user list.
    pub fn hoist(mut self) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.hoist = Some(true);
        }

        self
    }

    /// Set the id of the role.
    ///
    /// # Errors
    ///
    /// Returns a [`RoleFieldsErrorType::IdInvalid`] error type if the ID is set
    /// to 1.
    pub fn id(mut self, id: Id<RoleMarker>) -> Self {
        self.0 = self.0.and_then(|mut fields| {
            if id == Self::ROLE_ID {
                return Err(RoleFieldsError {
                    kind: RoleFieldsErrorType::IdInvalid,
                });
            }

            fields.id = id;

            Ok(fields)
        });

        self
    }

    /// Allow the role to be @mentioned.
    pub fn mentionable(mut self) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.mentionable = Some(true);
        }

        self
    }

    /// Set the permissions of the role.
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.permissions = Some(permissions);
        }

        self
    }

    /// Set the position of the role.
    pub fn position(mut self, position: i64) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.position = Some(position);
        }

        self
    }
}

/// Error building text fields.
#[derive(Debug)]
pub struct TextFieldsError {
    kind: TextFieldsErrorType,
}

impl TextFieldsError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &TextFieldsErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (TextFieldsErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for TextFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            TextFieldsErrorType::NameTooShort { name } => {
                f.write_str("the name is too short: ")?;

                Display::fmt(&name.len(), f)
            }
            TextFieldsErrorType::NameTooLong { name } => {
                f.write_str("the name is too long: ")?;

                Display::fmt(&name.len(), f)
            }
            TextFieldsErrorType::RateLimitInvalid { limit } => {
                f.write_str("the rate limit ")?;
                Display::fmt(limit, f)?;

                f.write_str(" is invalid")
            }
            TextFieldsErrorType::TopicTooLong { topic } => {
                f.write_str("the topic is too long: ")?;

                Display::fmt(&topic.len(), f)
            }
        }
    }
}

impl Error for TextFieldsError {}

/// Type of [`TextFieldsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum TextFieldsErrorType {
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
        limit: u16,
    },
    /// The topic is too long.
    TopicTooLong {
        /// The incorrect topic.
        topic: String,
    },
}

/// A builder for text fields.
#[derive(Debug)]
#[must_use = "must be built into a text channel"]
pub struct TextFieldsBuilder(Result<TextFields, TextFieldsError>);

impl TextFieldsBuilder {
    /// The minimum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MIN_NAME_LENGTH: usize = 1;

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
    pub const MAX_RATE_LIMIT: u16 = 21600;

    /// The maximum number of UTF-16 code points that can be in a channel topic.
    ///
    /// This is used by [`topic`].
    ///
    /// [`topic`]: Self::topic
    pub const MAX_TOPIC_LENGTH: usize = 1024;

    /// Create a new text fields builder.
    pub fn new(name: String) -> Self {
        let fields = Ok(TextFields {
            id: Id::new(1),
            kind: ChannelType::GuildText,
            name: String::new(),
            nsfw: None,
            permission_overwrites: None,
            parent_id: None,
            rate_limit_per_user: None,
            topic: None,
        })
        .and_then(|mut fields| {
            if name.len() < Self::MIN_NAME_LENGTH {
                return Err(TextFieldsError {
                    kind: TextFieldsErrorType::NameTooShort { name },
                });
            }

            if name.len() > Self::MAX_NAME_LENGTH {
                return Err(TextFieldsError {
                    kind: TextFieldsErrorType::NameTooLong { name },
                });
            }

            fields.name = name;

            Ok(fields)
        });

        Self(fields)
    }

    /// Build the text fields.
    ///
    /// # Errors
    ///
    /// Returns a [`TextFieldsErrorType::NameTooShort`] error type if the name
    /// is too short.
    ///
    /// Returns a [`TextFieldsErrorType::NameTooLong`] error type if the name is
    /// too long.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Result<TextFields, TextFieldsError> {
        self.0
    }

    /// Make the channel NSFW.
    pub fn nsfw(mut self) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.nsfw = Some(true);
        }

        self
    }

    /// Set the channel's permission overwrites.
    pub fn permission_overwrites(mut self, overwrites: Vec<PermissionOverwrite>) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.permission_overwrites.replace(overwrites);
        }

        self
    }

    /// Set the channel's rate limit per user.
    ///
    /// # Errors
    ///
    /// Returns a [`TextFieldsErrorType::RateLimitInvalid`] error type if the
    /// rate limit is invalid.
    pub fn rate_limit_per_user(mut self, limit: u16) -> Self {
        self.0 = self.0.and_then(|mut fields| {
            if limit > Self::MAX_RATE_LIMIT {
                return Err(TextFieldsError {
                    kind: TextFieldsErrorType::RateLimitInvalid { limit },
                });
            }

            fields.rate_limit_per_user.replace(limit);

            Ok(fields)
        });

        self
    }

    /// Set the channel's topic.
    ///
    /// # Errors
    ///
    /// Returns a [`TextFieldsErrorType::TopicTooLong`] error type if the topic
    /// is too long.
    pub fn topic(mut self, topic: String) -> Self {
        self.0 = self.0.and_then(|mut fields| {
            if topic.len() > Self::MAX_TOPIC_LENGTH {
                return Err(TextFieldsError {
                    kind: TextFieldsErrorType::TopicTooLong { topic },
                });
            }

            fields.topic.replace(topic);

            Ok(fields)
        });

        self
    }
}

/// Error building voice fields.
#[derive(Debug)]
pub struct VoiceFieldsError {
    kind: VoiceFieldsErrorType,
}

impl VoiceFieldsError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &VoiceFieldsErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (VoiceFieldsErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for VoiceFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            VoiceFieldsErrorType::NameTooShort { name } => {
                f.write_str("the name is too short: ")?;

                Display::fmt(&name.len(), f)
            }
            VoiceFieldsErrorType::NameTooLong { name } => {
                f.write_str("the name is too long: ")?;

                Display::fmt(&name.len(), f)
            }
        }
    }
}

impl Error for VoiceFieldsError {}

/// Type of [`VoiceFieldsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum VoiceFieldsErrorType {
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
}

/// A builder for voice fields.
#[derive(Debug)]
#[must_use = "must be built into a voice channel"]
pub struct VoiceFieldsBuilder(Result<VoiceFields, VoiceFieldsError>);

impl VoiceFieldsBuilder {
    /// The minimum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MIN_NAME_LENGTH: usize = 1;

    /// The maximum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MAX_NAME_LENGTH: usize = 100;

    /// Create a new voice fields builder.
    pub fn new(name: String) -> Self {
        let fields = Ok(VoiceFields {
            bitrate: None,
            id: Id::new(1),
            kind: ChannelType::GuildVoice,
            name: String::new(),
            permission_overwrites: None,
            parent_id: None,
            user_limit: None,
        })
        .and_then(|mut fields| {
            if name.len() < Self::MIN_NAME_LENGTH {
                return Err(VoiceFieldsError {
                    kind: VoiceFieldsErrorType::NameTooShort { name },
                });
            }

            if name.len() > Self::MAX_NAME_LENGTH {
                return Err(VoiceFieldsError {
                    kind: VoiceFieldsErrorType::NameTooLong { name },
                });
            }

            fields.name = name;

            Ok(fields)
        });

        Self(fields)
    }

    /// Build the voice fields.
    ///
    /// # Errors
    ///
    /// Returns a [`VoiceFieldsErrorType::NameTooShort`] error type if the name
    /// is too short.
    ///
    /// Returns a [`VoiceFieldsErrorType::NameTooLong`] error type if the name
    /// is too long.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Result<VoiceFields, VoiceFieldsError> {
        self.0
    }

    /// Set the voice channel's bitrate.
    pub fn bitrate(mut self, bitrate: u32) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.bitrate = Some(bitrate);
        }

        self
    }

    /// Set the channel's permission overwrites.
    pub fn permission_overwrites(mut self, overwrites: Vec<PermissionOverwrite>) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.permission_overwrites.replace(overwrites);
        }

        self
    }

    /// Set the voice channel's user limit.
    pub fn user_limit(mut self, limit: u16) -> Self {
        if let Ok(fields) = self.0.as_mut() {
            fields.user_limit = Some(limit);
        }

        self
    }
}

/// Error creating category fields.
#[derive(Debug)]
pub struct CategoryFieldsError {
    kind: CategoryFieldsErrorType,
}

impl CategoryFieldsError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CategoryFieldsErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CategoryFieldsErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for CategoryFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CategoryFieldsErrorType::NameTooShort { name } => {
                f.write_str("the name is too short: ")?;

                Display::fmt(&name.len(), f)
            }
            CategoryFieldsErrorType::NameTooLong { name } => {
                f.write_str("the name is too long: ")?;

                Display::fmt(&name.len(), f)
            }
        }
    }
}

impl Error for CategoryFieldsError {}

/// Type of [`CategoryFieldsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CategoryFieldsErrorType {
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
}

/// A builder for a category channel, and its children.
#[derive(Debug)]
#[must_use = "must be built into a category channel"]
pub struct CategoryFieldsBuilder {
    fields: Result<CategoryFields, CategoryFieldsError>,
    channels: Vec<GuildChannelFields>,
}

impl CategoryFieldsBuilder {
    /// The minimum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MIN_NAME_LENGTH: usize = 1;

    /// The maximum number of UTF-16 code points that can be in a channel name.
    ///
    /// This is used by [`new`].
    ///
    /// [`new`]: Self::new
    pub const MAX_NAME_LENGTH: usize = 100;

    /// Create a new category fields builder.
    pub fn new(name: String) -> Self {
        let fields = Ok(CategoryFields {
            id: Id::new(1),
            name: String::new(),
            kind: ChannelType::GuildCategory,
            permission_overwrites: None,
        })
        .and_then(|mut fields| {
            if name.len() < Self::MIN_NAME_LENGTH {
                return Err(CategoryFieldsError {
                    kind: CategoryFieldsErrorType::NameTooShort { name },
                });
            }

            if name.len() > Self::MAX_NAME_LENGTH {
                return Err(CategoryFieldsError {
                    kind: CategoryFieldsErrorType::NameTooLong { name },
                });
            }

            fields.name = name;

            Ok(fields)
        });

        Self {
            fields,
            channels: Vec::new(),
        }
    }

    /// Build the category fields.
    ///
    /// # Errors
    ///
    /// Returns a [`CategoryFieldsErrorType::NameTooShort`] error type if the
    /// name is too short.
    ///
    /// Returns a [`CategoryFieldsErrorType::NameTooLong`] error type if the
    /// name is too long.
    pub(super) fn build(
        mut self,
        id: Id<ChannelMarker>,
    ) -> Result<Vec<GuildChannelFields>, CategoryFieldsError> {
        let fields = self.fields?;

        for channel in &mut self.channels {
            match channel {
                GuildChannelFields::Text(t) => t.parent_id.replace(id),
                GuildChannelFields::Voice(v) => v.parent_id.replace(id),
                GuildChannelFields::Category(_) => None,
            };
        }

        self.channels.insert(
            0,
            GuildChannelFields::Category(CategoryFields { id, ..fields }),
        );

        Ok(self.channels)
    }

    /// Add a child text channel.
    pub fn add_text(mut self, channel: TextFields) -> Self {
        self.channels.push(GuildChannelFields::Text(channel));

        self
    }

    /// add a child voice channel.
    pub fn add_voice(mut self, channel: VoiceFields) -> Self {
        self.channels.push(GuildChannelFields::Voice(channel));

        self
    }
}

/// A builder for a list of channels.
#[derive(Debug)]
#[must_use = "must be built into a list of channels"]
pub struct GuildChannelFieldsBuilder(Result<Vec<GuildChannelFields>, CategoryFieldsError>);

impl GuildChannelFieldsBuilder {
    /// Create a new channels builder.
    pub const fn new() -> Self {
        Self(Ok(Vec::new()))
    }

    /// Build the list of channels.
    ///
    /// # Errors
    ///
    /// Returns a [`CategoryFieldsErrorType::NameTooShort`] error type if the
    /// name of a category is too short.
    ///
    /// Returns a [`CategoryFieldsErrorType::NameTooLong`] error type if the
    /// name of a category is too long.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Result<Vec<GuildChannelFields>, CategoryFieldsError> {
        self.0
    }

    /// Add a text channel to the builder.
    pub fn add_text(mut self, channel: TextFields) -> Self {
        if let Ok(list) = self.0.as_mut() {
            list.push(GuildChannelFields::Text(channel));
        }

        self
    }

    /// Add a voice channel to the builder.
    pub fn add_voice(mut self, channel: VoiceFields) -> Self {
        if let Ok(list) = self.0.as_mut() {
            list.push(GuildChannelFields::Voice(channel));
        }

        self
    }

    /// Add a category channel builder, and all its children to the builder.
    pub fn add_category_builder(mut self, channel: CategoryFieldsBuilder) -> Self {
        self.0 = self.0.and_then(|mut list| {
            let last_id = list
                .iter()
                .rev()
                .find(|c| matches!(c, GuildChannelFields::Category(_)))
                .map_or(Id::new(1), GuildChannelFields::id);

            let mut channels = channel.build(Id::new(last_id.get() + 1))?;

            list.append(&mut channels);

            Ok(list)
        });

        self
    }
}

impl Default for GuildChannelFieldsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{CategoryFields, GuildChannelFields, RoleFields, TextFields, VoiceFields},
        CategoryFieldsBuilder, CategoryFieldsErrorType, GuildChannelFieldsBuilder,
        RoleFieldsBuilder, RoleFieldsErrorType, TextFieldsBuilder, TextFieldsErrorType,
        VoiceFieldsBuilder, VoiceFieldsErrorType,
    };
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::{
        channel::ChannelType,
        guild::Permissions,
        http::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
        id::Id,
    };

    assert_impl_all!(RoleFieldsBuilder: Debug, Send, Sync);
    assert_impl_all!(TextFieldsBuilder: Debug, Send, Sync);
    assert_impl_all!(VoiceFieldsBuilder: Debug, Send, Sync);
    assert_impl_all!(CategoryFieldsBuilder: Debug, Send, Sync);
    assert_impl_all!(GuildChannelFieldsBuilder: Debug, Send, Sync);

    fn perms() -> Permissions {
        Permissions::CONNECT | Permissions::SPEAK | Permissions::SEND_TTS_MESSAGES
    }

    fn overwrite() -> PermissionOverwrite {
        PermissionOverwrite {
            allow: Some(perms()),
            deny: Some(Permissions::empty()),
            id: Id::new(2),
            kind: PermissionOverwriteType::Role,
        }
    }

    fn voice() -> VoiceFields {
        VoiceFieldsBuilder::new("voicename".to_owned())
            .bitrate(96_000)
            .permission_overwrites(vec![overwrite()])
            .user_limit(40)
            .build()
            .unwrap()
    }

    #[test]
    fn role_fields() {
        assert!(matches!(
            RoleFieldsBuilder::new("role".to_owned())
                .color(123_123_123)
                .build()
                .unwrap_err()
                .kind(),
            RoleFieldsErrorType::ColorNotRgb { color: 123_123_123 },
        ));

        let fields = RoleFieldsBuilder::new("rolename".to_owned())
            .color(0x12_34_56)
            .hoist()
            .id(Id::new(2))
            .mentionable()
            .permissions(Permissions::empty())
            .position(1)
            .build()
            .unwrap();

        assert_eq!(
            fields,
            RoleFields {
                color: Some(0x12_34_56),
                hoist: Some(true),
                id: Id::new(2),
                mentionable: Some(true),
                name: String::from("rolename"),
                permissions: Some(Permissions::empty()),
                position: Some(1),
            }
        );
    }

    #[test]
    fn voice_fields() {
        assert!(matches!(
            VoiceFieldsBuilder::new(String::new()).build().unwrap_err().kind(),
            VoiceFieldsErrorType::NameTooShort { name }
            if name.is_empty()
        ));

        assert_eq!(
            voice(),
            VoiceFields {
                bitrate: Some(96_000),
                id: Id::new(1),
                kind: ChannelType::GuildVoice,
                name: String::from("voicename"),
                permission_overwrites: Some(vec![PermissionOverwrite {
                    allow: Some(perms()),
                    deny: Some(Permissions::empty()),
                    id: Id::new(2),
                    kind: PermissionOverwriteType::Role,
                }]),
                parent_id: None,
                user_limit: Some(40),
            }
        );
    }

    fn text() -> TextFields {
        TextFieldsBuilder::new("textname".to_owned())
            .nsfw()
            .permission_overwrites(vec![overwrite()])
            .rate_limit_per_user(4_000)
            .topic("a topic".to_owned())
            .build()
            .unwrap()
    }

    #[test]
    fn text_fields() {
        assert!(matches!(
            TextFieldsBuilder::new(String::new()).build().unwrap_err().kind(),
            TextFieldsErrorType::NameTooShort { name }
            if name.is_empty()
        ));

        assert_eq!(
            text(),
            TextFields {
                id: Id::new(1),
                kind: ChannelType::GuildText,
                name: String::from("textname"),
                nsfw: Some(true),
                permission_overwrites: Some(vec![PermissionOverwrite {
                    allow: Some(perms()),
                    deny: Some(Permissions::empty()),
                    id: Id::new(2),
                    kind: PermissionOverwriteType::Role
                }]),
                parent_id: None,
                rate_limit_per_user: Some(4_000),
                topic: Some(String::from("a topic")),
            }
        );
    }

    fn category() -> CategoryFieldsBuilder {
        CategoryFieldsBuilder::new("category".to_owned())
            .add_text(text())
            .add_voice(voice())
    }

    #[test]
    fn category_fields() {
        assert!(matches!(
            CategoryFieldsBuilder::new(String::new()).build(Id::new(4)).unwrap_err().kind(),
            CategoryFieldsErrorType::NameTooShort { name }
            if name.is_empty()
        ));

        let fields = category();
        let channels = GuildChannelFieldsBuilder::new().add_category_builder(fields);

        assert_eq!(
            channels.build().unwrap(),
            vec![
                GuildChannelFields::Category(CategoryFields {
                    id: Id::new(2),
                    kind: ChannelType::GuildCategory,
                    name: String::from("category"),
                    permission_overwrites: None,
                }),
                GuildChannelFields::Text(TextFields {
                    id: Id::new(1),
                    kind: ChannelType::GuildText,
                    name: String::from("textname"),
                    nsfw: Some(true),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Some(
                            Permissions::CONNECT
                                | Permissions::SPEAK
                                | Permissions::SEND_TTS_MESSAGES
                        ),
                        deny: Some(Permissions::empty()),
                        id: Id::new(2),
                        kind: PermissionOverwriteType::Role,
                    }]),
                    parent_id: Some(Id::new(2)),
                    rate_limit_per_user: Some(4_000),
                    topic: Some(String::from("a topic")),
                }),
                GuildChannelFields::Voice(VoiceFields {
                    bitrate: Some(96_000),
                    id: Id::new(1),
                    kind: ChannelType::GuildVoice,
                    name: String::from("voicename"),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Some(
                            Permissions::CONNECT
                                | Permissions::SPEAK
                                | Permissions::SEND_TTS_MESSAGES
                        ),
                        deny: Some(Permissions::empty()),
                        id: Id::new(2),
                        kind: PermissionOverwriteType::Role,
                    }]),
                    parent_id: Some(Id::new(2)),
                    user_limit: Some(40),
                }),
            ]
        );
    }

    #[test]
    fn channels() {
        let channels = GuildChannelFieldsBuilder::new()
            .add_text(text())
            .add_voice(voice());

        assert_eq!(
            channels.build().unwrap(),
            vec![
                GuildChannelFields::Text(TextFields {
                    id: Id::new(1),
                    kind: ChannelType::GuildText,
                    name: String::from("textname"),
                    nsfw: Some(true),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Some(
                            Permissions::CONNECT
                                | Permissions::SPEAK
                                | Permissions::SEND_TTS_MESSAGES
                        ),
                        deny: Some(Permissions::empty()),
                        id: Id::new(2),
                        kind: PermissionOverwriteType::Role,
                    }]),
                    parent_id: None,
                    rate_limit_per_user: Some(4_000),
                    topic: Some(String::from("a topic")),
                }),
                GuildChannelFields::Voice(VoiceFields {
                    bitrate: Some(96_000),
                    id: Id::new(1),
                    kind: ChannelType::GuildVoice,
                    name: String::from("voicename"),
                    permission_overwrites: Some(vec![PermissionOverwrite {
                        allow: Some(
                            Permissions::CONNECT
                                | Permissions::SPEAK
                                | Permissions::SEND_TTS_MESSAGES
                        ),
                        deny: Some(Permissions::empty()),
                        id: Id::new(2),
                        kind: PermissionOverwriteType::Role,
                    }]),
                    parent_id: None,
                    user_limit: Some(40),
                }),
            ]
        );
    }
}
