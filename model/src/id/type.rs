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
pub type ApplicationId = Id<ApplicationMarker>;

/// ID with an attachment marker.
pub type AttachmentId = Id<AttachmentMarker>;

/// ID with an audit log entry marker.
pub type AuditLogEntryId = Id<AuditLogEntryMarker>;

/// ID with a channel marker.
pub type ChannelId = Id<ChannelMarker>;

/// ID with a command marker.
pub type CommandId = Id<CommandMarker>;

/// ID with a command marker.
pub type CommandVersionId = Id<CommandVersionMarker>;

/// ID with an emoji marker.
pub type EmojiId = Id<EmojiMarker>;

/// ID with a generic marker.
pub type GenericId = Id<GenericMarker>;

/// ID with a guild marker.
pub type GuildId = Id<GuildMarker>;

/// ID with an integration marker.
pub type IntegrationId = Id<IntegrationMarker>;

/// ID with an interaction marker.
pub type InteractionId = Id<InteractionMarker>;

/// ID with a message marker.
pub type MessageId = Id<MessageMarker>;

/// ID with an OAuth SKU marker.
pub type OauthSkuId = Id<OauthSkuMarker>;

/// ID with an OAuth team marker.
pub type OauthTeamId = Id<OauthTeamMarker>;

/// ID with a role marker.
pub type RoleId = Id<RoleMarker>;

/// ID with a stage marker.
pub type StageId = Id<StageMarker>;

/// ID with a sticker banner asset marker.
pub type StickerBannerAssetId = Id<StickerBannerAssetMarker>;

/// ID with a sticker marker.
pub type StickerId = Id<StickerMarker>;

/// ID with a sticker pack marker.
pub type StickerPackId = Id<StickerPackMarker>;

/// ID with a sticker pack SKU marker.
pub type StickerPackSkuId = Id<StickerPackSkuMarker>;

/// ID with a user marker.
pub type UserId = Id<UserMarker>;

/// ID with a webhook marker.
pub type WebhookId = Id<WebhookMarker>;
