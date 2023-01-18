use super::{Event, EventConversionError};
use serde::{Deserialize, Serialize};

/// Indicator that a shard is now fully connected.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connected {
    /// The interval that heartbeats are being sent to the gateway.
    pub heartbeat_interval: u64,
    /// The ID of the shard that's now connected.
    pub shard_id: u64,
}

/// Indicator that a shard is now connecting.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connecting {
    /// The URL used to connect to the gateway.
    pub gateway: String,
    /// The ID of the shard that's now connecting.
    pub shard_id: u64,
}

/// Indicator that a shard is now disconnected and may soon be reconnecting if
/// not explicitly shutdown.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Disconnected {
    /// The code for the disconnect if not initiated by the host, if any.
    pub code: Option<u16>,
    /// The reason for the disconnect if not initiated by the host, if any.
    pub reason: Option<String>,
    /// The ID of the shard that's now disconnected.
    pub shard_id: u64,
}

/// Indicator that a shard is now identifying with the gateway to create a new
/// session.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Identifying {
    /// The ID of the shard that identified with the gateway.
    pub shard_id: u64,
    /// The total shards used by the bot.
    pub shard_total: u64,
}

/// A payload of bytes came in through the gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Payload {
    /// The bytes that came in.
    pub bytes: Vec<u8>,
}

/// Indicator that a shard is now reconnecting.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Reconnecting {
    /// The ID of the shard that began reconnecting.
    pub shard_id: u64,
}

/// Indicator that a shard is now resuming a session after a disconnect.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Resuming {
    /// The event sequence sent when resuming was initiated.
    pub seq: u64,
    /// The ID of the shard that began resuming.
    pub shard_id: u64,
}

/// "Meta" events about a shard's status, not from the gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ShardEvent {
    /// A shard is now in a Connected stage after being fully connected to the
    /// gateway.
    Connected(Connected),
    /// A shard is now in a Connecting stage after starting to connect to the
    /// gateway.
    Connecting(Connecting),
    /// A shard is now in a Disconnected stage after the connection was closed.
    Disconnected(Disconnected),
    /// A shard is now in a Identifying stage after starting a new session.
    Identifying(Identifying),
    /// A payload of bytes came in through the shard's connection.
    Payload(Payload),
    /// A shard is now in a Reconnecting stage after a disconnect or session was
    /// ended.
    Reconnecting(Reconnecting),
    /// A shard is now in a Resuming stage after a disconnect.
    Resuming(Resuming),
}

impl TryFrom<Event> for ShardEvent {
    type Error = EventConversionError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        Ok(match event {
            Event::ShardConnected(v) => Self::Connected(v),
            Event::ShardConnecting(v) => Self::Connecting(v),
            Event::ShardDisconnected(v) => Self::Disconnected(v),
            Event::ShardIdentifying(v) => Self::Identifying(v),
            Event::ShardPayload(v) => Self::Payload(v),
            Event::ShardReconnecting(v) => Self::Reconnecting(v),
            Event::ShardResuming(v) => Self::Resuming(v),

            _ => return Err(EventConversionError::new(event)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Connected, Connecting, Disconnected, Event, Identifying, Payload, Reconnecting, Resuming,
        ShardEvent,
    };
    use serde_test::Token;

    #[test]
    fn connected() {
        let value = Connected {
            heartbeat_interval: 41_250,
            shard_id: 4,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Connected",
                    len: 2,
                },
                Token::Str("heartbeat_interval"),
                Token::U64(41_250),
                Token::Str("shard_id"),
                Token::U64(4),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn connecting() {
        let value = Connecting {
            gateway: "https://example.com".to_owned(),
            shard_id: 4,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Connecting",
                    len: 2,
                },
                Token::Str("gateway"),
                Token::Str("https://example.com"),
                Token::Str("shard_id"),
                Token::U64(4),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn disconnected() {
        let value = Disconnected {
            code: Some(4_000),
            reason: Some("the reason".to_owned()),
            shard_id: 4,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Disconnected",
                    len: 3,
                },
                Token::Str("code"),
                Token::Some,
                Token::U16(4_000),
                Token::Str("reason"),
                Token::Some,
                Token::Str("the reason"),
                Token::Str("shard_id"),
                Token::U64(4),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn identifying() {
        let value = Identifying {
            shard_id: 4,
            shard_total: 7,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Identifying",
                    len: 2,
                },
                Token::Str("shard_id"),
                Token::U64(4),
                Token::Str("shard_total"),
                Token::U64(7),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn payload() {
        let value = Payload {
            bytes: Vec::from([1, 2]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Payload",
                    len: 1,
                },
                Token::Str("bytes"),
                Token::Seq { len: Some(2) },
                Token::U8(1),
                Token::U8(2),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn reconnecting() {
        let value = Reconnecting { shard_id: 4 };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Reconnecting",
                    len: 1,
                },
                Token::Str("shard_id"),
                Token::U64(4),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn resuming() {
        let value = Resuming {
            seq: 100,
            shard_id: 4,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Resuming",
                    len: 2,
                },
                Token::Str("seq"),
                Token::U64(100),
                Token::Str("shard_id"),
                Token::U64(4),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn shard_event_try_from_event() {
        let connected = Event::ShardConnected(Connected {
            heartbeat_interval: 41_250,
            shard_id: 4,
        });
        assert!(matches!(
            connected.try_into().unwrap(),
            ShardEvent::Connected(_)
        ));

        let connecting = Event::ShardConnecting(Connecting {
            gateway: "https://example.com".to_owned(),
            shard_id: 4,
        });
        assert!(matches!(
            connecting.try_into().unwrap(),
            ShardEvent::Connecting(_)
        ));

        let disconnected = Event::ShardDisconnected(Disconnected {
            code: Some(4_000),
            reason: None,
            shard_id: 4,
        });
        assert!(matches!(
            disconnected.try_into().unwrap(),
            ShardEvent::Disconnected(_)
        ));

        let identifying = Event::ShardIdentifying(Identifying {
            shard_id: 4,
            shard_total: 7,
        });
        assert!(matches!(
            identifying.try_into().unwrap(),
            ShardEvent::Identifying(_)
        ));

        let payload = Event::ShardPayload(Payload {
            bytes: Vec::from([1, 2]),
        });
        assert!(matches!(
            payload.try_into().unwrap(),
            ShardEvent::Payload(_)
        ));

        let reconnecting = Event::ShardReconnecting(Reconnecting { shard_id: 4 });
        assert!(matches!(
            reconnecting.try_into().unwrap(),
            ShardEvent::Reconnecting(_)
        ));

        let resuming = Event::ShardResuming(Resuming {
            seq: 100,
            shard_id: 4,
        });
        assert!(matches!(
            resuming.try_into().unwrap(),
            ShardEvent::Resuming(_)
        ));
    }
}
