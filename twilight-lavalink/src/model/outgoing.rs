//! Events that clients send to Lavalink.
use serde::{Deserialize, Serialize};
use twilight_model::{
    gateway::payload::incoming::VoiceServerUpdate,
    id::{Id, marker::GuildMarker},
};

/// The track on the player. The encoded and identifier are mutually exclusive.
/// We don't support userData field currently.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlayerTrack {
    /// The string of the track to play.
    #[serde(flatten)]
    pub track_string: TrackOption,
}

/// Used to play a specific track. These are mutually exclusive.
/// When identifier is used, Lavalink will try to resolve the identifier as a
/// single track. An HTTP 400 error is returned when resolving a playlist,
/// search result, or no tracks.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TrackOption {
    /// The base64 encoded track to play. null stops the current track.
    Encoded(Option<String>),
    /// The identifier of the track to play.
    Identifier(String),
}

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
    /// Play a track.
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

impl OutgoingEvent {
    /// ID of the destination guild of this event.
    pub const fn guild_id(&self) -> Id<GuildMarker> {
        match self {
            Self::VoiceUpdate(voice_update) => voice_update.guild_id,
            Self::Play(play) => play.guild_id,
            Self::Destroy(destroy) => destroy.guild_id,
            Self::Equalizer(equalize) => equalize.guild_id,
            Self::Pause(pause) => pause.guild_id,
            Self::Seek(seek) => seek.guild_id,
            Self::Stop(stop) => stop.guild_id,
            Self::Volume(volume) => volume.guild_id,
        }
    }

    /// Whether this event replaces the currently playing track.
    pub(crate) const fn no_replace(&self) -> bool {
        match self {
            Self::Play(play) => play.no_replace,
            Self::Stop(_) => false,
            _ => true,
        }
    }
}

/// Destroy a player from a node.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Destroy {
    /// The guild ID of the player.
    pub guild_id: Id<GuildMarker>,
}

impl Destroy {
    /// Create a new destroy event.
    pub const fn new(guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id }
    }
}

impl From<Id<GuildMarker>> for Destroy {
    fn from(guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id }
    }
}

/// Filters to pass to the update player endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum Filters {
    /// Adjusts 15 different bands
    Equalizer(Equalizer),
}

/// Equalize a player.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Equalizer {
    /// The bands to use as part of the equalizer.
    pub equalizer: Vec<EqualizerBand>,
    /// The guild ID of the player.
    #[serde(skip_serializing)]
    pub guild_id: Id<GuildMarker>,
}

impl Equalizer {
    /// Create a new equalizer event.
    pub fn new(guild_id: Id<GuildMarker>, bands: Vec<EqualizerBand>) -> Self {
        Self::from((guild_id, bands))
    }
}

impl From<(Id<GuildMarker>, Vec<EqualizerBand>)> for Equalizer {
    fn from((guild_id, bands): (Id<GuildMarker>, Vec<EqualizerBand>)) -> Self {
        Self {
            equalizer: bands,
            guild_id,
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
    pub guild_id: Id<GuildMarker>,
    /// Whether to pause the player.
    ///
    /// Set to `true` to pause or `false` to resume.
    pub paused: bool,
}

impl Pause {
    /// Create a new pause event.
    ///
    /// Set to `true` to pause the player or `false` to resume it.
    pub fn new(guild_id: Id<GuildMarker>, pause: bool) -> Self {
        Self::from((guild_id, pause))
    }
}

impl From<(Id<GuildMarker>, bool)> for Pause {
    fn from((guild_id, pause): (Id<GuildMarker>, bool)) -> Self {
        Self {
            guild_id,
            paused: pause,
        }
    }
}

/// Play a track, optionally specifying to not skip the current track.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Play {
    /// The position in milliseconds to end the track.
    ///
    /// `Some(None)` resets this if it was set previously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Option<u64>>,
    /// The guild ID of the player.
    #[serde(skip_serializing)]
    pub guild_id: Id<GuildMarker>,
    /// Whether or not to replace the currently playing track with this new
    /// track.
    ///
    /// Set to `true` to keep playing the current playing track, or `false`
    /// to replace the current playing track with a new one.
    #[serde(skip_serializing)]
    pub no_replace: bool,
    /// The position in milliseconds to start the track from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// Whether the player is paused
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Information about the track to play.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<UpdatePlayerTrack>,
    /// The player volume, in percentage, from 0 to 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u64>,
}

impl Play {
    /// Create a new play event.
    pub fn new(
        guild_id: Id<GuildMarker>,
        track: impl Into<String>,
        start_time: impl Into<Option<u64>>,
        end_time: impl Into<Option<u64>>,
        no_replace: bool,
    ) -> Self {
        Self::from((guild_id, track, start_time, end_time, no_replace))
    }
}

impl<T: Into<String>> From<(Id<GuildMarker>, T)> for Play {
    fn from((guild_id, track): (Id<GuildMarker>, T)) -> Self {
        Self::from((guild_id, track, None, None, true))
    }
}

impl<T: Into<String>, S: Into<Option<u64>>> From<(Id<GuildMarker>, T, S)> for Play {
    fn from((guild_id, track, start_time): (Id<GuildMarker>, T, S)) -> Self {
        Self::from((guild_id, track, start_time, None, true))
    }
}

impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>> From<(Id<GuildMarker>, T, S, E)>
    for Play
{
    fn from((guild_id, track, start_time, end_time): (Id<GuildMarker>, T, S, E)) -> Self {
        Self::from((guild_id, track, start_time, end_time, true))
    }
}

impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>>
    From<(Id<GuildMarker>, T, S, E, bool)> for Play
{
    fn from(
        (guild_id, track, start_time, end_time, no_replace): (Id<GuildMarker>, T, S, E, bool),
    ) -> Self {
        Self {
            guild_id,
            no_replace,
            position: start_time.into(),
            end_time: Some(end_time.into()),
            volume: None,
            paused: None,
            track: Some(UpdatePlayerTrack {
                track_string: TrackOption::Encoded(Some(track.into())),
            }),
        }
    }
}

/// Seek a player's active track to a new position.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Seek {
    /// The guild ID of the player.
    #[serde(skip_serializing)]
    pub guild_id: Id<GuildMarker>,
    /// The position in milliseconds to seek to.
    pub position: i64,
}

impl Seek {
    /// Create a new seek event.
    pub fn new(guild_id: Id<GuildMarker>, position: i64) -> Self {
        Self::from((guild_id, position))
    }
}

impl From<(Id<GuildMarker>, i64)> for Seek {
    fn from((guild_id, position): (Id<GuildMarker>, i64)) -> Self {
        Self { guild_id, position }
    }
}

/// Stop a player.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    /// The guild ID of the player.
    #[serde(skip_serializing)]
    pub guild_id: Id<GuildMarker>,
    /// The track object to pass set to null
    pub track: UpdatePlayerTrack,
}

impl Stop {
    /// Create a new stop event.
    pub fn new(guild_id: Id<GuildMarker>) -> Self {
        Self::from(guild_id)
    }
}

impl From<Id<GuildMarker>> for Stop {
    fn from(guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            track: UpdatePlayerTrack {
                track_string: TrackOption::Encoded(None),
            },
        }
    }
}
/// The voice payload for the combined server and state to send to lavalink.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Voice {
    /// The Discord voice endpoint to connect to.
    pub endpoint: String,
    /// The Discord voice session id to authenticate with. This is separate from the session id of lavalink.
    pub session_id: String,
    /// The Discord voice token to authenticate with.
    pub token: String,
}

/// A combined voice server and voice state update.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct VoiceUpdate {
    /// The guild ID of the player.
    #[serde(skip_serializing)]
    pub guild_id: Id<GuildMarker>,
    /// The voice payload for the combined server and state to send to lavalink.
    pub voice: Voice,
}

impl VoiceUpdate {
    /// Create a new voice update event.
    pub fn new(
        guild_id: Id<GuildMarker>,
        session_id: impl Into<String>,
        event: VoiceServerUpdate,
    ) -> Self {
        Self::from((guild_id, session_id, event))
    }
}

impl<T: Into<String>> From<(Id<GuildMarker>, T, VoiceServerUpdate)> for VoiceUpdate {
    fn from((guild_id, session_id, event): (Id<GuildMarker>, T, VoiceServerUpdate)) -> Self {
        Self {
            guild_id,
            voice: Voice {
                token: event.token,
                endpoint: event.endpoint.unwrap_or("NO_ENDPOINT_RETURNED".to_string()),
                session_id: session_id.into(),
            },
        }
    }
}

/// Set the volume of a player.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    /// The guild ID of the player.
    #[serde(skip_serializing)]
    pub guild_id: Id<GuildMarker>,
    /// The volume of the player from 0 to 1000. 100 is the default.
    pub volume: i64,
}

impl Volume {
    /// Create a new volume event.
    pub fn new(guild_id: Id<GuildMarker>, volume: i64) -> Self {
        Self::from((guild_id, volume))
    }
}

impl From<(Id<GuildMarker>, i64)> for Volume {
    fn from((guild_id, volume): (Id<GuildMarker>, i64)) -> Self {
        Self { guild_id, volume }
    }
}
