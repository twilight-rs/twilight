use twilight_cache_inmemory::CacheableStageInstance;
use twilight_model::{
    channel::StageInstance,
    id::{marker::StageMarker, Id},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedStageInstance {
    pub id: Id<StageMarker>,
}

impl From<StageInstance> for MinimalCachedStageInstance {
    fn from(stage: StageInstance) -> Self {
        Self { id: stage.id }
    }
}

impl PartialEq<StageInstance> for MinimalCachedStageInstance {
    fn eq(&self, other: &StageInstance) -> bool {
        self.id == other.id
    }
}

impl CacheableStageInstance for MinimalCachedStageInstance {}
