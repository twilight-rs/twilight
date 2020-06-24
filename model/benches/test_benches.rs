#![feature(test)]

extern crate test;

use twilight_model::gateway::payload::MemberChunk;

#[bench]
fn bench_member_chunk(b: &mut test::Bencher) {
    let input = r#"{
        "chunk_count": 1,
        "chunk_index": 0,
        "guild_id": "1",
        "members": [{
            "deaf": false,
            "hoisted_role": "6",
            "joined_at": "2020-04-04T04:04:04.000000+00:00",
            "mute": false,
            "nick": "chunk",
            "roles": ["6"],
            "user": {
                "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "discriminator": "0001",
                "id": "5",
                "public_flags": 131072,
                "username": "test"
            }
        }, {
            "deaf": false,
            "hoisted_role": "6",
            "joined_at": "2020-04-04T04:04:04.000000+00:00",
            "mute": false,
            "nick": "chunk",
            "roles": ["6"],
            "user": {
                "avatar": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                "discriminator": "0001",
                "id": "6",
                "username": "test"
            }
        }, {
            "deaf": false,
            "hoisted_role": "6",
            "joined_at": "2020-04-04T04:04:04.000000+00:00",
            "mute": false,
            "nick": "chunk",
            "roles": ["6"],
            "user": {
                "avatar": "cccccccccccccccccccccccccccccccc",
                "bot": true,
                "discriminator": "0001",
                "id": "3",
                "username": "test"
            }
        }, {
            "deaf": false,
            "hoisted_role": "6",
            "joined_at": "2020-04-04T04:04:04.000000+00:00",
            "mute": false,
            "nick": "chunk",
            "roles": [
                "6",
                "7"
            ],
            "user": {
                "avatar": "dddddddddddddddddddddddddddddddd",
                "bot": true,
                "discriminator": "0001",
                "id": "2",
                "username": "test"
            }
        }],
        "presences": [{
            "activities": [],
            "client_status": {
                "web": "online"
            },
            "game": null,
            "status": "online",
            "user": {
                "id": "2"
            }
        }, {
            "activities": [],
            "client_status": {
                "web": "online"
            },
            "game": null,
            "status": "online",
            "user": {
                "id": "3"
            }
        }, {
            "activities": [],
            "client_status": {
                "desktop": "dnd"
            },
            "game": null,
            "status": "dnd",
            "user": {
                "id": "5"
            }
        }]
    }"#;

    b.iter(|| {
        serde_json::from_str::<MemberChunk>(input).unwrap();
    });
}
