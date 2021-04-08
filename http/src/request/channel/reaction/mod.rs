pub mod get_reactions;

mod create_reaction;
mod delete_all_reaction;
mod delete_all_reactions;
mod delete_reaction;

pub use self::{
    create_reaction::CreateReaction, delete_all_reaction::DeleteAllReaction,
    delete_all_reactions::DeleteAllReactions, delete_reaction::DeleteReaction,
    get_reactions::GetReactions,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fmt::Write;
use twilight_model::{channel::ReactionType, id::EmojiId};

#[derive(Eq, PartialEq)]
pub enum RequestReactionType {
    Custom { id: EmojiId, name: Option<String> },
    Unicode { name: String },
}

impl From<ReactionType> for RequestReactionType {
    fn from(other: ReactionType) -> Self {
        match other {
            ReactionType::Custom { id, name, .. } => Self::Custom { id, name },
            ReactionType::Unicode { name } => Self::Unicode { name },
        }
    }
}

fn format_emoji(emoji: RequestReactionType) -> String {
    match emoji {
        RequestReactionType::Custom { id, name } => {
            let mut emoji = String::new();
            match name {
                Some(name) => emoji.push_str(name.as_ref()),
                None => emoji.push('e'),
            }
            let _ = write!(emoji, ":{}", id);
            emoji
        }
        RequestReactionType::Unicode { name } => {
            utf8_percent_encode(&name, NON_ALPHANUMERIC).to_string()
        }
    }
}
