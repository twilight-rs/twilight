use super::{
    super::OpCode, DispatchEvent, DispatchEventWithTypeDeserializer, Event, EventConversionError,
};
use crate::gateway::payload::incoming::Hello;
use serde::{
    de::{
        value::U8Deserializer, DeserializeSeed, Deserializer, Error as DeError, IgnoredAny,
        IntoDeserializer, MapAccess, Unexpected, Visitor,
    },
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::{
    borrow::Cow,
    fmt::{Formatter, Result as FmtResult},
    str::FromStr,
};

/// An event from the gateway, which can either be a dispatch event with
/// stateful updates or a heartbeat, hello, etc. that a shard needs to operate.
#[derive(Clone, Debug)]
pub enum GatewayEvent {
    Dispatch(u64, DispatchEvent),
    Heartbeat,
    HeartbeatAck,
    Hello(Hello),
    InvalidateSession(bool),
    Reconnect,
}

impl TryFrom<Event> for GatewayEvent {
    type Error = EventConversionError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        Ok(match event {
            Event::GatewayHeartbeat => Self::Heartbeat,
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

/// Deserialize into a [`GatewayEvent`] by knowing its dispatch event type and
/// opcode.
#[derive(Debug)]
pub struct GatewayEventDeserializer<'a> {
    event_type: Option<Cow<'a, str>>,
    op: u8,
    sequence: Option<u64>,
}

impl<'a> GatewayEventDeserializer<'a> {
    /// Create a new gateway event deserializer when you already know the opcode
    /// and dispatch event type.
    pub fn new(op: u8, event_type: Option<&'a str>) -> Self {
        Self {
            event_type: event_type.map(Into::into),
            op,
            sequence: None,
        }
    }

    /// Create a gateway event deserializer by scanning the JSON payload for its
    /// opcode and dispatch event type.
    pub fn from_json(input: &'a str) -> Option<Self> {
        let op = Self::find_opcode(input)?;
        let event_type = Self::find_event_type(input).map(Into::into);
        let sequence = Self::find_sequence(input);

        Some(Self {
            event_type,
            op,
            sequence,
        })
    }

    /// Create a deserializer with an owned event type.
    ///
    /// This is necessary when using a mutable deserialization library such as
    /// `simd-json`.
    pub fn into_owned(self) -> GatewayEventDeserializer<'static> {
        GatewayEventDeserializer {
            event_type: self
                .event_type
                .map(|event_type| Cow::Owned(event_type.into_owned())),
            op: self.op,
            sequence: self.sequence,
        }
    }

    /// Consume the deserializer, returning its components.
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_parts(self) -> (u8, Option<u64>, Option<Cow<'a, str>>) {
        (self.op, self.sequence, self.event_type)
    }

    /// Dispatch event type of the payload.
    pub fn event_type(&self) -> Option<&str> {
        self.event_type.as_deref()
    }

    /// Opcode of the payload.
    pub const fn op(&self) -> u8 {
        self.op
    }

    /// Sequence of the payload.
    ///
    /// May only be available if the deserializer was created via
    /// [`from_json`][`Self::from_json`]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
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

struct GatewayEventVisitor<'a>(u8, Option<Cow<'a, str>>);

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
        while let Ok(Some(_)) | Err(_) = map.next_key::<Field>() {
            map.next_value::<IgnoredAny>()?;
        }

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

        let op_deser: U8Deserializer<V::Error> = self.0.into_deserializer();

        let op = OpCode::deserialize(op_deser).ok().ok_or_else(|| {
            let unexpected = Unexpected::Unsigned(u64::from(self.0));

            DeError::invalid_value(unexpected, &"an opcode")
        })?;

        Ok(match op {
            OpCode::Dispatch => {
                let t = self
                    .1
                    .ok_or_else(|| DeError::custom("event type not provided beforehand"))?;

                let mut d = None;
                let mut s = None;

                loop {
                    let key = match map.next_key() {
                        Ok(Some(key)) => key,
                        Ok(None) => break,
                        Err(_) => {
                            map.next_value::<IgnoredAny>()?;

                            continue;
                        }
                    };

                    match key {
                        Field::D => {
                            if d.is_some() {
                                return Err(DeError::duplicate_field("d"));
                            }

                            let deserializer = DispatchEventWithTypeDeserializer::new(&t);

                            d = Some(map.next_value_seed(deserializer)?);
                        }
                        Field::S => {
                            if s.is_some() {
                                return Err(DeError::duplicate_field("s"));
                            }

                            s = Some(map.next_value()?);
                        }
                        Field::Op | Field::T => {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let s = s.ok_or_else(|| DeError::missing_field("s"))?;

                GatewayEvent::Dispatch(s, d)
            }
            OpCode::Heartbeat => {
                Self::ignore_all(&mut map)?;

                GatewayEvent::Heartbeat
            }
            OpCode::HeartbeatAck => {
                Self::ignore_all(&mut map)?;

                GatewayEvent::HeartbeatAck
            }
            OpCode::Hello => {
                let hello = Self::field::<Hello, _>(&mut map, Field::D)?;

                Self::ignore_all(&mut map)?;

                GatewayEvent::Hello(hello)
            }
            OpCode::InvalidSession => {
                let invalidate = Self::field::<bool, _>(&mut map, Field::D)?;

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
            OpCode::VoiceStateUpdate => {
                return Err(DeError::unknown_variant("VoiceStateUpdate", VALID_OPCODES))
            }
        })
    }
}

impl<'de> DeserializeSeed<'de> for GatewayEventDeserializer<'_> {
    type Value = GatewayEvent;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        const FIELDS: &[&str] = &["op", "d", "s", "t"];

        deserializer.deserialize_struct(
            "GatewayEvent",
            FIELDS,
            GatewayEventVisitor(self.op, self.event_type),
        )
    }
}

impl Serialize for GatewayEvent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        const fn opcode(gateway_event: &GatewayEvent) -> OpCode {
            match gateway_event {
                GatewayEvent::Dispatch(_, _) => OpCode::Dispatch,
                GatewayEvent::Heartbeat => OpCode::Heartbeat,
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
            Self::Hello(hello) => {
                s.serialize_field("d", &hello)?;
            }
            Self::InvalidateSession(invalidate) => {
                s.serialize_field("d", &invalidate)?;
            }
            Self::Heartbeat | Self::HeartbeatAck | Self::Reconnect => {
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
        gateway::payload::incoming::{Hello, RoleDelete},
        id::Id,
        test::image_hash,
    };
    use serde::de::DeserializeSeed;
    use serde_json::de::Deserializer;
    use serde_test::Token;

    #[test]
    fn deserialize_dispatch_role_delete() {
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
    fn deserialize_dispatch_guild_update() {
        let input = format!(
            r#"{{
  "d": {{
    "afk_channel_id": "1337",
    "afk_timeout": 300,
    "application_id": null,
    "banner": null,
    "default_message_notifications": 0,
    "description": null,
    "discovery_splash": null,
    "emojis": [
      {{
        "animated": false,
        "available": true,
        "id": "1338",
        "managed": false,
        "name": "goodboi",
        "require_colons": true,
        "roles": []
      }}
    ],
    "explicit_content_filter": 0,
    "features": [
      "INVITE_SPLASH",
      "ANIMATED_ICON"
    ],
    "guild_id": "1339",
    "icon": "{icon}",
    "id": "13310",
    "max_members": 250000,
    "max_presences": null,
    "mfa_level": 0,
    "name": "FooBaz",
    "nsfw_level": 1,
    "owner_id": "13311",
    "preferred_locale": "en-US",
    "premium_progress_bar_enabled": true,
    "premium_subscription_count": 4,
    "premium_tier": 1,
    "region": "eu-central",
    "roles": [
      {{
        "color": 0,
        "hoist": false,
        "id": "13312",
        "managed": false,
        "mentionable": false,
        "name": "@everyone",
        "permissions": "104193601",
        "position": 0,
        "flags": 0
      }}
    ],
    "rules_channel_id": null,
    "splash": "{splash}",
    "system_channel_flags": 0,
    "system_channel_id": "13313",
    "vanity_url_code": null,
    "verification_level": 0,
    "widget_channel_id": null,
    "widget_enabled": false
  }},
  "op": 0,
  "s": 42,
  "t": "GUILD_UPDATE"
}}"#,
            icon = image_hash::ICON_INPUT,
            splash = image_hash::SPLASH_INPUT,
        );

        let deserializer = GatewayEventDeserializer::from_json(&input).unwrap();
        let mut json_deserializer = Deserializer::from_str(&input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Dispatch(42, _)));
    }

    #[test]
    fn deserialize_dispatch_guild_update_2() {
        let input = format!(
            r#"{{
  "d": {{
    "afk_channel_id": null,
    "afk_timeout": 300,
    "application_id": null,
    "banner": null,
    "default_message_notifications": 0,
    "description": null,
    "discovery_splash": null,
    "emojis": [
      {{
        "animated": false,
        "available": true,
        "id": "42",
        "managed": false,
        "name": "emmet",
        "require_colons": true,
        "roles": []
      }}
    ],
    "explicit_content_filter": 2,
    "features": [],
    "guild_id": "43",
    "icon": "{icon}",
    "id": "45",
    "max_members": 250000,
    "max_presences": null,
    "mfa_level": 0,
    "name": "FooBar",
    "nsfw_level": 0,
    "owner_id": "46",
    "preferred_locale": "en-US",
    "premium_progress_bar_enabled": false,
    "premium_subscription_count": null,
    "premium_tier": 0,
    "region": "us-central",
    "roles": [
      {{
        "color": 0,
        "hoist": false,
        "id": "47",
        "managed": false,
        "mentionable": false,
        "name": "@everyone",
        "permissions": "104324673",
        "position": 0,
        "flags": 0
      }}
    ],
    "rules_channel_id": null,
    "splash": null,
    "system_channel_flags": 0,
    "system_channel_id": "48",
    "vanity_url_code": null,
    "verification_level": 4,
    "widget_channel_id": null,
    "widget_enabled": true
  }},
  "op": 0,
  "s": 1190911,
  "t": "GUILD_UPDATE"
}}"#,
            icon = image_hash::ICON_INPUT
        );

        let deserializer = GatewayEventDeserializer::from_json(&input).unwrap();
        let mut json_deserializer = Deserializer::from_str(&input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Dispatch(1_190_911, _)));
    }

    // Test that events which are not documented to have any data will not fail if
    // they contain it
    #[test]
    fn deserialize_dispatch_resumed() {
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
    fn deserialize_heartbeat() {
        let input = r#"{
            "t": null,
            "s": null,
            "op": 1,
            "d": null
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::Heartbeat));
    }

    #[test]
    fn deserialize_heartbeat_ack() {
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
    fn deserialize_hello() {
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

        assert!(matches!(
            event,
            GatewayEvent::Hello(Hello {
                heartbeat_interval: 41_250
            })
        ));
    }

    #[test]
    fn deserialize_invalidate_session() {
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
    fn deserialize_reconnect() {
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
    fn deserializer_from_json_nested_quotes() {
        let input = r#"{
            "t": "DOESNT_MATTER",
            "s": 5144,
            "op": 0,
            "d": {
                "name": "a \"t\"role"
            }
        }"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        assert_eq!(deserializer.event_type(), Some("DOESNT_MATTER"));
        assert_eq!(deserializer.op, 0);
    }

    // Test that the GatewayEventDeserializer handles non-string (read: null)
    // event types. For example HeartbeatAck
    #[allow(unused)]
    #[test]
    fn deserializer_handles_null_event_types() {
        let input = r#"{"t":null,"op":11}"#;

        let deserializer = GatewayEventDeserializer::from_json(input).unwrap();
        let mut json_deserializer = Deserializer::from_str(input);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert!(matches!(event, GatewayEvent::HeartbeatAck));
    }

    #[test]
    fn serialize_dispatch() {
        let role_delete = RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(2),
        };
        let value = GatewayEvent::Dispatch(2_048, DispatchEvent::RoleDelete(role_delete));

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
                Token::U8(OpCode::Dispatch as u8),
                Token::Str("d"),
                Token::Struct {
                    name: "RoleDelete",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("role_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serialize_heartbeat() {
        serde_test::assert_ser_tokens(
            &GatewayEvent::Heartbeat,
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
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serialize_heartbeat_ack() {
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
    fn serialize_hello() {
        serde_test::assert_ser_tokens(
            &GatewayEvent::Hello(Hello {
                heartbeat_interval: 41250,
            }),
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
    fn serialize_invalidate() {
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
    fn serialize_reconnect() {
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
