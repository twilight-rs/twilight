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

use crate::client::ReactionType;
use std::fmt::Write;

fn format_emoji(emoji: ReactionType) -> String {
    match emoji {
        ReactionType::Custom { id, name } => {
            let mut emoji = String::new();
            match name {
                Some(name) => emoji.push_str(name.as_ref()),
                None => emoji.push_str("e"),
            }
            let _ = write!(emoji, ":{}", id);
            emoji
        }
        ReactionType::Unicode { name } => name,
    }
}
