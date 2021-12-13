use super::{marker, Id};

/// ID with an application marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type ApplicationId = Id<marker::Application>;

/// ID with an attachment marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type AttachmentId = Id<marker::Attachment>;

/// ID with an audit log entry marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type AuditLogEntryId = Id<marker::AuditLogEntry>;

/// ID with a channel marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type ChannelId = Id<marker::Channel>;

/// ID with a command marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type CommandId = Id<marker::Command>;

/// ID with a command marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type CommandVersionId = Id<marker::CommandVersion>;

/// ID with an emoji marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type EmojiId = Id<marker::Emoji>;

/// ID with a generic marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type GenericId = Id<marker::Generic>;

/// ID with a guild marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type GuildId = Id<marker::Guild>;

/// ID with an integration marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type IntegrationId = Id<marker::Integration>;

/// ID with an interaction marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type InteractionId = Id<marker::Interaction>;

/// ID with a message marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type MessageId = Id<marker::Message>;

/// ID with an OAuth SKU marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type OauthSkuId = Id<marker::OauthSku>;

/// ID with an OAuth team marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type OauthTeamId = Id<marker::OauthTeam>;

/// ID with a role marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type RoleId = Id<marker::Role>;

/// ID with a stage marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StageId = Id<marker::Stage>;

/// ID with a sticker banner asset marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerBannerAssetId = Id<marker::StickerBannerAsset>;

/// ID with a sticker marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerId = Id<marker::Sticker>;

/// ID with a sticker pack marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerPackId = Id<marker::StickerPack>;

/// ID with a sticker pack SKU marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerPackSkuId = Id<marker::StickerPackSku>;

/// ID with a user marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type UserId = Id<marker::User>;

/// ID with a webhook marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type WebhookId = Id<marker::Webhook>;
