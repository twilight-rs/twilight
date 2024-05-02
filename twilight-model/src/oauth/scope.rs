//! Known list of available OAuth2 scopes.
//!
//! Refer to [Discord Docs/OAuth 2 Scopes] for a complete up-to-date list.
//!
//! [Discord Docs/OAuth 2 Scopes]: https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes

/// Allows your app to fetch data from a user's "Now Playing/Recently Played"
/// list.
///
/// Requires approval from Discord.
pub const ACTIVITIES_READ: &str = "activities.read";

/// Allows your app to update a user's activity
///
/// Requires approval from Discord, but is not required for the Game SDK
/// activity manager.
pub const ACTIVITIES_WRITE: &str = "activities.write";

/// Allows your app to read build data for a user's applications.
pub const APPLICATIONS_BUILDS_READ: &str = "applications.builds.read";

/// Allows your app to upload/update builds for a user's applications.
///
/// Requires approval from Discord.
pub const APPLICATIONS_BUILDS_UPLOAD: &str = "applications.builds.upload";

/// Allows your app to use commands in a guild.
pub const APPLICATIONS_COMMANDS: &str = "applications.commands";

/// Allows your app to update its commands using a Bearer token.
///
/// This is a client credentials grant only.
pub const APPLICATIONS_COMMANDS_UPDATE: &str = "applications.commands.update";

/// Allows your app to update permissions for its commands in a guild a user has
/// permissions to.
pub const APPLICATIONS_COMMANDS_PERMISSIONS_UPDATE: &str =
    "applications.commands.permissions.update";

/// Allows your app to read entitlements for a user's applications
pub const APPLICATIONS_ENTITLEMENTS: &str = "applications.entitlements";

/// Allows your app to read and update store data (SKUs, store listings,
/// achievements, etc.) for a user's applications.
pub const APPLICATIONS_STORE_UPDATE: &str = "applications.store.update";

/// For oauth2 bots, this puts the bot in the user's selected guild by default
pub const BOT: &str = "bot";

/// Allows /users/@me/connections to return linked third-party accounts
pub const CONNECTIONS: &str = "connections";

/// Allows your app to see information about the user's DMs and group DMs
///
/// Requires approval from Discord.
pub const DM_CHANNELS_READ: &str = "dm_channels.read";

/// Enables `GET /users/@me` returning an email.
pub const EMAIL: &str = "email";

/// Allows your app to join users to a group DM.
pub const GDM_JOIN: &str = "gdm.join";

/// Allows `GET /users/@me/guilds` to return basic information about all of a
/// user's guilds.
pub const GUILDS: &str = "guilds";

/// Allows `GET /guilds/{guild.id}/members/{user.id}` to be used for joining
/// users to a guild.
pub const GUILDS_JOIN: &str = "guilds.join";

/// Allows `GET /users/@me/guilds/{guild.id}/member` to return a user's member
/// information in a guild.
pub const GUILDS_MEMBERS_READ: &str = "guilds.members.read";

/// Allows `GET /users/@me`, but without the user's email.
pub const IDENTIFY: &str = "identify";

/// For local RPC server API access, this allows you to read messages from all
/// client channels (otherwise restricted to channels/guilds your app creates).
pub const MESSAGES_READ: &str = "messages.read";

/// Allows your app to know a user's friends and implicit relationships.
///
/// Requires approval from Discord.
pub const RELATIONSHIPS_READ: &str = "relationships.read";

/// Allows your app to update a user's connection and metadata for the app.
pub const ROLE_CONNECTIONS_WRITE: &str = "role_connections.write";

/// For local RPC server access, this allows you to control a user's local
/// Discord client.
///
/// Requires approval from Discord.
pub const RPC: &str = "rpc";

/// For local rpc server access, this allows you to update a user's activity
///
/// Requires approval from Discord.
pub const RPC_ACTIVITIES_WRITE: &str = "rpc.activities.write";

/// For local RPC server access, this allows you to receive notifications pushed
/// out to the user.
///
/// Requires approval from Discord.
pub const RPC_NOTIFICATIONS_READ: &str = "rpc.notifications.read";

/// For local RPC server access, this allows you to read a user's voice settings
/// and listen for voice events.
///
/// Requires approval from Discord.
pub const RPC_VOICE_READ: &str = "rpc.voice.read";

/// For local RPC server access, this allows you to update a user's voice
/// settings.
///
/// Requires approval from Discord.
pub const RPC_VOICE_WRITE: &str = "rpc.voice.write";

/// Allows your app to connect to voice on the user's behalf and see all the
/// voice members.
///
/// Requires approval from Discord.
pub const VOICE: &str = "voice";

/// This generates a webhook that is returned in the oauth token response for
/// authorization code grants.
pub const WEBHOOK_INCOMING: &str = "webhook.incoming";
