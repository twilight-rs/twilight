use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZero,
};

pub struct ShardIdParseError {
    kind: ShardIdParseErrorType,
}

impl ShardIdParseError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ShardIdParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ShardIdParseErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for ShardIdParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ShardIdParseErrorType::NumberGreaterOrEqualTotal { number, total } => {
                f.write_str("ShardId's number (")?;
                Display::fmt(&number, f)?;
                f.write_str(") was greater or equal to its total (")?;
                Display::fmt(&total, f)?;

                f.write_str(")")
            }
        }
    }
}

/// Type of [`ShardIdParseError`] that occurred.
#[derive(Debug)]
pub enum ShardIdParseErrorType {
    /// `ShardId`'s number was greater or equal to its total.
    NumberGreaterOrEqualTotal {
        /// Value of number.
        number: u32,
        /// Value of total.
        total: u32,
    },
}

/// Shard identifier to calculate if it receivies a given event.
///
/// A shard ID consist of two fields: `number` and `total`. These values do not
/// need to be unique, and are used by Discord for calculating which events to
/// send to which shard. Shards should in general share the same `total` value
/// and have an unique `number` value, but users may deviate from this when
/// resharding/migrating to a new set of shards.
///
/// # Advanced use
///
/// Incoming events are split by their originating guild and are received by the
/// shard with the id calculated from the following formula:
///
/// > `number = (guild_id >> 22) % total`.
///
/// `total` is in other words unrelated to the total number of shards and is
/// only used to specify the share of events a shard will receive. The formula
/// is independently calculated for all shards, which means that events may be
/// duplicated or lost if it's determined that an event should be sent to
/// multiple or no shard.
///
/// It may be helpful to visualize the logic in code:
///
/// ```ignore
/// for shard in shards {
///     if shard.id().number() == (guild_id >> 22) % shard.id().total() {
///         unimplemented!("send event to shard");
///     }
/// }
/// ```
///
/// See [Discord Docs/Sharding].
///
/// [Discord Docs/Sharding]: https://discord.com/developers/docs/topics/gateway#sharding
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(try_from = "[u32; 2]", into = "[u32; 2]")]
pub struct ShardId {
    /// Number of the shard, 0-indexed.
    number: u32,
    /// Total number of shards used by the bot, 1-indexed.
    total: NonZero<u32>,
}

impl ShardId {
    /// ID of a bot that has only one shard.
    ///
    /// Should *only* be used by small bots in under one or two thousand guilds.
    pub const ONE: ShardId = ShardId::new(0, 1);

    /// Create a new shard identifier.
    ///
    /// The shard number is 0-indexed while the total number of shards is
    /// 1-indexed. A shard number of 7 with a total of 8 is therefore valid,
    /// whilst a shard number of 8 out of 8 total shards is invalid.
    ///
    /// # Examples
    ///
    /// Create a new shard with a shard number of 13 out of a total of 24
    /// shards:
    ///
    /// ```
    /// use twilight_model::gateway::ShardId;
    ///
    /// let id = ShardId::new(13, 24);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the shard number is greater than or equal to the total number
    /// of shards.
    pub const fn new(number: u32, total: u32) -> Self {
        assert!(number < total, "number must be less than total");
        if let Some(total) = NonZero::new(total) {
            Self { number, total }
        } else {
            panic!("unreachable: total is at least 1")
        }
    }

    /// Create a new shard identifier if the shard indexes are valid.
    #[allow(clippy::missing_panics_doc)]
    pub const fn new_checked(number: u32, total: u32) -> Option<Self> {
        if number >= total {
            return None;
        }

        if let Some(total) = NonZero::new(total) {
            Some(Self { number, total })
        } else {
            panic!("unreachable: total is at least 1")
        }
    }

    /// Identifying number of the shard, 0-indexed.
    pub const fn number(self) -> u32 {
        self.number
    }

    /// Total number of shards, 1-indexed.
    pub const fn total(self) -> u32 {
        self.total.get()
    }
}

/// Display the shard ID.
///
/// Formats as `[{number}, {total}]`.
impl Display for ShardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list()
            .entries(Into::<[u32; 2]>::into(*self))
            .finish()
    }
}

impl TryFrom<[u32; 2]> for ShardId {
    type Error = ShardIdParseError;

    fn try_from([number, total]: [u32; 2]) -> Result<Self, Self::Error> {
        Self::new_checked(number, total).ok_or(ShardIdParseError {
            kind: ShardIdParseErrorType::NumberGreaterOrEqualTotal { number, total },
        })
    }
}

impl From<ShardId> for [u32; 2] {
    fn from(id: ShardId) -> Self {
        [id.number(), id.total()]
    }
}

#[cfg(test)]
mod tests {
    use super::ShardId;
    use serde::{Serialize, de::DeserializeOwned};
    use serde_test::Token;
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    const_assert_eq!(ShardId::ONE.number(), 0);
    const_assert_eq!(ShardId::ONE.total(), 1);
    assert_impl_all!(
        ShardId: Clone,
        Copy,
        Debug,
        DeserializeOwned,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    const fn checked_invalid() {
        assert!(ShardId::new_checked(0, 1).is_some());
        assert!(ShardId::new_checked(1, 1).is_none());
        assert!(ShardId::new_checked(2, 1).is_none());
        assert!(ShardId::new_checked(0, 0).is_none());
    }

    #[test]
    const fn getters() {
        let id = ShardId::new(2, 4);

        assert!(id.number() == 2);
        assert!(id.total() == 4);
    }

    #[test]
    fn serde() {
        let value = ShardId::new(0, 1);

        serde_test::assert_tokens(
            &value,
            &[
                Token::Tuple { len: 2 },
                Token::U32(0),
                Token::U32(1),
                Token::TupleEnd,
            ],
        )
    }

    #[should_panic(expected = "number must be less than total")]
    #[test]
    const fn number_equal() {
        ShardId::new(1, 1);
    }

    #[should_panic(expected = "number must be less than total")]
    #[test]
    const fn number_greater() {
        ShardId::new(2, 1);
    }
}
