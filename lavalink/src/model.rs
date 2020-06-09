//! Models to (de)serialize incoming/outgoing websocket events and HTTP
//! responses.

use serde::{Deserialize, Serialize};

/// The type of event that something is.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    /// Destroy a player from a node.
    Destroy,
    /// Equalize a player.
    Equalizer,
    /// Meta information about a track starting or ending.
    Event,
    /// Pause a player.
    Pause,
    /// Add a track to a player's queue, optionally forcing it to play now.
    Play,
    /// An update about a player's current track.
    PlayerUpdate,
    /// Seek a player's active track to a new position.
    Seek,
    /// Updated statistics about a node.
    Stats,
    /// Stop a player.
    Stop,
    /// A combined voice server and voice state update.
    VoiceUpdate,
    /// Set the volume of a player.
    Volume,
}

mod outgoing {
    use super::Opcode;
    use serde::{Deserialize, Serialize};
    use twilight_model::{gateway::payload::VoiceServerUpdate, id::GuildId};

    /// An outgoing event to send to Lavalink.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(untagged)]
    pub enum OutgoingEvent {
        /// Destroy a player for a guild.
        Destroy(Destroy),
        /// Equalize a player.
        Equalizer(Equalizer),
        /// Pause or unpause a player.
        Pause(Pause),
        /// Add a track to the queue, optionally forcing it to play now.
        Play(Play),
        /// Seek a player's active track to a new position.
        Seek(Seek),
        /// Stop a player.
        Stop(Stop),
        /// A combined voice server and voice state update.
        VoiceUpdate(VoiceUpdate),
        /// Set the volume of a player.
        Volume(Volume),
    }

    impl From<Destroy> for OutgoingEvent {
        fn from(event: Destroy) -> OutgoingEvent {
            Self::Destroy(event)
        }
    }

    impl From<Equalizer> for OutgoingEvent {
        fn from(event: Equalizer) -> OutgoingEvent {
            Self::Equalizer(event)
        }
    }

    impl From<Pause> for OutgoingEvent {
        fn from(event: Pause) -> OutgoingEvent {
            Self::Pause(event)
        }
    }

    impl From<Play> for OutgoingEvent {
        fn from(event: Play) -> OutgoingEvent {
            Self::Play(event)
        }
    }

    impl From<Seek> for OutgoingEvent {
        fn from(event: Seek) -> OutgoingEvent {
            Self::Seek(event)
        }
    }

    impl From<Stop> for OutgoingEvent {
        fn from(event: Stop) -> OutgoingEvent {
            Self::Stop(event)
        }
    }

    impl From<VoiceUpdate> for OutgoingEvent {
        fn from(event: VoiceUpdate) -> OutgoingEvent {
            Self::VoiceUpdate(event)
        }
    }

    impl From<Volume> for OutgoingEvent {
        fn from(event: Volume) -> OutgoingEvent {
            Self::Volume(event)
        }
    }

    /// Destroy a player from a node.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Destroy {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
    }

    impl Destroy {
        /// Create a new destroy event.
        pub fn new(guild_id: GuildId) -> Self {
            Self {
                guild_id,
                op: Opcode::Destroy,
            }
        }
    }

    impl From<GuildId> for Destroy {
        fn from(guild_id: GuildId) -> Self {
            Self {
                guild_id,
                op: Opcode::Destroy,
            }
        }
    }

    /// Equalize a player.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Equalizer {
        /// The bands to use as part of the equalizer.
        pub bands: Vec<EqualizerBand>,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
    }

    impl Equalizer {
        /// Create a new equalizer event.
        pub fn new(guild_id: GuildId, bands: Vec<EqualizerBand>) -> Self {
            Self::from((guild_id, bands))
        }
    }

    impl From<(GuildId, Vec<EqualizerBand>)> for Equalizer {
        fn from((guild_id, bands): (GuildId, Vec<EqualizerBand>)) -> Self {
            Self {
                bands,
                guild_id,
                op: Opcode::Equalizer,
            }
        }
    }

    /// A band of the equalizer event.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct EqualizerBand {
        /// The band.
        pub band: i64,
        /// The gain.
        pub gain: f64,
    }

    impl EqualizerBand {
        /// Create a new equalizer band.
        pub fn new(band: i64, gain: f64) -> Self {
            Self::from((band, gain))
        }
    }

    impl From<(i64, f64)> for EqualizerBand {
        fn from((band, gain): (i64, f64)) -> Self {
            Self { band, gain }
        }
    }

    /// Pause or unpause a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Pause {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
        /// Whether to pause the player.
        ///
        /// Set to `true` to pause or `false` to resume.
        pub pause: bool,
    }

    impl Pause {
        /// Create a new pause event.
        ///
        /// Set to `true` to pause the player or `false` to resume it.
        pub fn new(guild_id: GuildId, pause: bool) -> Self {
            Self::from((guild_id, pause))
        }
    }

    impl From<(GuildId, bool)> for Pause {
        fn from((guild_id, pause): (GuildId, bool)) -> Self {
            Self {
                guild_id,
                op: Opcode::Pause,
                pause,
            }
        }
    }

    /// Add a track to the queue, optionally forcing it to play now.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Play {
        /// The position in milliseconds to end the track.
        ///
        /// This currently [does nothing] as of this writing.
        ///
        /// [does nothing]: https://github.com/Frederikam/Lavalink/issues/179
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<u64>,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// Whether or not to replace the currently playing track with this new
        /// track.
        ///
        /// Set to `true` to push this track to the end of the queue, or `false`
        /// to replace the current track in-place.
        pub no_replace: bool,
        /// The opcode of the event.
        pub op: Opcode,
        /// The position in milliseconds to start the track from.
        ///
        /// For example, set to 5000 to start the track 5 seconds in.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<u64>,
        /// The base64 track information.
        pub track: String,
    }

    impl Play {
        /// Create a new play event.
        pub fn new(
            guild_id: GuildId,
            track: impl Into<String>,
            start_time: impl Into<Option<u64>>,
            end_time: impl Into<Option<u64>>,
            no_replace: bool,
        ) -> Self {
            Self::from((guild_id, track, start_time, end_time, no_replace))
        }
    }

    impl<T: Into<String>> From<(GuildId, T)> for Play {
        fn from((guild_id, track): (GuildId, T)) -> Self {
            Self::from((guild_id, track, None, None, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>> From<(GuildId, T, S)> for Play {
        fn from((guild_id, track, start_time): (GuildId, T, S)) -> Self {
            Self::from((guild_id, track, start_time, None, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>> From<(GuildId, T, S, E)>
        for Play
    {
        fn from((guild_id, track, start_time, end_time): (GuildId, T, S, E)) -> Self {
            Self::from((guild_id, track, start_time, end_time, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>> From<(GuildId, T, S, E, bool)>
        for Play
    {
        fn from(
            (guild_id, track, start_time, end_time, no_replace): (GuildId, T, S, E, bool),
        ) -> Self {
            Self {
                end_time: end_time.into(),
                guild_id,
                no_replace,
                op: Opcode::Play,
                start_time: start_time.into(),
                track: track.into(),
            }
        }
    }

    /// Seek a player's active track to a new position.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Seek {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
        /// The position in milliseconds to seek to.
        pub position: i64,
    }

    impl Seek {
        /// Create a new seek event.
        pub fn new(guild_id: GuildId, position: i64) -> Self {
            Self::from((guild_id, position))
        }
    }

    impl From<(GuildId, i64)> for Seek {
        fn from((guild_id, position): (GuildId, i64)) -> Self {
            Self {
                guild_id,
                op: Opcode::Seek,
                position,
            }
        }
    }

    /// Stop a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Stop {
        /// The opcode of the event.
        pub op: Opcode,
        /// The guild ID of the player.
        pub guild_id: GuildId,
    }

    impl Stop {
        /// Create a new stop event.
        pub fn new(guild_id: GuildId) -> Self {
            Self::from(guild_id)
        }
    }

    impl From<GuildId> for Stop {
        fn from(guild_id: GuildId) -> Self {
            Self {
                guild_id,
                op: Opcode::Stop,
            }
        }
    }

    /// A combined voice server and voice state update.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct VoiceUpdate {
        /// The inner event being forwarded to a node.
        pub event: SlimVoiceServerUpdate,
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
        /// The session ID of the voice channel.
        pub session_id: String,
    }

    impl VoiceUpdate {
        /// Create a new voice update event.
        pub fn new(
            guild_id: GuildId,
            session_id: impl Into<String>,
            event: SlimVoiceServerUpdate,
        ) -> Self {
            Self::from((guild_id, session_id, event))
        }
    }

    impl<T: Into<String>> From<(GuildId, T, SlimVoiceServerUpdate)> for VoiceUpdate {
        fn from((guild_id, session_id, event): (GuildId, T, SlimVoiceServerUpdate)) -> Self {
            Self {
                event,
                guild_id,
                op: Opcode::VoiceUpdate,
                session_id: session_id.into(),
            }
        }
    }

    /// A slimmed version of a twilight voice server update.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "snake_case")]
    pub struct SlimVoiceServerUpdate {
        /// The endpoint of the Discord voice server.
        pub endpoint: Option<String>,
        /// The guild ID of the player.
        pub guild_id: Option<GuildId>,
        /// The authentication token used by the bot to connect to the Discord
        /// voice server.
        pub token: String,
    }

    impl From<VoiceServerUpdate> for SlimVoiceServerUpdate {
        fn from(update: VoiceServerUpdate) -> Self {
            Self {
                endpoint: update.endpoint,
                guild_id: update.guild_id,
                token: update.token,
            }
        }
    }

    /// Set the volume of a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Volume {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
        /// The volume of the player from 0 to 1000. 100 is the default.
        pub volume: i64,
    }

    impl Volume {
        /// Create a new volume event.
        pub fn new(guild_id: GuildId, volume: i64) -> Self {
            Self::from((guild_id, volume))
        }
    }

    impl From<(GuildId, i64)> for Volume {
        fn from((guild_id, volume): (GuildId, i64)) -> Self {
            Self {
                guild_id,
                op: Opcode::Volume,
                volume,
            }
        }
    }
}

mod incoming {
    use super::Opcode;
    use serde::{Deserialize, Serialize};
    use twilight_model::id::GuildId;

    /// An incoming event from a Lavalink node.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(untagged)]
    pub enum IncomingEvent {
        /// An update about the information of a player.
        PlayerUpdate(PlayerUpdate),
        /// New statistics about a node and its host.
        Stats(Stats),
        /// A track ended.
        TrackEnd(TrackEnd),
        /// A track started.
        TrackStart(TrackStart),
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

    /// An update about the information of a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerUpdate {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The opcode of the event.
        pub op: Opcode,
        /// The new state of the player.
        pub state: PlayerUpdateState,
    }

    /// New statistics about a node and its host.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerUpdateState {
        /// The new position of the player.
        pub position: i64,
        /// The new time of the player.
        pub time: i64,
    }

    /// Statistics about a node and its host.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Stats {
        /// CPU information about the node's host.
        pub cpu: StatsCpu,
        /// Statistics about audio frames.
        #[serde(rename = "frameStats")]
        pub frames: StatsFrames,
        /// Memory information about the node's host.
        pub memory: StatsMemory,
        /// The current number of total players (active and not active) within
        /// the node.
        pub players: u64,
        /// The current number of active players within the node.
        pub playing_players: u64,
        /// The opcode of the event.
        pub op: Opcode,
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
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct StatsFrames {
        /// The number of CPU cores.
        pub sent: u64,
        /// The load of the Lavalink server.
        pub nulled: u64,
        /// The load of the system as a whole.
        pub deficit: u64,
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

    /// The type of track event that was received.
    #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    pub enum TrackEventType {
        /// A track for a player ended.
        #[serde(rename = "TrackEndEvent")]
        End,
        /// A track for a player started.
        #[serde(rename = "TrackStartEvent")]
        Start,
    }

    /// A track ended.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct TrackEnd {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The opcode of the event.
        pub op: Opcode,
        /// The reason that the track ended.
        ///
        /// For example, this may be `"FINISHED"`.
        pub reason: String,
        /// The base64 track that was affected.
        pub track: String,
    }

    /// A track started.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct TrackStart {
        /// The guild ID of the player.
        pub guild_id: GuildId,
        /// The type of track event.
        #[serde(rename = "type")]
        pub kind: TrackEventType,
        /// The opcode of the event.
        pub op: Opcode,
        /// The base64 track that was affected.
        pub track: String,
    }
}

pub use self::{incoming::*, outgoing::*};

#[cfg(test)]
mod tests {
    use super::{
        incoming::{
            IncomingEvent, PlayerUpdate, PlayerUpdateState, Stats, StatsCpu, StatsFrames,
            StatsMemory, TrackEnd, TrackEventType, TrackStart,
        },
        outgoing::{
            Destroy, Equalizer, EqualizerBand, OutgoingEvent, Pause, Play, Seek,
            SlimVoiceServerUpdate, Stop, VoiceUpdate, Volume,
        },
    };
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        Destroy: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        EqualizerBand: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Equalizer: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        IncomingEvent: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        OutgoingEvent: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Pause: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        PlayerUpdateState: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        PlayerUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Play: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Seek: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        SlimVoiceServerUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Stats: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        StatsCpu: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        StatsFrames: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        StatsMemory: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Stop: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        TrackEnd: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        TrackEventType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        TrackStart: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        VoiceUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        Volume: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
}
