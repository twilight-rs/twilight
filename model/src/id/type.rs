use super::{
    marker::{
        ApplicationMarker, AttachmentMarker, AuditLogEntryMarker, ChannelMarker, CommandMarker,
        CommandVersionMarker, EmojiMarker, GenericMarker, GuildMarker, IntegrationMarker,
        InteractionMarker, MessageMarker, OauthSkuMarker, OauthTeamMarker, RoleMarker, StageMarker,
        StickerBannerAssetMarker, StickerMarker, StickerPackMarker, StickerPackSkuMarker,
        UserMarker, WebhookMarker,
    },
    Id,
};

/// ID with an application marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type ApplicationId = Id<ApplicationMarker>;

/// ID with an attachment marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type AttachmentId = Id<AttachmentMarker>;

/// ID with an audit log entry marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type AuditLogEntryId = Id<AuditLogEntryMarker>;

/// ID with a channel marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type ChannelId = Id<ChannelMarker>;

/// ID with a command marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type CommandId = Id<CommandMarker>;

/// ID with a command marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type CommandVersionId = Id<CommandVersionMarker>;

/// ID with an emoji marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type EmojiId = Id<EmojiMarker>;

/// ID with a generic marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type GenericId = Id<GenericMarker>;

/// ID with a guild marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type GuildId = Id<GuildMarker>;

/// ID with an integration marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type IntegrationId = Id<IntegrationMarker>;

/// ID with an interaction marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type InteractionId = Id<InteractionMarker>;

/// ID with a message marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type MessageId = Id<MessageMarker>;

/// ID with an OAuth SKU marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type OauthSkuId = Id<OauthSkuMarker>;

/// ID with an OAuth team marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type OauthTeamId = Id<OauthTeamMarker>;

/// ID with a role marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type RoleId = Id<RoleMarker>;

/// ID with a stage marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StageId = Id<StageMarker>;

/// ID with a sticker banner asset marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerBannerAssetId = Id<StickerBannerAssetMarker>;

/// ID with a sticker marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerId = Id<StickerMarker>;

/// ID with a sticker pack marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerPackId = Id<StickerPackMarker>;

/// ID with a sticker pack SKU marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type StickerPackSkuId = Id<StickerPackSkuMarker>;

/// ID with a user marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type UserId = Id<UserMarker>;

/// ID with a webhook marker.
#[deprecated(since = "0.9.0", note = "use `Id` and its marker types")]
pub type WebhookId = Id<WebhookMarker>;
