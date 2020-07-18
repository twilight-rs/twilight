use super::super::OpCode;
use super::{DispatchEvent, DispatchEventWithTypeDeserializer};
use serde::{
    de::{
        value::U8Deserializer, DeserializeSeed, Deserializer, Error as DeError, IgnoredAny,
        IntoDeserializer, MapAccess, Unexpected, Visitor,
    },
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    D,
    Op,
    S,
    T,
}

/// A deserializer that deserializes into a `GatewayEvent` by cloning some bits
/// of scanned information before the actual deserialisation.
///
/// This is the owned version of [`GatewayEventDeserializer`].
///
/// You should use this if you're using a mutable deserialization library
/// like `simd-json`.
///
/// [`GatewayEventDeserializer`]: struct.GatewayEventDeserializer.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayEventDeserializerOwned {
    event_type: Option<String>,
    op: u8,
}

impl GatewayEventDeserializerOwned {
    pub fn from_json(input: &str) -> Option<Self> {
        let deser = GatewayEventDeserializer::from_json(input)?;
        let GatewayEventDeserializer { event_type, op } = deser;

        Some(Self {
            event_type: event_type.map(ToOwned::to_owned),
            op,
        })
    }
}

/// A deserializer that deserializes into a `GatewayEvent` by borrowing some bits
/// of scanned information before the actual deserialisation.
///
/// This is the borrowed version of [`GatewayEventDeserializerOwned`].
///
/// You should use this if you're using an immutable deserialization library
/// like `serde_json`.
///
/// [`GatewayEventDeserializerOwned`]: struct.GatewayEventDeserializerOwned.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatewayEventDeserializer<'a> {
    event_type: Option<&'a str>,
    op: u8,
}

impl<'a> GatewayEventDeserializer<'a> {
    // Create a gateway event deserializer with some information found by
    // scanning the JSON payload to deserialise.
    //
    // This will scan the payload for the opcode and, optionally, event type if
    // provided. The opcode key ("op"), must be in the payload while the event
    // type key ("t") is optional and only required for event ops.
    pub fn from_json(input: &'a str) -> Option<Self> {
        let op = Self::find_opcode(input)?;
        let event_type = Self::find_event_type(input);

        Some(Self { event_type, op })
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
        // Find the op key's position and then search for where the first
        // character that's not base 10 is. This'll give us the bytes with the
        // op which can be parsed.
        //
        // Add 5 at the end since that's the length of what we're finding.
        let from = input.find(r#""op":"#)? + 5;

        // Look for the first thing that isn't a base 10 digit or whitespace,
        // i.e. a comma (denoting another JSON field), curly brace (end of the
        // object), etc. This'll give us the op number, maybe with a little
        // whitespace.
        let to = input.get(from..)?.find(&[',', '}'] as &[_])?;
        // We might have some whitespace, so let's trim this.
        let clean = input.get(from..from + to)?.trim();

        clean.parse::<u8>().ok()
    }
}

struct GatewayEventVisitor<'a>(u8, Option<&'a str>);

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
            OpCode::Event => {
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

                            let deserializer = DispatchEventWithTypeDeserializer::new(t);

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

                GatewayEvent::Dispatch(s, Box::new(d))
            }
            OpCode::Heartbeat => {
                let seq = Self::field(&mut map, Field::S)?;

                GatewayEvent::Heartbeat(seq)
            }
            OpCode::HeartbeatAck => {
                while let Ok(Some(_)) | Err(_) = map.next_key::<Field>() {
                    map.next_value::<IgnoredAny>()?;
                }
                GatewayEvent::HeartbeatAck
            }
            OpCode::Hello => {
                #[derive(Deserialize)]
                struct Hello {
                    heartbeat_interval: u64,
                }

                let hello = Self::field::<Hello, _>(&mut map, Field::D)?;

                GatewayEvent::Hello(hello.heartbeat_interval)
            }
            OpCode::InvalidSession => {
                let invalidate = Self::field::<bool, _>(&mut map, Field::D)?;

                GatewayEvent::InvalidateSession(invalidate)
            }
            OpCode::Identify => return Err(DeError::unknown_variant("Identify", VALID_OPCODES)),
            OpCode::Reconnect => GatewayEvent::Reconnect,
            OpCode::RequestGuildMembers => {
                return Err(DeError::unknown_variant(
                    "RequestGuildMembers",
                    VALID_OPCODES,
                ))
            }
            OpCode::Resume => return Err(DeError::unknown_variant("Resume", VALID_OPCODES)),
            OpCode::StatusUpdate => {
                return Err(DeError::unknown_variant("StatusUpdate", VALID_OPCODES))
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
            GatewayEventVisitor(self.op, self.event_type.as_deref()),
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
            GatewayEventVisitor(self.op, self.event_type.as_deref()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{GatewayEvent, GatewayEventDeserializer};
    use serde::de::DeserializeSeed;
    use serde_json::de::Deserializer;

    #[test]
    fn test_deserializer_constructor() {
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
    fn test_guild() {
        let input = r#"{
  "d": {
    "afk_channel_id": "1337",
    "afk_timeout": 300,
    "application_id": null,
    "banner": null,
    "default_message_notifications": 0,
    "description": null,
    "discovery_splash": null,
    "embed_channel_id": null,
    "embed_enabled": false,
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
        "permissions": 104193601,
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
    fn test_guild_2() {
        let input = r#"{
  "d": {
    "afk_channel_id": null,
    "afk_timeout": 300,
    "application_id": null,
    "banner": null,
    "default_message_notifications": 0,
    "description": null,
    "discovery_splash": null,
    "embed_channel_id": null,
    "embed_enabled": true,
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
        "permissions": 104324673,
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

        assert!(matches!(event, GatewayEvent::Dispatch(1190911, _)));
    }

    #[test]
    fn hello() {
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

        assert!(matches!(event, GatewayEvent::Hello(41250)));
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

    // Test that events which are not documented to have any data will not fail if
    // they contain it
    #[allow(unused)]
    #[test]
    fn test_deserializer_handles_resumed() {
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
}
