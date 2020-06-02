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
        ReactionType::Custom { animated, id, name } => {
            let mut emoji = String::from("<");

            if animated {
                emoji.push('a');
            }

            emoji.push(':');

            if let Some(name) = name {
                emoji.push_str(name.as_ref());
            }

            emoji.push(':');
            let _ = write!(emoji, "{}", id);
            emoji.push('>');

            emoji
        }
        ReactionType::Unicode { name } => name,
    }
}
