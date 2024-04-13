use twilight_cache_inmemory::CacheableCurrentUser;
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::CurrentUser,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedCurrentUser {
    pub id: Id<UserMarker>,
}

impl From<CurrentUser> for MinimalCachedCurrentUser {
    fn from(value: CurrentUser) -> Self {
        Self { id: value.id }
    }
}

impl PartialEq<CurrentUser> for MinimalCachedCurrentUser {
    fn eq(&self, other: &CurrentUser) -> bool {
        self.id == other.id
    }
}

impl CacheableCurrentUser for MinimalCachedCurrentUser {
    fn id(&self) -> Id<UserMarker> {
        self.id
    }
}
