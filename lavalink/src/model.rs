//! Models to (de)serialize incoming/outgoing websocket events and HTTP
//! responses.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    Destroy,
    Equalizer,
    Pause,
    Play,
    PlayerUpdate,
    Seek,
    Stats,
    Stop,
    VoiceUpdate,
    Volume,
}

mod outgoing {
    use super::Opcode;
    use serde::{Deserialize, Serialize};
    use twilight_model::{gateway::payload::VoiceServerUpdate, id::GuildId};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(untagged)]
    pub enum OutgoingEvent {
        Destroy(Destroy),
        Equalizer(Equalizer),
        Pause(Pause),
        Play(Play),
        Seek(Seek),
        Stop(Stop),
        VoiceUpdate(VoiceUpdate),
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

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Destroy {
        op: Opcode,
        pub guild_id: GuildId,
    }

    impl Destroy {
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

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Equalizer {
        op: Opcode,
        pub bands: Vec<EqualizerBand>,
        pub guild_id: GuildId,
    }

    impl Equalizer {
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

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct EqualizerBand {
        pub band: i64,
        pub gain: f64,
    }

    impl EqualizerBand {
        pub fn new(band: i64, gain: f64) -> Self {
            Self::from((band, gain))
        }
    }

    impl From<(i64, f64)> for EqualizerBand {
        fn from((band, gain): (i64, f64)) -> Self {
            Self { band, gain }
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Pause {
        op: Opcode,
        pub guild_id: GuildId,
        pub pause: bool,
    }

    impl Pause {
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

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Play {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<u64>,
        pub guild_id: GuildId,
        pub no_replace: bool,
        op: Opcode,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_time: Option<u64>,
        pub track: String,
    }

    impl Play {
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

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Seek {
        op: Opcode,
        pub guild_id: GuildId,
        pub position: i64,
    }

    impl Seek {
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

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Stop {
        op: Opcode,
        pub guild_id: GuildId,
    }

    impl Stop {
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

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct VoiceUpdate {
        op: Opcode,
        pub guild_id: GuildId,
        pub session_id: String,
        pub event: VoiceServerUpdate,
    }

    impl VoiceUpdate {
        pub fn new(
            guild_id: GuildId,
            session_id: impl Into<String>,
            event: VoiceServerUpdate,
        ) -> Self {
            Self::from((guild_id, session_id, event))
        }
    }

    impl<T: Into<String>> From<(GuildId, T, VoiceServerUpdate)> for VoiceUpdate {
        fn from((guild_id, session_id, event): (GuildId, T, VoiceServerUpdate)) -> Self {
            Self {
                event,
                guild_id,
                op: Opcode::VoiceUpdate,
                session_id: session_id.into(),
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Volume {
        op: Opcode,
        pub guild_id: GuildId,
        pub volume: i64,
    }

    impl Volume {
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

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(untagged)]
    pub enum IncomingEvent {
        PlayerUpdate(PlayerUpdate),
        Stats(Stats),
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

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerUpdate {
        pub op: Opcode,
        pub guild_id: GuildId,
        pub state: PlayerUpdateState,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerUpdateState {
        pub time: i64,
        pub position: i64,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Stats {
        pub cpu: StatsCpu,
        pub memory: StatsMemory,
        pub players: u64,
        pub playing_players: u64,
        pub op: Opcode,
        pub uptime: u64,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct StatsCpu {
        pub cores: usize,
        pub lavalink_load: f64,
        pub system_load: f64,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct StatsMemory {
        pub allocated: u64,
        pub free: u64,
        pub reservable: u64,
        pub used: u64,
    }
}

pub use self::{incoming::*, outgoing::*};

#[cfg(test)]
mod tests {
    use super::{
        incoming::{IncomingEvent, PlayerUpdate, PlayerUpdateState, Stats, StatsCpu, StatsMemory},
        outgoing::{
            Destroy, Equalizer, EqualizerBand, OutgoingEvent, Pause, Play, Seek, Stop, VoiceUpdate,
            Volume,
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
