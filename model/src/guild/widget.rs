use crate::{
    gateway::presence::Status,
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidget {
    pub channels: Vec<GuildWidgetChannel>,
    pub id: GuildId,
    pub instant_invite: Option<String>,
    pub members: Vec<GuildWidgetMember>,
    pub name: String,
    pub presence_count: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetChannel {
    pub id: ChannelId,
    pub name: String,
    pub position: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetMember {
    pub activity: Option<GuildWidgetActivity>,
    pub avatar_url: Option<String>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    #[serde(rename = "username")]
    pub name: String,
    pub self_deaf: Option<bool>,
    pub self_mute: Option<bool>,
    pub status: Status,
    pub suppress: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetActivity {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{
        ChannelId, GuildId, GuildWidget, GuildWidgetActivity, GuildWidgetChannel,
        GuildWidgetMember, Status,
    };
    use serde_test::Token;

    #[test]
    fn test_guild_widget() {
        let value = GuildWidget {
            channels: vec![
                GuildWidgetChannel {
                    id: ChannelId::new(2).expect("non zero"),
                    name: "General".to_string(),
                    position: 1,
                }
            ],
            id: GuildId::new(1).expect("non zero"),
            instant_invite: Some("https://discord.com/invite/P8PkgN2".to_string()),
            members: vec![
                GuildWidgetMember {
                    activity: Some(GuildWidgetActivity {name: "Twilight".to_string()}),
                    avatar_url: Some("https://cdn.discordapp.com/widget-avatars/FfvURgcr3Za92K3JtoCppqnYMppMDc5B-Rll74YrGCU/C-1DyBZPQ6t5q2RuATFuMFgq0_uEMZVzd_6LbGN_uJKvZflobA9diAlTjhf6CAESLLeTuu4dLuHFWOb_PNLteooNfhC4C6k5QgAGuxEOP12tVVVCvX6t64k14PMXZrGTDq8pWZhukP40Wg".to_string()),
                    deaf: Some(false),
                    mute: Some(false),
                    name: "Foo".to_string(),
                    self_deaf: Some(false),
                    self_mute: Some(true),
                    status: Status::Online,
                    suppress: Some(false),
                }
            ],
            name: "Twilight".to_string(),
            presence_count: 15,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildWidget",
                    len: 6,
                },
                Token::Str("channels"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "GuildWidgetChannel", len: 3},
                Token::Str("id"),
                Token::NewtypeStruct {name: "ChannelId" },
                Token::Str("2"),
                Token::Str("name"),
                Token::Str("General"),
                Token::Str("position"),
                Token::I64(1),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct {name: "GuildId" },
                Token::Str("1"),
                Token::Str("instant_invite"),
                Token::Some,
                Token::Str("https://discord.com/invite/P8PkgN2"),
                Token::Str("members"),
                Token::Seq { len: Some(1) },
                Token::Struct { name: "GuildWidgetMember", len: 6},
                Token::Str("activity"),
                Token::Struct { name: "GuildWidgetActivity", len: 1},
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::StructEnd,
                Token::Str("avatar_url"),
                Token::Some,
                Token::Str("https://cdn.discordapp.com/widget-avatars/FfvURgcr3Za92K3JtoCppqnYMppMDc5B-Rll74YrGCU/C-1DyBZPQ6t5q2RuATFuMFgq0_uEMZVzd_6LbGN_uJKvZflobA9diAlTjhf6CAESLLeTuu4dLuHFWOb_PNLteooNfhC4C6k5QgAGuxEOP12tVVVCvX6t64k14PMXZrGTDq8pWZhukP40Wg"),
                Token::Str("deaf"),
                Token::Some,
                Token::Bool(false),
                Token::Str("mute"),
                Token::Some,
                Token::Bool(false),
                Token::Str("username"),
                Token::Str("Foo"),
                Token::Str("self_deaf"),
                Token::Some,
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(true),
                Token::Str("status"),
                Token::UnitVariant {
                    name: "Status",
                    variant: "online",
                },
                Token::StructEnd,
                Token::Str("suppress"),
                Token::Some,
                Token::Bool(false),
                Token::SeqEnd,
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::Str("presence_count"),
                Token::U64(15),
                Token::StructEnd,
            ],
        );
    }
}
