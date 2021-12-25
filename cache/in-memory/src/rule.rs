//! Rules for accepting entities into the cache.

use crate::InMemoryCache;
use std::fmt::Debug;
use twilight_model::{
    channel::Channel,
    guild::{Emoji, GuildIntegration, Member, Role},
    user::User,
    voice::VoiceState,
};

/// Entity being checked on whether it should be accepted into the cache.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Entity<'a> {
    /// Channel is being resolved.
    Channel(&'a Channel),
    /// Emoji is being resolved.
    Emoji(&'a Emoji),
    /// Integration is being resolved.
    Integration(&'a GuildIntegration),
    /// Member is being resolved.
    Member(&'a Member),
    /// Role is being resolved.
    Role(&'a Role),
    /// User is being resolved.
    User(&'a User),
    /// Voice state is being resolved.
    VoiceState(&'a VoiceState),
}

/// Resolution of a [`Rule`].
#[derive(Debug)]
#[non_exhaustive]
pub enum RuleResolution {
    /// Accept the entity.
    Accept,
    /// Reject the entity.
    Reject,
    /// Reject the entity, but only if no other rule has accepted the entity.
    UnsettledReject,
}

impl RuleResolution {
    /// An accepted resolution.
    pub const ACCEPT: Self = Self::Accept;

    /// A denied resolution.
    pub const REJECT: Self = Self::Reject;

    /// An unsettled denial resolution.
    ///
    /// Refer to [`RuleResolution::UnsettledReject`] for more documentation.
    pub const UNSETTLED_REJECT: Self = Self::UnsettledReject;

    /// Create a resolution from a boolean.
    ///
    /// True maps to [`Self::ACCEPT`] while false maps to [`Self::REJECT`]. This
    /// does not map to [`Self::UNSETTLED_REJECT`].
    ///
    /// # Examples
    ///
    /// Create rules from booleans:
    ///
    /// ```
    /// use twilight_cache_inmemory::rule::RuleResolution;
    ///
    /// assert_eq!(RuleResolution::ACCEPT, RuleResolution::from_bool(true));
    /// assert_eq!(RuleResolution::REJECT, RuleResolution::from_bool(true));
    /// ```
    pub const fn from_bool(accept: bool) -> Self {
        if accept {
            Self::ACCEPT
        } else {
            Self::REJECT
        }
    }
}

/// Rules for accepting or rejecting entities into the cache.
///
/// The concept of a rule is simple at its heart: given an object, do I want it
/// to be cached? Rules are predicates that determine this. The cache is
/// provided to help aid in computing this.
///
/// Rules can be added to the cache via [`InMemoryCacheBuilder::rules_mut`].
///
/// # Examples
///
/// Define a rule to accept only member objects into the cache that are
/// associated with the current user, rejecting all other member objects:
///
/// ```
/// use twilight_cache_inmemory::{config::{Entity, Rule}, InMemoryCache};
///
/// #[derive(Clone, Debug, Eq, PartialEq)]
/// pub struct CurrentMemberRule;
///
/// impl Rule for CurrentMemberRule {
///     fn accept(&self, cache: &InMemoryCache, entity: &Entity) -> bool {
///         // We only care about members; if this rule is given something that
///         // isn't a member then we don't have an opinion about it and allow
///         // it into the cache.
///         let member = if let Entity::Member(member) = entity {
///             member
///         } else {
///             return RuleResolution::Accept;
///         };
///
///         // If the current user isn't cached (it should be!) then we don't
///         // want to accept any members because they may not be the current
///         // user.
///         if let Some(current_user) = cache.current_user() {
///             RuleResolution::from(member.user.id == current_user.id)
///         } else {
///             RuleResolution::REJECT
///         }
///     }
/// }
/// ```
///
/// [`InMemoryCacheBuilder::rules_mut`]: super::builder::InMemoryCacheBuilder::rules
pub trait Rule: Debug + Send + Sync + 'static {
    /// Whether to accept a given entity into the cache.
    fn resolve(&self, cache: &InMemoryCache, entity: Entity<'_>) -> RuleResolution;
}

pub(crate) fn resolve_entity<'a, T: Rule + ?Sized>(
    cache: &'a InMemoryCache,
    entity: Entity<'a>,
    rules: &'a [Box<T>],
) -> bool {
    let mut soft_decline = false;

    for rule in rules {
        match rule.resolve(cache, entity) {
            RuleResolution::Accept => {}
            RuleResolution::Reject => return false,
            RuleResolution::UnsettledReject => {
                soft_decline = true;
            }
        }
    }

    !soft_decline
}

pub(crate) fn resolve_entities<'a, T: Rule + ?Sized>(
    cache: &'a InMemoryCache,
    entities: impl Iterator<Item = Entity<'a>> + 'a,
    rules: &'a [Box<T>],
) -> impl Iterator<Item = Entity<'a>> + 'a {
    entities.filter(move |entity| {
        let mut soft_decline = false;

        for rule in rules {
            match rule.resolve(cache, *entity) {
                RuleResolution::Accept => {}
                RuleResolution::Reject => return false,
                RuleResolution::UnsettledReject => {
                    soft_decline = true;
                }
            }
        }

        !soft_decline
    })
}
