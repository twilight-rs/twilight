//! Events that Lavalink sends to clients.

/// The type of event that is coming in from a Lavalink message.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    /// Meta information about a track starting or ending.
    Event,
    /// An update about a player's current track.
    PlayerUpdate,
    /// Lavalink is connected and ready.
    Ready,
    /// Updated statistics about a node.
    Stats,
}

use serde::{Deserialize, Serialize};
use twilight_model::id::{Id, marker::GuildMarker};

/// The levels of severity that an exception can have.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    /// The cause is known and expected, indicates that there is nothing wrong
    /// with the library itself.
    Common,
    /// The probable cause is an issue with the library or there is no way to
    /// tell what the cause might be. This is the default level and other
    /// levels are used in cases where the thrower has more in-depth knowledge
    /// about the error.
    Fault,
    /// The cause might not be exactly known, but is possibly caused by outside
    /// factors. For example when an outside service responds in a format that
    /// we do not expect.
    Suspicious,
}

/// The exception with the details attached on what happened when making a query
/// to Lavalink.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Exception {
    /// The cause of the exception.
    pub cause: String,
    /// The message of the exception.
    pub message: Option<String>,
    /// The severity of the exception.
    pub severity: Severity,
    /// The full stack trace of the cause.
    pub cause_stack_trace: String,
}

/// An incoming event from a Lavalink node.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(tag = "op", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum IncomingEvent {
    /// Dispatched when player or voice events occur.
    Event(Event),
    /// Dispatched when you successfully connect to the Lavalink node.
    Ready(Ready),
    /// New statistics about a node and its host.
    Stats(Stats),
    /// An update about the information of a player.
    PlayerUpdate(PlayerUpdate),
}

impl From<Ready> for IncomingEvent {
    fn from(event: Ready) -> IncomingEvent {
        Self::Ready(event)
    }
}

impl From<Event> for IncomingEvent {
    fn from(event: Event) -> IncomingEvent {
        Self::Event(event)
    }
}

impl From<PlayerUpdate> for IncomingEvent {
    fn from(event: PlayerUpdate) -> IncomingEvent {
        Self::PlayerUpdate(event)
    }
}

impl From<Stats> for IncomingEvent {
    fn from(event: Stats) -> IncomingEvent {
        Self::Stats(event)
    }
}

/// The Discord voice information that Lavalink uses for connection and sending
/// information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct VoiceState {
    /// The Discord voice endpoint to connect to.
    pub endpoint: String,
    /// The Discord voice session id to authenticate with. Note this is separate
    /// from the lavalink session id.
    pub session_id: String,
    /// The Discord voice token to authenticate with.
    pub token: String,
}

/// An update of a player's status and state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlayerUpdate {
    /// The guild ID of the player.
    pub guild_id: Id<GuildMarker>,
    /// The new state of the player.
    pub state: PlayerUpdateState,
}

impl PlayerUpdate {
    /// The operation type of the `PlayerUpate` event.
    pub const OPCODE: Opcode = Opcode::PlayerUpdate;
}

/// New statistics about a node and its host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlayerUpdateState {
    /// True when the player is connected to the voice gateway.
    pub connected: bool,
    /// The ping of the node to the Discord voice server in milliseconds (-1 if not connected).
    pub ping: i64,
    /// Track position in milliseconds. None if not playing anything.
    pub position: i64,
    /// Unix timestamp of the player in milliseconds.
    pub time: i64,
}

/// Dispatched by Lavalink upon successful connection and authorization.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Ready {
    /// Whether this session was resumed.
    pub resumed: bool,
    /// The Lavalink session id of this connection. Not to be confused with a
    /// Discord voice session id.
    pub session_id: String,
}

/// Statistics about a node and its host.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// CPU information about the node's host.
    pub cpu: StatsCpu,
    /// The frame stats of the node. `null` if the node has no players or when
    /// retrieved via /v4/stats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_stats: Option<StatsFrame>,
    /// Memory information about the node's host.
    pub memory: StatsMemory,
    /// The current number of total players (active and not active) within
    /// the node.
    pub players: u64,
    /// The current number of active players within the node.
    pub playing_players: u64,
    /// The uptime of the Lavalink server in seconds.
    pub uptime: u64,
}

/// CPU information about a node and its host.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatsCpu {
    /// The number of CPU cores.
    pub cores: usize,
    /// The load of the Lavalink server.
    pub lavalink_load: f64,
    /// The load of the system as a whole.
    pub system_load: f64,
}

/// CPU information about a node and its host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatsFrame {
    /// The load of the system as a whole.
    pub deficit: i64,
    /// The load of the Lavalink server.
    pub nulled: i64,
    /// The number of CPU cores.
    pub sent: i64,
}

/// Memory information about a node and its host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatsMemory {
    /// The number of bytes allocated.
    pub allocated: u64,
    /// The number of bytes free.
    pub free: u64,
    /// The number of bytes reservable.
    pub reservable: u64,
    /// The number of bytes used.
    pub used: u64,
}

/// Information about the track returned or playing on Lavalink.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackInfo {
    /// The track artwork url.
    pub artwork_url: Option<String>,
    /// The track author.
    pub author: String,
    /// The track [ISRC](https://en.wikipedia.org/wiki/International_Standard_Recording_Code).
    pub isrc: Option<String>,
    /// The track identifier.
    pub identifier: String,
    /// Whether the track is seekable.
    pub is_seekable: bool,
    /// Whether the track is a stream.
    pub is_stream: bool,
    /// The track length in milliseconds.
    pub length: u64,
    /// The track position in milliseconds.
    pub position: u64,
    /// The track source name.
    pub source_name: String,
    /// The track title.
    pub title: String,
    /// The track uri.
    pub uri: Option<String>,
}

/// A track object for lavalink to consume and read.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// The base64 encoded track to play
    pub encoded: String,
    /// Info about the track
    pub info: TrackInfo,
}

/// Server dispatched an event. See the Event Types section for more information.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// The data of the event type.
    #[serde(flatten)]
    pub data: EventData,
    /// The guild id that this was received from.
    pub guild_id: String,
    /// The type of event.
    pub r#type: EventType,
}

/// The type of event being dispatched as a message from the server as the event
/// triggers.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum EventType {
    /// Dispatched when a track starts playing.
    TrackStartEvent,
    /// Dispatched when a track ends.
    TrackEndEvent,
    /// Dispatched when a track throws an exception.
    TrackExceptionEvent,
    /// Dispatched when a track gets stuck while playing.
    TrackStuckEvent,
    /// Dispatched when the websocket connection to Discord voice servers is closed.
    WebSocketClosedEvent,
}

/// The data of the server event that was dispatched when event triggers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventData {
    /// Dispatched when a track ends.
    TrackEndEvent(TrackEnd),
    /// Dispatched when a track throws an exception.
    TrackExceptionEvent(TrackException),
    /// Dispatched when a track gets stuck while playing.
    TrackStuckEvent(TrackStuck),
    /// Dispatched when a track starts playing.
    TrackStartEvent(TrackStart),
    /// Dispatched when the websocket connection to Discord voice servers is closed.
    WebSocketClosedEvent(WebSocketClosed),
}

/// The reason for the track ending.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum TrackEndReason {
    /// The track was cleaned up.
    Cleanup,
    /// The track finished playing.
    Finished,
    /// The track failed to load.
    LoadFailed,
    /// The track was replaced
    Replaced,
    /// The track was stopped.
    Stopped,
}

/// A track ended event from lavalink.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackEnd {
    /// The reason that the track ended.
    pub reason: TrackEndReason,
    /// The track that ended playing.
    pub track: Track,
}

/// A track started.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackStart {
    /// The track that started playing.
    pub track: Track,
}

/// Dispatched when a track throws an exception.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackException {
    /// The occurred exception.
    pub exception: Exception,
    /// The track that threw the exception.
    pub track: Track,
}

/// Dispatched when a track gets stuck while playing.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackStuck {
    /// The threshold in milliseconds that was exceeded.
    pub threshold_ms: u64,
    /// The track that got stuck.
    pub track: Track,
}

/// The voice websocket connection to Discord has been closed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct WebSocketClosed {
    /// True if Discord closed the connection, false if Lavalink closed it.
    pub by_remote: bool,
    /// [Discord websocket opcode](https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-close-event-codes)
    /// that closed the connection.
    pub code: u64,
    /// Reason the connection was closed.
    pub reason: String,
}
