use twilight_cache_inmemory::CacheableMessage;
use twilight_model::{
    channel::{message::Reaction, Message},
    id::{marker::MessageMarker, Id},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedMessage {
    pub id: Id<MessageMarker>,
    pub content: String,
}

impl From<Message> for MinimalCachedMessage {
    fn from(message: Message) -> Self {
        Self {
            id: message.id,
            content: message.content,
        }
    }
}

impl PartialEq<Message> for MinimalCachedMessage {
    fn eq(&self, other: &Message) -> bool {
        self.id == other.id && self.content == other.content
    }
}

impl CacheableMessage for MinimalCachedMessage {
    fn add_reaction(&mut self, _reaction: Reaction) {
        // No-op
    }

    fn clear_reactions(&mut self) {
        // No-op
    }

    fn reactions(&self) -> &[Reaction] {
        &[]
    }

    fn reactions_mut(&mut self) -> &mut [Reaction] {
        &mut []
    }

    fn remove_reaction(&mut self, _idx: usize) {
        // No-op
    }

    fn retain_reactions(&mut self, _f: impl FnMut(&Reaction) -> bool) {
        // No-op
    }
}
