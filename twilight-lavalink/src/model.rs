//! Models to (de)serialize incoming/outgoing websocket events and HTTP
//! responses.

pub mod incoming;
pub mod outgoing;

pub use self::{
    incoming::{
        Exception, IncomingEvent, PlayerUpdate, PlayerUpdateState, Stats, StatsCpu, StatsFrame,
        StatsMemory, Track, TrackEnd, TrackException, TrackStart, TrackStuck, WebSocketClosed,
    },
    outgoing::{
        Destroy, Equalizer, EqualizerBand, OutgoingEvent, Pause, Play, Seek, Stop,
        UpdatePlayerTrack, VoiceUpdate, Volume,
    },
};

#[cfg(test)]
mod lavalink_struct_tests {
    use super::incoming::{Stats, StatsCpu, StatsMemory};
    use serde_test::Token;

    #[test]
    fn stats_frames_not_provided() {
        const LAVALINK_LOAD: f64 = 0.276_119_402_985_074_65;
        const MEM_ALLOCATED: u64 = 62_914_560;
        const MEM_FREE: u64 = 27_664_576;
        const MEM_RESERVABLE: u64 = 4_294_967_296;
        const MEM_USED: u64 = 35_249_984;
        const SYSTEM_LOAD: f64 = 0.195_380_536_378_835_9;

        let expected = Stats {
            op: crate::model::incoming::Opcode::Stats,
            cpu: StatsCpu {
                cores: 4,
                lavalink_load: LAVALINK_LOAD,
                system_load: SYSTEM_LOAD,
            },
            frame_stats: None,
            memory: StatsMemory {
                allocated: MEM_ALLOCATED,
                free: MEM_FREE,
                reservable: MEM_RESERVABLE,
                used: MEM_USED,
            },
            players: 0,
            playing_players: 0,
            uptime: 18589,
        };

        serde_test::assert_de_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "Stats",
                    len: 6,
                },
                Token::Str("cpu"),
                Token::Struct {
                    name: "StatsCpu",
                    len: 3,
                },
                Token::Str("cores"),
                Token::U64(4),
                Token::Str("lavalinkLoad"),
                Token::F64(LAVALINK_LOAD),
                Token::Str("systemLoad"),
                Token::F64(SYSTEM_LOAD),
                Token::StructEnd,
                Token::Str("memory"),
                Token::Struct {
                    name: "StatsMemory",
                    len: 4,
                },
                Token::Str("allocated"),
                Token::U64(MEM_ALLOCATED),
                Token::Str("free"),
                Token::U64(MEM_FREE),
                Token::Str("reservable"),
                Token::U64(MEM_RESERVABLE),
                Token::Str("used"),
                Token::U64(MEM_USED),
                Token::StructEnd,
                Token::Str("op"),
                Token::UnitVariant {
                    name: "Opcode",
                    variant: "stats",
                },
                Token::Str("players"),
                Token::U64(0),
                Token::Str("playingPlayers"),
                Token::U64(0),
                Token::Str("uptime"),
                Token::U64(18589),
                Token::StructEnd,
            ],
        );
    }
}

#[cfg(test)]
mod lavalink_incoming_model_tests {
    use crate::model::{TrackEnd, TrackException, TrackStart, TrackStuck};
    use twilight_model::id::{marker::GuildMarker, Id};

    use super::{
        incoming::{
            Event, EventData, EventType, Exception, Opcode, PlayerUpdate, PlayerUpdateState, Ready,
            Severity, Stats, StatsCpu, StatsFrame, StatsMemory, Track, TrackEndReason, TrackInfo,
        },
        WebSocketClosed,
    };

    // These are incoming so we only need to check that the input json can deserialize into the struct.
    fn compare_json_payload<
        T: std::fmt::Debug + for<'a> serde::Deserialize<'a> + std::cmp::PartialEq,
    >(
        data_struct: &T,
        json_payload: &str,
    ) {
        // Deserialize
        let deserialized: T = serde_json::from_str(json_payload).unwrap();
        assert_eq!(deserialized, *data_struct);
    }

    #[test]
    fn should_deserialize_a_ready_response() {
        let ready = Ready {
            op: Opcode::Ready,
            resumed: false,
            session_id: "la3kfsdf5eafe848".to_string(),
        };
        compare_json_payload(
            &ready,
            r#"{"op":"ready","resumed":false,"sessionId":"la3kfsdf5eafe848"}"#,
        );
    }

    #[test]
    fn should_deserialize_a_player_update_response() {
        let update = PlayerUpdate {
            op: Opcode::PlayerUpdate,
            guild_id: Id::<GuildMarker>::new(987_654_321),
            state: PlayerUpdateState {
                time: 1_710_214_147_839,
                position: 534,
                connected: true,
                ping: 0,
            },
        };
        compare_json_payload(
            &update,
            r#"{"op":"playerUpdate","guildId":"987654321","state":{"time":1710214147839,"position":534,"connected":true,"ping":0}}"#,
        );
    }

    #[test]
    fn should_deserialize_stat_event() {
        let stat_event = Stats {
            op: Opcode::Stats,
            players: 0,
            playing_players: 0,
            uptime: 1_139_738,
            cpu: StatsCpu {
                cores: 16,
                lavalink_load: 3.497_090_420_769_919E-5,
                system_load: 0.055_979_978_347_863_06,
            },
            frame_stats: None,
            memory: StatsMemory {
                allocated: 331_350_016,
                free: 228_139_904,
                reservable: 8_396_996_608,
                used: 103_210_112,
            },
        };
        compare_json_payload(
            &stat_event.clone(),
            r#"{"op":"stats","frameStats":null,"players":0,"playingPlayers":0,"uptime":1139738,"memory":{"free":228139904,"used":103210112,"allocated":331350016,"reservable":8396996608},"cpu":{"cores":16,"systemLoad":0.05597997834786306,"lavalinkLoad":3.497090420769919E-5}}"#,
        );
    }

    #[test]
    fn should_deserialize_stat_event_with_frame_stat() {
        let stat_event = Stats {
            op: Opcode::Stats,
            players: 0,
            playing_players: 0,
            uptime: 1_139_738,
            cpu: StatsCpu {
                cores: 16,
                lavalink_load: 3.497_090_420_769_919E-5,
                system_load: 0.055_979_978_347_863_06,
            },
            frame_stats: Some(StatsFrame {
                sent: 6000,
                nulled: 10,
                deficit: -3010,
            }),
            memory: StatsMemory {
                allocated: 331_350_016,
                free: 228_139_904,
                reservable: 8_396_996_608,
                used: 103_210_112,
            },
        };
        compare_json_payload(
            &stat_event.clone(),
            r#"{"op":"stats","frameStats":{"sent":6000,"nulled":10,"deficit":-3010},"players":0,"playingPlayers":0,"uptime":1139738,"memory":{"free":228139904,"used":103210112,"allocated":331350016,"reservable":8396996608},"cpu":{"cores":16,"systemLoad":0.05597997834786306,"lavalinkLoad":3.497090420769919E-5}}"#,
        );
    }

    #[test]
    fn should_deserialize_track_start_event() {
        let track_start_event = Event {
            op: Opcode::Event,
            r#type: EventType::TrackStartEvent,
            guild_id: Id::<GuildMarker>::new(987_654_321).to_string(),
            data: EventData::TrackStartEvent(
                TrackStart {
                    track: Track {
                        encoded: "QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA".to_string(),
                        info: TrackInfo {
                            identifier: "OnuuYcqhzCE".to_string(),
                            is_seekable: true,
                            author: "Linkin Park".to_string(),
                            length: 169_000,
                            is_stream: false,
                            position: 0,
                            title: "Bleed It Out [Official Music Video] - Linkin Park".to_string(),
                            uri:Some("https://www.youtube.com/watch?v=OnuuYcqhzCE".to_string()),
                            source_name:"youtube".to_string(),
                            artwork_url:Some("https://i.ytimg.com/vi/OnuuYcqhzCE/maxresdefault.jpg".to_string()),
                            isrc: None
                        }
                    }
                }
            )

        };
        compare_json_payload(
            &track_start_event.clone(),
            r#"{"op":"event","guildId":"987654321","type":"TrackStartEvent","track":{"encoded":"QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA","info":{"identifier":"OnuuYcqhzCE","isSeekable":true,"author":"Linkin Park","length":169000,"isStream":false,"position":0,"title":"Bleed It Out [Official Music Video] - Linkin Park","uri":"https://www.youtube.com/watch?v=OnuuYcqhzCE","artworkUrl":"https://i.ytimg.com/vi/OnuuYcqhzCE/maxresdefault.jpg","isrc":null,"sourceName":"youtube"},"pluginInfo":{},"userData":{}}}"#,
        );
    }

    #[test]
    fn should_deserialize_track_exception_event() {
        let track_exception_event = Event {
            op: Opcode::Event,
            r#type: EventType::TrackExceptionEvent,
            guild_id: Id::<GuildMarker>::new(987_654_321).to_string(),
            data: EventData::TrackExceptionEvent(
                TrackException {
                    track: Track {
                        encoded: "QAAAjQIAJVJpY2sgQXN0bGV5IC0gTmV2ZXIgR29ubmEgR2l2ZSBZb3UgVXAADlJpY2tBc3RsZXlWRVZPAAAAAAADPCAAC2RRdzR3OVdnWGNRAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9ZFF3NHc5V2dYY1EAB3lvdXR1YmUAAAAAAAAAAA==".to_string(),
                        info: TrackInfo {
                            identifier: "dQw4w9WgXcQ".to_string(),
                            is_seekable: true,
                            author: "RickAstleyVEVO".to_string(),
                            length: 212_000,
                            is_stream: false,
                            position: 0,
                            title: "Rick Astley - Never Gonna Give You Up".to_string(),
                            uri:Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()),
                            source_name:"youtube".to_string(),
                            artwork_url:Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg".to_string()),
                            isrc: None
                        }
                    },
                    exception: Exception {
                        message: Some(String::new()),
                        severity: Severity::Common,
                        cause: "No video found.".to_string(),
                    }

                }
            )

        };
        compare_json_payload(
            &track_exception_event.clone(),
            r#"{"op":"event","type":"TrackExceptionEvent","guildId":"987654321","track":{"encoded":"QAAAjQIAJVJpY2sgQXN0bGV5IC0gTmV2ZXIgR29ubmEgR2l2ZSBZb3UgVXAADlJpY2tBc3RsZXlWRVZPAAAAAAADPCAAC2RRdzR3OVdnWGNRAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9ZFF3NHc5V2dYY1EAB3lvdXR1YmUAAAAAAAAAAA==","info":{"identifier":"dQw4w9WgXcQ","isSeekable":true,"author":"RickAstleyVEVO","length":212000,"isStream":false,"position":0,"title":"Rick Astley - Never Gonna Give You Up","uri":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","artworkUrl":"https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg","isrc":null,"sourceName":"youtube"},"pluginInfo":{}},"exception":{"message":"","severity":"common","cause":"No video found."}}"#,
        );
    }

    #[test]
    fn should_deserialize_track_stuck_event() {
        let track_stuck_event = Event {
            op: Opcode::Event,
            r#type: EventType::TrackStuckEvent,
            guild_id: Id::<GuildMarker>::new(987_654_321).to_string(),
            data: EventData::TrackStuckEvent(
                TrackStuck {
                    track: Track {
                        encoded: "QAAAjQIAJVJpY2sgQXN0bGV5IC0gTmV2ZXIgR29ubmEgR2l2ZSBZb3UgVXAADlJpY2tBc3RsZXlWRVZPAAAAAAADPCAAC2RRdzR3OVdnWGNRAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9ZFF3NHc5V2dYY1EAB3lvdXR1YmUAAAAAAAAAAA==".to_string(),
                        info: TrackInfo {
                            identifier: "dQw4w9WgXcQ".to_string(),
                            is_seekable: true,
                            author: "RickAstleyVEVO".to_string(),
                            length: 212_000,
                            is_stream: false,
                            position: 0,
                            title: "Rick Astley - Never Gonna Give You Up".to_string(),
                            uri:Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()),
                            source_name:"youtube".to_string(),
                            artwork_url:Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg".to_string()),
                            isrc: None
                        }
                    },
                    threshold_ms: 123_456_789,

                }
            )

        };
        compare_json_payload(
            &track_stuck_event.clone(),
            r#"{"op":"event","type":"TrackStuckEvent","guildId":"987654321","track":{"encoded":"QAAAjQIAJVJpY2sgQXN0bGV5IC0gTmV2ZXIgR29ubmEgR2l2ZSBZb3UgVXAADlJpY2tBc3RsZXlWRVZPAAAAAAADPCAAC2RRdzR3OVdnWGNRAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9ZFF3NHc5V2dYY1EAB3lvdXR1YmUAAAAAAAAAAA==","info":{"identifier":"dQw4w9WgXcQ","isSeekable":true,"author":"RickAstleyVEVO","length":212000,"isStream":false,"position":0,"title":"Rick Astley - Never Gonna Give You Up","uri":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","artworkUrl":"https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg","isrc":null,"sourceName":"youtube"},"pluginInfo":{}},"thresholdMs":123456789}"#,
        );
    }

    #[test]
    fn should_deserialize_track_end_event() {
        let track_stuck_event = Event {
            op: Opcode::Event,
            r#type: EventType::TrackEndEvent,
            guild_id: Id::<GuildMarker>::new(987_654_321).to_string(),
            data: EventData::TrackEndEvent(
                TrackEnd {
                    track: Track {
                        encoded: "QAAAjQIAJVJpY2sgQXN0bGV5IC0gTmV2ZXIgR29ubmEgR2l2ZSBZb3UgVXAADlJpY2tBc3RsZXlWRVZPAAAAAAADPCAAC2RRdzR3OVdnWGNRAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9ZFF3NHc5V2dYY1EAB3lvdXR1YmUAAAAAAAAAAA==".to_string(),
                        info: TrackInfo {
                            identifier: "dQw4w9WgXcQ".to_string(),
                            is_seekable: true,
                            author: "RickAstleyVEVO".to_string(),
                            length: 212_000,
                            is_stream: false,
                            position: 0,
                            title: "Rick Astley - Never Gonna Give You Up".to_string(),
                            uri:Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()),
                            source_name:"youtube".to_string(),
                            artwork_url:Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg".to_string()),
                            isrc: None
                        }
                    },
                    reason: TrackEndReason::Finished,
                }
            )

        };
        compare_json_payload(
            &track_stuck_event.clone(),
            r#"{"op":"event","type":"TrackEndEvent","guildId":"987654321","track":{"encoded":"QAAAjQIAJVJpY2sgQXN0bGV5IC0gTmV2ZXIgR29ubmEgR2l2ZSBZb3UgVXAADlJpY2tBc3RsZXlWRVZPAAAAAAADPCAAC2RRdzR3OVdnWGNRAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9ZFF3NHc5V2dYY1EAB3lvdXR1YmUAAAAAAAAAAA==","info":{"identifier":"dQw4w9WgXcQ","isSeekable":true,"author":"RickAstleyVEVO","length":212000,"isStream":false,"position":0,"title":"Rick Astley - Never Gonna Give You Up","uri":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","artworkUrl":"https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg","isrc":null,"sourceName":"youtube"},"pluginInfo":{}},"reason":"finished"}"#,
        );
    }

    #[test]
    fn should_deserialize_websocketclosed_event() {
        let websocket_closed_event = Event {
            op: Opcode::Event,
            r#type: EventType::WebSocketClosedEvent,
            guild_id: Id::<GuildMarker>::new(987_654_321).to_string(),
            data: EventData::WebSocketClosedEvent(WebSocketClosed {
                code: 1000,
                reason: String::new(),
                by_remote: false,
            }),
        };
        compare_json_payload(
            &websocket_closed_event.clone(),
            r#"{"op":"event","type":"WebSocketClosedEvent","guildId":"987654321","code":1000,"reason":"","byRemote":false}"#,
        );
    }
}

#[cfg(test)]
mod lavalink_outgoing_model_tests {
    use crate::model::outgoing::TrackOption;
    use crate::model::{Destroy, Equalizer, Pause, Play, Seek, Stop, Volume};

    use twilight_model::id::{marker::GuildMarker, Id};

    use super::outgoing::{OutgoingEvent, UpdatePlayerTrack, Voice, VoiceUpdate};
    use super::EqualizerBand;

    // For some of the outgoing we have fields that don't get deserialized. We only need
    // to check weather the serialization is working.
    fn compare_json_payload<T: serde::Serialize + std::fmt::Debug + std::cmp::PartialEq>(
        data_struct: &T,
        json_payload: &str,
    ) {
        let serialized = serde_json::to_string(&data_struct).unwrap();
        let expected_serialized = json_payload;
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn should_serialize_an_outgoing_voice_update() {
        let voice = VoiceUpdate {
            guild_id: Id::<GuildMarker>::new(987_654_321),
            voice: Voice {
                token: String::from("863ea8ef2ads8ef2"),
                endpoint: String::from("eu-centra654863.discord.media:443"),
                session_id: String::from("asdf5w1efa65feaf315e8a8effsa1e5f"),
            },
        };
        compare_json_payload(
            &voice,
            r#"{"voice":{"endpoint":"eu-centra654863.discord.media:443","sessionId":"asdf5w1efa65feaf315e8a8effsa1e5f","token":"863ea8ef2ads8ef2"}}"#,
        );
    }

    #[test]
    fn should_serialize_an_outgoing_play() {
        let play = OutgoingEvent::Play(Play{
            track: UpdatePlayerTrack {
                track_string: TrackOption::Encoded(Some("QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA".to_string())),
            },
            position: None,
            end_time: Some(None),
            volume: None,
            paused: None,
            guild_id: Id::<GuildMarker>::new(987_654_321),
            no_replace: true,
        });
        compare_json_payload(
            &play,
            r#"{"endTime":null,"track":{"encoded":"QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA"}}"#,
        );
    }

    #[test]
    fn should_serialize_an_outgoing_stop() {
        let stop = OutgoingEvent::Stop(Stop {
            track: UpdatePlayerTrack {
                track_string: TrackOption::Encoded(None),
            },
            guild_id: Id::<GuildMarker>::new(987_654_321),
        });
        compare_json_payload(&stop, r#"{"track":{"encoded":null}}"#);
    }

    #[test]
    fn should_serialize_an_outgoing_pause() {
        let pause = OutgoingEvent::Pause(Pause {
            paused: true,
            guild_id: Id::<GuildMarker>::new(987_654_321),
        });
        compare_json_payload(&pause, r#"{"guildId":"987654321","paused":true}"#);
    }

    #[test]
    fn should_serialize_an_outgoing_seek() {
        let seek = OutgoingEvent::Seek(Seek {
            position: 66000,
            guild_id: Id::<GuildMarker>::new(987_654_321),
        });
        compare_json_payload(&seek, r#"{"position":66000}"#);
    }

    #[test]
    fn should_serialize_an_outgoing_volume() {
        let volume = OutgoingEvent::Volume(Volume {
            volume: 50,
            guild_id: Id::<GuildMarker>::new(987_654_321),
        });
        compare_json_payload(&volume, r#"{"volume":50}"#);
    }

    #[test]
    fn should_serialize_an_outgoing_destroy_aka_leave() {
        let destroy = OutgoingEvent::Destroy(Destroy {
            guild_id: Id::<GuildMarker>::new(987_654_321),
        });
        compare_json_payload(&destroy, r#"{"guildId":"987654321"}"#);
    }

    #[test]
    fn should_serialize_an_outgoing_equalize() {
        let equalize = OutgoingEvent::Equalizer(Equalizer {
            equalizer: vec![EqualizerBand::new(5, -0.15)],
            guild_id: Id::<GuildMarker>::new(987_654_321),
        });
        compare_json_payload(&equalize, r#"{"equalizer":[{"band":5,"gain":-0.15}]}"#);
    }
}
