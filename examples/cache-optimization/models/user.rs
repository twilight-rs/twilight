use twilight_cache_inmemory::CacheableUser;
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::User,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedUser {
    pub id: Id<UserMarker>,
}

impl From<User> for MinimalCachedUser {
    fn from(user: User) -> Self {
        Self { id: user.id }
    }
}

impl PartialEq<User> for MinimalCachedUser {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id
    }
}

impl CacheableUser for MinimalCachedUser {}
