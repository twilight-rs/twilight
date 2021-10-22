use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{
    channel::StageInstance,
    gateway::payload::incoming::{StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate},
    id::{GuildId, StageId},
};

impl InMemoryCache {
    pub(crate) fn cache_stage_instances(
        &self,
        guild_id: GuildId,
        stage_instances: impl IntoIterator<Item = StageInstance>,
    ) {
        for stage_instance in stage_instances {
            self.cache_stage_instance(guild_id, stage_instance);
        }
    }

    fn cache_stage_instance(&self, guild_id: GuildId, stage_instance: StageInstance) {
        self.guild_stage_instances
            .entry(guild_id)
            .or_default()
            .insert(stage_instance.id);

        crate::upsert_guild_item(
            &self.stage_instances,
            guild_id,
            stage_instance.id,
            stage_instance,
        );
    }

    fn delete_stage_instance(&self, stage_id: StageId) {
        if let Some((_, data)) = self.stage_instances.remove(&stage_id) {
            let guild_id = data.guild_id;

            if let Some(mut stage_instances) = self.guild_stage_instances.get_mut(&guild_id) {
                stage_instances.remove(&stage_id);
            }
        }
    }
}

impl UpdateCache for StageInstanceCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::STAGE_INSTANCE) {
            return;
        }

        cache.cache_stage_instance(self.guild_id, self.0.clone());
    }
}

impl UpdateCache for StageInstanceDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::STAGE_INSTANCE) {
            return;
        }

        cache.delete_stage_instance(self.id);
    }
}

impl UpdateCache for StageInstanceUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::STAGE_INSTANCE) {
            return;
        }

        cache.cache_stage_instance(self.guild_id, self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_model::{channel::stage_instance::PrivacyLevel, id::ChannelId};

    #[test]
    fn test_stage_channels() {
        let cache = InMemoryCache::new();

        let stage_instance = StageInstance {
            channel_id: ChannelId::new(1).expect("non zero"),
            discoverable_disabled: true,
            guild_id: GuildId::new(2).expect("non zero"),
            id: StageId::new(3).expect("non zero"),
            privacy_level: PrivacyLevel::GuildOnly,
            topic: "topic".into(),
        };

        cache.update(&StageInstanceCreate(stage_instance.clone()));

        {
            let cached_instances = cache
                .guild_stage_instances(stage_instance.guild_id)
                .unwrap();
            assert_eq!(1, cached_instances.len());
        }

        {
            let cached_instance = cache.stage_instance(stage_instance.id).unwrap();
            assert_eq!(stage_instance.topic, cached_instance.topic);
        }

        let new_stage_instance = StageInstance {
            topic: "a new topic".into(),
            ..stage_instance
        };

        cache.update(&StageInstanceUpdate(new_stage_instance.clone()));

        {
            let cached_instance = cache.stage_instance(stage_instance.id).unwrap();
            assert_ne!(stage_instance.topic, cached_instance.topic);
            assert_eq!(new_stage_instance.topic, "a new topic");
        }

        cache.update(&StageInstanceDelete(new_stage_instance));

        {
            let cached_instances = cache
                .guild_stage_instances(stage_instance.guild_id)
                .unwrap();
            assert_eq!(0, cached_instances.len());
        }

        {
            let cached_instance = cache.stage_instance(stage_instance.id);
            assert!(cached_instance.is_none());
        }
    }
}
