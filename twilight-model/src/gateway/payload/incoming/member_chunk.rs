use serde::{Deserialize, Serialize};

use crate::{
    gateway::presence::Presence,
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MemberChunk {
    pub chunk_count: u32,
    pub chunk_index: u32,
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(default)]
    pub not_found: Vec<Id<UserMarker>>,
    #[serde(default)]
    pub presences: Vec<Presence>,
}

#[cfg(test)]
mod tests {
    use super::super::MemberChunk;
    use crate::{
        gateway::presence::{ClientStatus, Presence, Status, UserOrId},
        guild::{Member, MemberFlags},
        id::Id,
        test::image_hash,
        user::{User, UserFlags},
        util::datetime::{Timestamp, TimestampParseError},
    };
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn simple_member_chunk() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-04-04T04:04:04.000000+00:00")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let input = serde_json::json!({
            "chunk_count": 1,
            "chunk_index": 0,
            "guild_id": "1",
            "members": [{
                "communication_disabled_until": null,
                "deaf": false,
                "flags": flags.bits(),
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "pending": true,
                "roles": ["6"],
                "user": {
                    "avatar": image_hash::AVATAR_INPUT,
                    "discriminator": "0001",
                    "id": "5",
                    "public_flags": 131_072,
                    "username": "test",
                },
            }, {
                "communication_disabled_until": null,
                "deaf": false,
                "flags": flags.bits(),
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": ["6"],
                "user": {
                    "avatar": image_hash::AVATAR_INPUT,
                    "discriminator": "0001",
                    "id": "6",
                    "username": "test",
                },
            }, {
                "communication_disabled_until": null,
                "deaf": false,
                "flags": flags.bits(),
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": ["6"],
                "user": {
                    "avatar": image_hash::AVATAR_INPUT,
                    "bot": true,
                    "discriminator": "0001",
                    "id": "3",
                    "username": "test",
                },
            }, {
                "communication_disabled_until": null,
                "deaf": false,
                "flags": flags.bits(),
                "hoisted_role": "6",
                "joined_at": "2020-04-04T04:04:04.000000+00:00",
                "mute": false,
                "nick": "chunk",
                "roles": [
                    "6",
                    "7",
                ],
                "user": {
                    "avatar": image_hash::AVATAR_INPUT,
                    "bot": true,
                    "discriminator": "0001",
                    "id": "2",
                    "username": "test",
                },
            }],
            "presences": [{
                "activities": [],
                "client_status": {
                    "web": "online",
                },
                "guild_id": "1",
                "status": "online",
                "user": {
                    "id": "2",
                },
            }, {
                "activities": [],
                "client_status": {
                    "web": "online",
                },
                "guild_id": "1",
                "status": "online",
                "user": {
                    "id": "3",
                },
            }, {
                "activities": [],
                "client_status": {
                    "desktop": "dnd",
                },
                "guild_id": "1",
                "status": "dnd",
                "user": {
                    "id": "5",
                },
            }],
        });

        let expected = MemberChunk {
            chunk_count: 1,
            chunk_index: 0,
            guild_id: Id::new(1),
            members: Vec::from([
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    flags,
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: false,
                    premium_since: None,
                    roles: vec![Id::new(6), Id::new(7)],
                    user: User {
                        id: Id::new(2),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: true,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: None,
                    },
                },
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    flags,
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: false,
                    premium_since: None,
                    roles: vec![Id::new(6)],
                    user: User {
                        id: Id::new(3),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: true,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: None,
                    },
                },
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    flags,
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: true,
                    premium_since: None,
                    roles: vec![Id::new(6)],
                    user: User {
                        id: Id::new(5),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: false,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: Some(UserFlags::VERIFIED_DEVELOPER),
                    },
                },
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    flags,
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: false,
                    premium_since: None,
                    roles: vec![Id::new(6)],
                    user: User {
                        id: Id::new(6),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: false,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: None,
                    },
                },
            ]),
            nonce: None,
            not_found: Vec::new(),
            presences: Vec::from([
                Presence {
                    activities: Vec::new(),
                    client_status: ClientStatus {
                        desktop: None,
                        mobile: None,
                        web: Some(Status::Online),
                    },
                    guild_id: Some(Id::new(1)),
                    status: Status::Online,
                    user: UserOrId::UserId { id: Id::new(2) },
                },
                Presence {
                    activities: Vec::new(),
                    client_status: ClientStatus {
                        desktop: None,
                        mobile: None,
                        web: Some(Status::Online),
                    },
                    guild_id: Some(Id::new(1)),
                    status: Status::Online,
                    user: UserOrId::UserId { id: Id::new(3) },
                },
                Presence {
                    activities: Vec::new(),
                    client_status: ClientStatus {
                        desktop: Some(Status::DoNotDisturb),
                        mobile: None,
                        web: None,
                    },
                    guild_id: Some(Id::new(1)),
                    status: Status::DoNotDisturb,
                    user: UserOrId::UserId { id: Id::new(5) },
                },
            ]),
        };

        let actual = serde_json::from_value::<MemberChunk>(input).unwrap();
        assert_eq!(expected.chunk_count, actual.chunk_count);
        assert_eq!(expected.chunk_index, actual.chunk_index);
        assert_eq!(expected.guild_id, actual.guild_id);
        assert_eq!(expected.nonce, actual.nonce);
        assert_eq!(expected.not_found, actual.not_found);

        for member in &actual.members {
            assert!(expected.members.iter().any(|m| m == member));
        }

        for presences in &actual.presences {
            assert!(expected.presences.iter().any(|p| p == presences));
        }

        Ok(())
    }
}
