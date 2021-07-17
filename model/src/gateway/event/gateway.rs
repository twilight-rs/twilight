use super::{
    super::OpCode, DispatchEvent, DispatchEventWithTypeDeserializer, Event, EventConversionError,
};
use serde::{
    de::{
        value::U8Deserializer, DeserializeSeed, Deserializer, Error as DeError, IgnoredAny,
        IntoDeserializer, MapAccess, Unexpected, Visitor,
    },
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};
use std::{convert::TryFrom, str::FromStr};

/// An event from the gateway, which can either be a dispatch event with
/// stateful updates or a heartbeat, hello, etc. that a shard needs to operate.
#[derive(Clone, Debug)]
pub enum GatewayEvent {
    Dispatch(u64, Box<DispatchEvent>),
    Heartbeat(u64),
    HeartbeatAck,
    Hello(u64),
    InvalidateSession(bool),
    Reconnect,
}

impl TryFrom<Event> for GatewayEvent {
    type Error = EventConversionError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        Ok(match event {
            Event::GatewayHeartbeat(v) => Self::Heartbeat(v),
            Event::GatewayHeartbeatAck => Self::HeartbeatAck,
            Event::GatewayHello(v) => Self::Hello(v),
            Event::GatewayInvalidateSession(v) => Self::InvalidateSession(v),
            Event::GatewayReconnect => Self::Reconnect,

            _ => return Err(EventConversionError::new(event)),
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    D,
    Op,
    S,
    T,
}

#[derive(Debug, Deserialize, Serialize)]
struct Hello {
    heartbeat_interval: u64,
}

/// A deserializer that deserializes into a `GatewayEvent` by cloning some bits
/// of scanned information before the actual deserialisation.
///
/// This is the owned version of [`GatewayEventDeserializer`].
///
/// You should use this if you're using a mutable deserialization library
/// like `simd-json`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayEventDeserializerOwned {
    event_type: Option<String>,
    op: u8,
    sequence: Option<u64>,
}

impl GatewayEventDeserializerOwned {
    /// Create a new owned gateway event deserializer when you already know the
    /// event type and opcode.
    ///
    /// This might be useful if you scan the payload for this information and
    /// do some work with the event type prior to deserializing the payload.
    pub fn new(op: u8, sequence: Option<u64>, event_type: impl Into<Option<String>>) -> Self {
        Self {
            event_type: event_type.into(),
            op,
            sequence,
        }
    }

    pub fn from_json(input: &str) -> Option<Self> {
        let deser = GatewayEventDeserializer::from_json(input)?;
        let GatewayEventDeserializer {
            event_type,
            op,
            sequence,
        } = deser;

        Some(Self {
            event_type: event_type.map(ToOwned::to_owned),
            op,
            sequence,
        })
    }

    /// Return an immutable reference to the event type of the payload.
    pub fn event_type_ref(&self) -> Option<&str> {
        self.event_type.as_deref()
    }

    /// Return the opcode of the payload.
    pub const fn op(&self) -> u8 {
        self.op
    }

    /// Return the sequence of the payload.
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Consume the deserializer, returning its opcode, sequence, and event type
    /// components.
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_parts(self) -> (u8, Option<u64>, Option<String>) {
        (self.op, self.sequence, self.event_type)
    }
}

/// A deserializer that deserializes into a `GatewayEvent` by borrowing some bits
/// of scanned information before the actual deserialisation.
///
/// This is the borrowed version of [`GatewayEventDeserializerOwned`].
///
/// You should use this if you're using an immutable deserialization library
/// like `serde_json`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayEventDeserializer<'a> {
    event_type: Option<&'a str>,
    op: u8,
    sequence: Option<u64>,
}

impl<'a> GatewayEventDeserializer<'a> {
    /// Create a new gateway event deserializer when you already know the event
    /// type and opcode.
    ///
    /// This might be useful if you scan the payload for this information and
    /// do some work with the event type prior to deserializing the payload.
    pub const fn new(op: u8, sequence: Option<u64>, event_type: Option<&'a str>) -> Self {
        Self {
            event_type,
            op,
            sequence,
        }
    }

    /// Create a gateway event deserializer with some information found by
    /// scanning the JSON payload to deserialise.
    ///
    /// This will scan the payload for the opcode and, optionally, event type if
    /// provided. The opcode key ("op"), must be in the payload while the event
    /// type key ("t") is optional and only required for event ops.
    pub fn from_json(input: &'a str) -> Option<Self> {
        let op = Self::find_opcode(input)?;
        let event_type = Self::find_event_type(input);
        let sequence = Self::find_sequence(input);

        Some(Self {
            event_type,
            op,
            sequence,
        })
    }

    /// Return an immutable reference to the event type of the payload.
    pub const fn event_type_ref(&self) -> Option<&str> {
        self.event_type
    }

    /// Return the opcode of the payload.
    pub const fn op(&self) -> u8 {
        self.op
    }

    /// Return the sequence of the payload.
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Consume the deserializer, returning its opcode and event type
    /// components.
    pub const fn into_parts(self) -> (u8, Option<u64>, Option<&'a str>) {
        (self.op, self.sequence, self.event_type)
    }

    fn find_event_type(input: &'a str) -> Option<&'a str> {
        // We're going to search for the event type key from the start. Discord
        // always puts it at the front before the D key from some testing of
        // several hundred payloads.
        //
        // If we find it, add 4, since that's the length of what we're searching
        // for.
        let from = input.find(r#""t":"#)? + 4;

        // Now let's find where the value starts, which may be a string or null.
        // Or maybe something else. If it's anything but a string, then there's
        // no event type.
        let start = input.get(from..)?.find(|c: char| !c.is_whitespace())? + from + 1;

        // Check if the character just before the cursor is '"'.
        if input.as_bytes().get(start - 1).copied()? != b'"' {
            return None;
        }

        let to = input.get(start..)?.find('"')?;

        input.get(start..start + to)
    }

    fn find_opcode(input: &'a str) -> Option<u8> {
        Self::find_integer(input, r#""op":"#)
    }

    fn find_sequence(input: &'a str) -> Option<u64> {
        Self::find_integer(input, r#""s":"#)
    }

    fn find_integer<T: FromStr>(input: &'a str, key: &str) -> Option<T> {
        // Find the op key's position and then search for where the first
        // character that's not base 10 is. This'll give us the bytes with the
        // op which can be parsed.
        //
        // Add 5 at the end since that's the length of what we're finding.
        let from = input.find(key)? + key.len();

        // Look for the first thing that isn't a base 10 digit or whitespace,
        // i.e. a comma (denoting another JSON field), curly brace (end of the
        // object), etc. This'll give us the op number, maybe with a little
        // whitespace.
        let to = input.get(from..)?.find(&[',', '}'] as &[_])?;
        // We might have some whitespace, so let's trim this.
        let clean = input.get(from..from + to)?.trim();

        T::from_str(clean).ok()
    }
}

struct GatewayEventVisitor<'a>(u8, Option<u64>, Option<&'a str>);

impl GatewayEventVisitor<'_> {
    fn field<'de, T: Deserialize<'de>, V: MapAccess<'de>>(
        map: &mut V,
        field: Field,
    ) -> Result<T, V::Error> {
        let mut found = None;

        loop {
            match map.next_key::<Field>() {
                Ok(Some(key)) if key == field => found = Some(map.next_value()?),
                Ok(Some(_)) | Err(_) => {
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
                Ok(None) => {
                    break;
                }
            }
        }

        found.ok_or_else(|| {
            DeError::missing_field(match field {
                Field::D => "d",
                Field::Op => "op",
                Field::S => "s",
                Field::T => "t",
            })
        })
    }

    fn ignore_all<'de, V: MapAccess<'de>>(map: &mut V) -> Result<(), V::Error> {
        tracing::trace!("ignoring all other fields");

        while let Ok(Some(_)) | Err(_) = map.next_key::<Field>() {
            map.next_value::<IgnoredAny>()?;
        }

        tracing::trace!("ignored all other fields");

        Ok(())
    }
}

impl<'de> Visitor<'de> for GatewayEventVisitor<'_> {
    type Value = GatewayEvent;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("struct GatewayEvent")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V>(self, mut map: V) -> Result<GatewayEvent, V::Error>
    where
        V: MapAccess<'de>,
    {
        static VALID_OPCODES: &[&str] = &[
            "EVENT",
            "HEARTBEAT",
            "HEARTBEAT_ACK",
            "HELLO",
            "IDENTIFY",
            "INVALID_SESSION",
            "RECONNECT",
        ];

        let span = tracing::trace_span!("deserializing gateway event");
        let _span_enter = span.enter();
        tracing::trace!(event_type=?self.2, op=self.0, seq=?self.1);

        let op_deser: U8Deserializer<V::Error> = self.0.into_deserializer();

        let op = OpCode::deserialize(op_deser).ok().ok_or_else(|| {
            tracing::trace!(op = self.0, "unknown opcode");
            let unexpected = Unexpected::Unsigned(u64::from(self.0));

            DeError::invalid_value(unexpected, &"an opcode")
        })?;

        Ok(match op {
            OpCode::Event => {
                let t = self
                    .2
                    .ok_or_else(|| DeError::custom("event type not provided beforehand"))?;

                tracing::trace!("deserializing gateway dispatch");

                let mut d = None;

                loop {
                    let span_child = tracing::trace_span!("iterating over element");
                    let _span_child_enter = span_child.enter();

                    let key = match map.next_key() {
                        Ok(Some(key)) => {
                            tracing::trace!(?key, "found key");

                            key
                        }
                        Ok(None) => break,
                        Err(why) => {
                            map.next_value::<IgnoredAny>()?;

                            tracing::trace!("ran into an unknown key: {:?}", why);

                            continue;
                        }
                    };

                    match key {
                        Field::D => {
                            if d.is_some() {
                                return Err(DeError::duplicate_field("d"));
                            }

                            let deserializer = DispatchEventWithTypeDeserializer::new(t);

                            d = Some(map.next_value_seed(deserializer)?);
                        }
                        Field::Op | Field::S | Field::T => {
                            map.next_value::<IgnoredAny>()?;

                            tracing::trace!(key=?key, "ignoring key");
                        }
                    }
                }

                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let s = self.1.ok_or_else(|| DeError::missing_field("s"))?;

                Self::ignore_all(&mut map)?;

                GatewayEvent::Dispatch(s, Box::new(d))
            }
            OpCode::Heartbeat => {
                tracing::trace!("deserializing gateway heartbeat");
                let seq = Self::field(&mut map, Field::D)?;
                tracing::trace!(seq = %seq);

                Self::ignore_all(&mut map)?;

                GatewayEvent::Heartbeat(seq)
            }
            OpCode::HeartbeatAck => {
                tracing::trace!("deserializing gateway heartbeat ack");

                Self::ignore_all(&mut map)?;

                GatewayEvent::HeartbeatAck
            }
            OpCode::Hello => {
                tracing::trace!("deserializing gateway hello");
                let hello = Self::field::<Hello, _>(&mut map, Field::D)?;
                tracing::trace!(hello = ?hello);

                Self::ignore_all(&mut map)?;

                GatewayEvent::Hello(hello.heartbeat_interval)
            }
            OpCode::InvalidSession => {
                tracing::trace!("deserializing invalid session");
                let invalidate = Self::field::<bool, _>(&mut map, Field::D)?;
                tracing::trace!(invalidate = %invalidate);

                Self::ignore_all(&mut map)?;

                GatewayEvent::InvalidateSession(invalidate)
            }
            OpCode::Identify => return Err(DeError::unknown_variant("Identify", VALID_OPCODES)),
            OpCode::Reconnect => {
                Self::ignore_all(&mut map)?;

                GatewayEvent::Reconnect
            }
            OpCode::RequestGuildMembers => {
                return Err(DeError::unknown_variant(
                    "RequestGuildMembers",
                    VALID_OPCODES,
                ))
            }
            OpCode::Resume => return Err(DeError::unknown_variant("Resume", VALID_OPCODES)),
            OpCode::PresenceUpdate => {
                return Err(DeError::unknown_variant("PresenceUpdate", VALID_OPCODES))
            }
            OpCode::VoiceServerPing => {
                return Err(DeError::unknown_variant("VoiceServerPing", VALID_OPCODES))
            }
            OpCode::VoiceStateUpdate => {
                return Err(DeError::unknown_variant("VoiceStateUpdate", VALID_OPCODES))
            }
        })
    }
}

impl<'de> DeserializeSeed<'de> for GatewayEventDeserializer<'_> {
    type Value = GatewayEvent;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        const FIELDS: &[&str] = &["d", "s"];

        deserializer.deserialize_struct(
            "GatewayEvent",
            FIELDS,
            GatewayEventVisitor(self.op, self.sequence, self.event_type.as_deref()),
        )
    }
}

impl<'de> DeserializeSeed<'de> for GatewayEventDeserializerOwned {
    type Value = GatewayEvent;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        const FIELDS: &[&str] = &["d", "s"];

        deserializer.deserialize_struct(
            "GatewayEvent",
            FIELDS,
            GatewayEventVisitor(self.op, self.sequence, self.event_type.as_deref()),
        )
    }
}

impl Serialize for GatewayEvent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        const fn opcode(gateway_event: &GatewayEvent) -> OpCode {
            match gateway_event {
                GatewayEvent::Dispatch(_, _) => OpCode::Event,
                GatewayEvent::Heartbeat(_) => OpCode::Heartbeat,
                GatewayEvent::HeartbeatAck => OpCode::HeartbeatAck,
                GatewayEvent::Hello(_) => OpCode::Hello,
                GatewayEvent::InvalidateSession(_) => OpCode::InvalidSession,
                GatewayEvent::Reconnect => OpCode::Reconnect,
            }
        }

        let mut s = serializer.serialize_struct("GatewayEvent", 4)?;

        if let Self::Dispatch(sequence, event) = self {
            s.serialize_field("t", &event.kind())?;
            s.serialize_field("s", &sequence)?;
            s.serialize_field("op", &opcode(self))?;
            s.serialize_field("d", &event)?;

            return s.end();
        }

        // S and T are always null when not a Dispatch event
        s.serialize_field("t", &None::<&str>)?;
        s.serialize_field("s", &None::<u64>)?;
        s.serialize_field("op", &opcode(self))?;

        match self {
            Self::Dispatch(_, _) => unreachable!("dispatch already handled"),
            Self::Heartbeat(sequence) => {
                s.serialize_field("d", &sequence)?;
            }
            Self::Hello(interval) => {
                let hello = Hello {
                    heartbeat_interval: *interval,
                };

                s.serialize_field("d", &hello)?;
            }
            Self::InvalidateSession(invalidate) => {
                s.serialize_field("d", &invalidate)?;
            }
            Self::HeartbeatAck | Self::Reconnect => {
                s.serialize_field("d", &None::<u64>)?;
            }
        }

        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::{DispatchEvent, GatewayEvent, GatewayEventDeserializer, OpCode};
    use crate::{
        gateway::payload::RoleDelete,
        id::{GuildId, RoleId},
    };
    use serde::de::DeserializeSeed;
    use serde_json::de::Deserializer;
    use serde_test::Token;

    #[test]
    fn test_deserialize_dispatch_role_delete() {
        let input = r#"{
            "d": {
                "guild_id": "1",
                "role_id": "2"
            },
            "op": 0,
            "s": 7,
            "t": "GUILD_ROLE_DELETE"
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();
        assert!(matches!(event, GatewayEvent::Dispatch(7, _)));
    }

    #[test]
    fn test_deserialize_dispatch_guild_update() {
        let input = r#"{
  "d": {
    "afk_channel_id": "1337",
    "afk_timeout": 300,
    "application_id": null,
    "banner": null,
    "default_message_notifications": 0,
    "description": null,
    "discovery_splash": null,
    "emojis": [
      {
        "animated": false,
        "available": true,
        "id": "1338",
        "managed": false,
        "name": "goodboi",
        "require_colons": true,
        "roles": []
      }
    ],
    "explicit_content_filter": 0,
    "features": [
      "INVITE_SPLASH",
      "ANIMATED_ICON"
    ],
    "guild_id": "1339",
    "icon": "foobar",
    "id": "13310",
    "max_members": 250000,
    "max_presences": null,
    "mfa_level": 0,
    "name": "FooBaz",
    "nsfw_level": 1,
    "owner_id": "13311",
    "preferred_locale": "en-US",
    "premium_subscription_count": 4,
    "premium_tier": 1,
    "region": "eu-central",
    "roles": [
      {
        "color": 0,
        "hoist": false,
        "id": "13312",
        "managed": false,
        "mentionable": false,
        "name": "@everyone",
        "permissions": "104193601",
        "position": 0
      }
    ],
    "rules_channel_id": null,
    "splash": "barbaz",
    "system_channel_flags": 0,
    "system_channel_id": "13313",
    "vanity_url_code": null,
    "verification_level": 0,
    "widget_channel_id": null,
    "widget_enabled": false
  },
  "op": 0,
  "s": 42,
  "t": "GUILD_UPDATE"
}"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Dispatch(42, _)));
    }

    #[test]
    fn test_deserialize_dispatch_guild_update_2() {
        let input = r#"{
  "d": {
    "afk_channel_id": null,
    "afk_timeout": 300,
    "application_id": null,
    "banner": null,
    "default_message_notifications": 0,
    "description": null,
    "discovery_splash": null,
    "emojis": [
      {
        "animated": false,
        "available": true,
        "id": "42",
        "managed": false,
        "name": "emmet",
        "require_colons": true,
        "roles": []
      }
    ],
    "explicit_content_filter": 2,
    "features": [],
    "guild_id": "43",
    "icon": "44",
    "id": "45",
    "max_members": 250000,
    "max_presences": null,
    "mfa_level": 0,
    "name": "FooBar",
    "nsfw_level": 0,
    "owner_id": "46",
    "preferred_locale": "en-US",
    "premium_subscription_count": null,
    "premium_tier": 0,
    "region": "us-central",
    "roles": [
      {
        "color": 0,
        "hoist": false,
        "id": "47",
        "managed": false,
        "mentionable": false,
        "name": "@everyone",
        "permissions": "104324673",
        "position": 0
      }
    ],
    "rules_channel_id": null,
    "splash": null,
    "system_channel_flags": 0,
    "system_channel_id": "48",
    "vanity_url_code": null,
    "verification_level": 4,
    "widget_channel_id": null,
    "widget_enabled": true
  },
  "op": 0,
  "s": 1190911,
  "t": "GUILD_UPDATE"
}"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Dispatch(1_190_911, _)));
    }

    // Test that events which are not documented to have any data will not fail if
    // they contain it
    #[test]
    fn test_deserialize_dispatch_resumed() {
        let input = r#"{
  "t": "RESUMED",
  "s": 37448,
  "op": 0,
  "d": {
    "_trace": [
      "[\"gateway-prd-main-zqnl\",{\"micros\":11488,\"calls\":[\"discord-sessions-prd-1-38\",{\"micros\":1756}]}]"
    ]
  }
}"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Dispatch(_, _)));
    }

    #[test]
    fn test_deserialize_heartbeat() {
        let input = r#"{
            "t": null,
            "s": null,
            "op": 1,
            "d": 123
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Heartbeat(123)));
    }

    #[test]
    fn test_deserialize_heartbeat_ack() {
        let input = r#"{
            "t": null,
            "s": null,
            "op": 11,
            "d": null
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::HeartbeatAck));
    }

    #[test]
    fn test_deserialize_hello() {
        let input = r#"{
            "t": null,
            "s": null,
            "op": 10,
            "d": {
                "heartbeat_interval": 41250,
                "_trace": [
                    "[\"gateway-prd-main-mjmw\",{\"micros\":0.0}]"
                ]
            }
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Hello(41_250)));
    }

    #[test]
    fn test_deserialize_invalidate_session() {
        let input = r#"{
            "t": null,
            "s": null,
            "op": 9,
            "d": true
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::InvalidateSession(true)));
    }

    #[test]
    fn test_deserialize_reconnect() {
        let input = r#"{
            "t": null,
            "s": null,
            "op": 7,
            "d": null
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Reconnect));
    }

    /// Test that the deserializer won't mess up on a nested "t" in user input
    /// while searching for the event type.
    #[test]
    fn test_deserializer_from_json_nested_quotes() {
        let input = r#"{
            "t": "DOESNT_MATTER",
            "s": 5144,
            "op": 0,
            "d": {
                "name": "a \"t\"role"
            }
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        assert_eq!(deserializer.event_type, Some("DOESNT_MATTER"));
        assert_eq!(deserializer.op, 0);
    }

    // Test that the GatewayEventDeserializer handles non-string (read: null)
    // event types. For example HeartbeatAck
    #[allow(unused)]
    #[test]
    fn test_deserializer_handles_null_event_types() {
        let input = r#"{"t":null,"op":11}"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::HeartbeatAck));
    }

    #[test]
    fn test_serialize_dispatch() {
        let role_delete = RoleDelete {
            guild_id: GuildId::new(1).expect("non zero"),
            role_id: RoleId::new(2).expect("non zero"),
        };
        let dispatch = Box::new(DispatchEvent::RoleDelete(role_delete));
        let value = GatewayEvent::Dispatch(2_048, dispatch);

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GatewayEvent",
                    len: 4,
                },
                Token::Str("t"),
                Token::UnitVariant {
                    name: "EventType",
                    variant: "GUILD_ROLE_DELETE",
                },
                Token::Str("s"),
                Token::U64(2_048),
                Token::Str("op"),
                Token::U8(OpCode::Event as u8),
                Token::Str("d"),
                Token::Struct {
                    name: "RoleDelete",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("role_id"),
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("2"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_heartbeat() {
        serde_test::assert_ser_tokens(
            &GatewayEvent::Heartbeat(1024),
            &[
                Token::Struct {
                    name: "GatewayEvent",
                    len: 4,
                },
                Token::Str("t"),
                Token::None,
                Token::Str("s"),
                Token::None,
                Token::Str("op"),
                Token::U8(OpCode::Heartbeat as u8),
                Token::Str("d"),
                Token::U64(1024),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_heartbeat_ack() {
        serde_test::assert_ser_tokens(
            &GatewayEvent::HeartbeatAck,
            &[
                Token::Struct {
                    name: "GatewayEvent",
                    len: 4,
                },
                Token::Str("t"),
                Token::None,
                Token::Str("s"),
                Token::None,
                Token::Str("op"),
                Token::U8(OpCode::HeartbeatAck as u8),
                Token::Str("d"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_hello() {
        serde_test::assert_ser_tokens(
            &GatewayEvent::Hello(41250),
            &[
                Token::Struct {
                    name: "GatewayEvent",
                    len: 4,
                },
                Token::Str("t"),
                Token::None,
                Token::Str("s"),
                Token::None,
                Token::Str("op"),
                Token::U8(OpCode::Hello as u8),
                Token::Str("d"),
                Token::Struct {
                    name: "Hello",
                    len: 1,
                },
                Token::Str("heartbeat_interval"),
                Token::U64(41250),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_invalidate() {
        let value = GatewayEvent::InvalidateSession(true);

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GatewayEvent",
                    len: 4,
                },
                Token::Str("t"),
                Token::None,
                Token::Str("s"),
                Token::None,
                Token::Str("op"),
                Token::U8(OpCode::InvalidSession as u8),
                Token::Str("d"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_reconnect() {
        serde_test::assert_ser_tokens(
            &GatewayEvent::Reconnect,
            &[
                Token::Struct {
                    name: "GatewayEvent",
                    len: 4,
                },
                Token::Str("t"),
                Token::None,
                Token::Str("s"),
                Token::None,
                Token::Str("op"),
                Token::U8(OpCode::Reconnect as u8),
                Token::Str("d"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
