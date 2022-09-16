//! Markers for various resource types, such as channels or users.
//!
//! Markers themselves perform no logical action, and are only used to
//! ensure that IDs of incorrect types aren't used. If IDs were only 64-bit
//! integers then a role's ID may be erroneously used in the place of where
//! a user's ID is required; by using markers it can be ensured that only an
//! ID with a [`RoleMarker`] can be used where a role's ID is required.

// DEVELOPMENT: When adding a new marker, be sure to add its implementation to
// `util/snowflake`.

/// Marker for application IDs.
///
/// Types such as [`Message::application_id`] or [`Guild::application_id`]
/// use this ID marker.
///
/// [`Guild::application_id`]: crate::guild::Guild::application_id
/// [`Message::application_id`]: crate::channel::Message::application_id
#[derive(Debug)]
#[non_exhaustive]
pub struct ApplicationMarker;

/// Marker for attachment IDs.
///
/// Types such as [`Attachment`] use this ID marker.
///
/// [`Attachment`]: crate::channel::Attachment
#[derive(Debug)]
#[non_exhaustive]
pub struct AttachmentMarker;

/// Marker for audit log entry IDs.
///
/// Types such as [`AuditLogEntry`] use this ID marker.
///
/// [`AuditLogEntry`]: crate::guild::audit_log::AuditLogEntry
#[derive(Debug)]
#[non_exhaustive]
pub struct AuditLogEntryMarker;

/// Marker for auto moderation rule IDs.
///
/// Types such as [`AutoModerationRule`] use this ID marker.
///
/// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
#[derive(Debug)]
#[non_exhaustive]
pub struct AutoModerationRuleMarker;

/// Marker for channel IDs.
///
/// Types such as [`Channel`] or [`GatewayReaction`] use this ID marker.
///
/// [`Channel`]: crate::channel::Channel
/// [`GatewayReaction`]: crate::gateway::GatewayReaction
#[derive(Debug)]
#[non_exhaustive]
pub struct ChannelMarker;

/// Marker for command IDs.
///
/// Types such as [`Command`] use this ID marker.
///
/// [`Command`]: crate::application::command::Command
#[derive(Debug)]
#[non_exhaustive]
pub struct CommandMarker;

/// Marker for command versions.
///
/// Types such as [`Command`] use this ID marker.
///
/// [`Command`]: crate::application::command::Command
#[derive(Debug)]
#[non_exhaustive]
pub struct CommandVersionMarker;

/// Marker for emoji IDs.
///
/// Types such as [`Emoji`] or [`ReactionType`] use this ID marker.
///
/// [`Emoji`]: crate::guild::Emoji
/// [`ReactionType`]: crate::channel::message::ReactionType
#[derive(Debug)]
#[non_exhaustive]
pub struct EmojiMarker;

/// Marker for generic IDs.
///
/// Types such as [`AuditLogChange::Id`] or [`CommandOptionValue`] use this
/// ID marker.
///
/// [`AuditLogChange::Id`]: crate::guild::audit_log::AuditLogChange::Id
/// [`CommandOptionValue`]: crate::application::interaction::application_command::CommandOptionValue
#[derive(Debug)]
#[non_exhaustive]
pub struct GenericMarker;

/// Marker for guild IDs.
///
/// Types such as [`Guild`] or [`Message`] use this ID marker.
///
/// [`Guild`]: crate::guild::Guild
/// [`Message`]: crate::channel::Message
#[derive(Debug)]
#[non_exhaustive]
pub struct GuildMarker;

/// Marker for integration IDs.
///
/// Types such as [`GuildIntegration`] or [`RoleTags`] use this ID marker.
///
/// [`GuildIntegration`]: crate::guild::GuildIntegration
/// [`RoleTags`]: crate::guild::RoleTags
#[derive(Debug)]
#[non_exhaustive]
pub struct IntegrationMarker;

/// Marker for interaction IDs.
///
/// Types such as [`Interaction`] or [`MessageInteraction`] use this ID
/// marker.
///
/// [`Interaction`]: crate::application::interaction::Interaction
/// [`MessageInteraction`]: crate::channel::message::MessageInteraction
#[derive(Debug)]
#[non_exhaustive]
pub struct InteractionMarker;

/// Marker for message IDs.
///
/// Types such as [`Message`] or [`GatewayReaction`] use this ID marker.
///
/// [`Message`]: crate::channel::Message
/// [`GatewayReaction`]: crate::gateway::GatewayReaction
#[derive(Debug)]
#[non_exhaustive]
pub struct MessageMarker;

/// Marker for OAuth SKU IDs.
///
/// Types such as [`Application`] use this ID marker.
///
/// [`Application`]: crate::oauth::Application
#[derive(Debug)]
#[non_exhaustive]
pub struct OauthSkuMarker;

/// Marker for OAuth team IDs.
///
/// Types such as [`Team`] or [`TeamMember`] use this ID marker.
///
/// [`Team`]: crate::oauth::team::Team
/// [`TeamMember`]: crate::oauth::team::TeamMember
#[derive(Debug)]
#[non_exhaustive]
pub struct OauthTeamMarker;

/// Marker for role IDs.
///
/// Types such as [`Member`] or [`Role`] use this ID marker.
///
/// [`Member`]: crate::guild::Member
/// [`Role`]: crate::guild::Role
#[derive(Debug)]
#[non_exhaustive]
pub struct RoleMarker;

/// Marker for scheduled event IDs.
///
/// Types such as [`GuildScheduledEvent`] use this ID marker.
///
/// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
#[derive(Debug)]
#[non_exhaustive]
pub struct ScheduledEventMarker;

/// Marker for scheduled event entity IDs.
///
/// Types such as [`GuildScheduledEvent`] use this ID marker.
///
/// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
#[derive(Debug)]
#[non_exhaustive]
pub struct ScheduledEventEntityMarker;

/// Marker for stage IDs.
///
/// Types such as [`StageInstance`] use this ID marker.
///
/// [`StageInstance`]: crate::channel::StageInstance
#[derive(Debug)]
#[non_exhaustive]
pub struct StageMarker;

/// Marker for sticker banner asset IDs.
///
/// Types such as [`StickerPack`] use this ID marker.
///
/// [`StickerPack`]: crate::channel::message::sticker::StickerPack
#[derive(Debug)]
#[non_exhaustive]
pub struct StickerBannerAssetMarker;

/// Marker for sticker IDs.
///
/// Types such as [`Message`] or [`Sticker`] use this ID marker.
///
/// [`Message`]: crate::channel::Message
/// [`Sticker`]: crate::channel::message::sticker::Sticker
#[derive(Debug)]
#[non_exhaustive]
pub struct StickerMarker;

/// Marker for sticker pack IDs.
///
/// Types such as [`Sticker`] or [`StickerPack`] use this ID marker.
///
/// [`Sticker`]: crate::channel::message::sticker::Sticker
/// [`StickerPack`]: crate::channel::message::sticker::StickerPack
#[derive(Debug)]
#[non_exhaustive]
pub struct StickerPackMarker;

/// Marker for sticker pack SKU IDs.
///
/// Types such as [`StickerPack`] use this ID marker.
///
/// [`StickerPack`]: crate::channel::message::sticker::StickerPack
#[derive(Debug)]
#[non_exhaustive]
pub struct StickerPackSkuMarker;

/// Marker for forum tag IDs.
///
/// Types such as [`ForumTag`] use this ID marker.
///
/// [`ForumTag`]: crate::channel::forum::ForumTag
#[derive(Debug)]
#[non_exhaustive]
pub struct TagMarker;

/// Marker for user IDs.
///
/// Types such as [`Channel`] or [`User`] use this ID marker.
///
/// [`Channel`]: crate::channel::Channel
/// [`User`]: crate::user::User
#[derive(Debug)]
#[non_exhaustive]
pub struct UserMarker;

/// Marker for webhook IDs.
///
/// Types such as [`Webhook`] use this ID marker.
///
/// [`Webhook`]: crate::channel::webhook::Webhook
#[derive(Debug)]
#[non_exhaustive]
pub struct WebhookMarker;
