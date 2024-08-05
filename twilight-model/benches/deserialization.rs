use criterion::{criterion_group, criterion_main, Criterion};

use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use twilight_model::gateway::{
    event::GatewayEventDeserializer,
    payload::incoming::{MemberChunk, TypingStart},
    GatewayReaction,
};

fn gateway_event_role_delete() {
    let input = r#"{
        "op": 0,
        "s": 2,
        "d": {
            "guild_id": "1",
            "role_id": "2"
        },
        "t": "GUILD_ROLE_DELETE"
    }"#;

    let mut json_deserializer = Deserializer::from_str(input);
    let gateway_deserializer = GatewayEventDeserializer::from_json(input).unwrap();
    gateway_deserializer
        .deserialize(&mut json_deserializer)
        .unwrap();
}

fn member_chunk() {
    let input = r#"{
        "chunk_count": 1,
        "chunk_index": 0,
        "guild_id": "1",
        "members": [{
            "deaf": false,
            "hoisted_role": "6",
            "flags": 0,
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
            "flags": 0,
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
            "flags": 0,
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
            "flags": 0,
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
            "status": "online",
            "user": {
                "id": "2"
            }
        }, {
            "activities": [],
            "client_status": {
                "web": "online"
            },
            "status": "online",
            "user": {
                "id": "3"
            }
        }, {
            "activities": [],
            "client_status": {
                "desktop": "dnd"
            },
            "status": "dnd",
            "user": {
                "id": "5"
            }
        }]
    }"#;

    serde_json::from_str::<MemberChunk>(input).unwrap();
}

fn reaction() {
    let input = r#"{
        "channel_id": "2",
        "emoji": {
            "id": null,
            "name": "🙂"
        },
        "guild_id": "1",
        "member": {
            "deaf": false,
            "hoisted_role": "5",
            "flags": 0,
            "joined_at": "2020-01-01T00:00:00.000000+00:00",
            "mute": false,
            "nick": "typing",
            "roles": ["5"],
            "user": {
                "username": "test",
                "id": "4",
                "discriminator": "0001",
                "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }
        },
        "message_id": "3",
        "user_id": "4"
    }"#;

    serde_json::from_str::<GatewayReaction>(input).unwrap();
}

fn typing_start() {
    let input = r#"{
        "channel_id": "2",
        "guild_id": "1",
        "member": {
            "deaf": false,
            "hoisted_role": "4",
            "flags": 0,
            "joined_at": "2020-01-01T00:00:00.000000+00:00",
            "mute": false,
            "nick": "typing",
            "roles": ["4"],
            "user": {
                "username": "test",
                "id": "3",
                "discriminator": "0001",
                "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }
        },
        "timestamp": 1500000000,
        "user_id": "3"
    }"#;

    serde_json::from_str::<TypingStart>(input).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gateway event role delete", |b| {
        b.iter(gateway_event_role_delete)
    });
    c.bench_function("member chunk", |b| b.iter(member_chunk));
    c.bench_function("reaction", |b| b.iter(reaction));
    c.bench_function("typing start", |b| b.iter(typing_start));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
