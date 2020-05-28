use super::super::OpCode;
use super::{DispatchEvent, DispatchEventWithTypeDeserializer};
use serde::{
    de::{Deserialize, DeserializeSeed, Deserializer, Error as DeError, MapAccess, Visitor},
    Deserialize as DeserializeMacro,
};
use serde_value::Value;
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

#[derive(DeserializeMacro)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    D,
    Op,
    S,
    T,
}

struct GatewayEventVisitor;

impl<'de> Visitor<'de> for GatewayEventVisitor {
    type Value = GatewayEvent;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("struct GatewayEvent")
    }

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

        // Have to use a serde_json::Value here because serde has no
        // abstract container type.
        let mut d = None::<Value>;
        let mut op = None::<OpCode>;
        let mut s = None::<u64>;
        let mut t = None::<String>;

        while let Some(key) = map.next_key()? {
            match key {
                Field::D => {
                    if d.is_some() {
                        return Err(DeError::duplicate_field("d"));
                    }

                    d = Some(map.next_value()?);
                }
                Field::Op => {
                    if op.is_some() {
                        return Err(DeError::duplicate_field("op"));
                    }

                    op = Some(map.next_value()?);
                }
                Field::S => {
                    if s.is_some() {
                        return Err(DeError::duplicate_field("s"));
                    }

                    s = map.next_value::<Option<_>>()?;
                }
                Field::T => {
                    if t.is_some() {
                        return Err(DeError::duplicate_field("t"));
                    }

                    t = map.next_value::<Option<_>>()?;
                }
            }
        }

        let op = op.ok_or_else(|| DeError::missing_field("op"))?;

        Ok(match op {
            OpCode::Event => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let s = s.ok_or_else(|| DeError::missing_field("s"))?;
                let t = t.ok_or_else(|| DeError::missing_field("t"))?;
                let event_deserialize = DispatchEventWithTypeDeserializer::new(t.as_ref());
                let dispatch = event_deserialize.deserialize(d).map_err(DeError::custom)?;

                GatewayEvent::Dispatch(s, Box::new(dispatch))
            }
            OpCode::Heartbeat => {
                let s = s.ok_or_else(|| DeError::missing_field("s"))?;

                GatewayEvent::Heartbeat(s)
            }
            OpCode::HeartbeatAck => GatewayEvent::HeartbeatAck,
            OpCode::Hello => {
                #[derive(DeserializeMacro)]
                struct Hello {
                    heartbeat_interval: u64,
                }

                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let hello = Hello::deserialize(d).map_err(DeError::custom)?;

                GatewayEvent::Hello(hello.heartbeat_interval)
            }
            OpCode::InvalidSession => {
                let d = d.ok_or_else(|| DeError::missing_field("d"))?;
                let resumeable = bool::deserialize(d).map_err(DeError::custom)?;

                GatewayEvent::InvalidateSession(resumeable)
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

impl<'de> Deserialize<'de> for GatewayEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &["d", "op", "s", "t"];

        deserializer.deserialize_struct("GatewayEvent", FIELDS, GatewayEventVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guild() {
        let broken_guild = r#"{
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

        serde_json::from_str::<GatewayEvent>(broken_guild).unwrap();
    }

    #[test]
    fn test_guild_2() {
        let broken_guild = r#"{
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
        serde_json::from_str::<GatewayEvent>(broken_guild).unwrap();
    }
}
