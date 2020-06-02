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

use twilight_model::channel::ReactionType;

pub fn format_emoji(emoji: ReactionType) -> String {
    match emoji {
        ReactionType::Custom { animated, id, name } => match animated {
            true => format!("<a:{}:{}>", name.unwrap_or("".into()), id).to_string(),
            false => format!("<:{}:{}>", name.unwrap_or("".into()), id).to_string(),
        },
        ReactionType::Unicode { name } => name,
    }
}
