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

use std::fmt::Write;
use twilight_model::channel::ReactionType;

fn format_emoji(emoji: ReactionType) -> String {
    match emoji {
        ReactionType::Custom { id, name, .. } => {
            let mut emoji = String::new();
            match name {
                Some(name) => emoji.push_str(&name),
                None => emoji.push_str("e"),
            }
            let _ = write!(emoji, ":{}", id);

            emoji
        }
        ReactionType::Unicode { name } => name,
    }
}
