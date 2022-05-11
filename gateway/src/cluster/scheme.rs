//! Schemes for instantiating a cluster of shards.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    iter::StepBy,
    ops::{Bound, RangeBounds, RangeInclusive},
};

/// Starting a cluster failed.
#[derive(Debug)]
pub struct ShardSchemeRangeError {
    kind: ShardSchemeRangeErrorType,
}

impl ShardSchemeRangeError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ShardSchemeRangeErrorType {
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
    pub fn into_parts(
        self,
    ) -> (
        ShardSchemeRangeErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ShardSchemeRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ShardSchemeRangeErrorType::BucketTooLarge {
                bucket_id,
                concurrency,
                ..
            } => {
                f.write_str("bucket ID ")?;
                Display::fmt(bucket_id, f)?;
                f.write_str(" is larger than maximum concurrency (")?;
                Display::fmt(concurrency, f)?;

                f.write_str(")")
            }
            ShardSchemeRangeErrorType::IdTooLarge { end, start, total } => {
                f.write_str("The shard ID range ")?;
                Display::fmt(start, f)?;
                f.write_str("-")?;
                Display::fmt(end, f)?;
                f.write_str("/")?;
                Display::fmt(total, f)?;

                f.write_str(" is larger than the total")
            }
        }
    }
}

/// Starting a cluster failed.
#[derive(Debug)]
#[non_exhaustive]
pub enum ShardSchemeRangeErrorType {
    /// Bucket ID is larger than the maximum concurrency.
    BucketTooLarge {
        /// ID of the bucket.
        bucket_id: u64,
        /// Number of shards in a bucket.
        concurrency: u64,
        /// Total number of buckets.
        total: u64,
    },
    /// Start of the shard range was greater than the end or total.
    IdTooLarge {
        /// Last shard in the range to manage.
        end: u64,
        /// First shard in the range to manage.
        start: u64,
        /// Total number of shards used by the bot.
        total: u64,
    },
}

impl Error for ShardSchemeRangeError {}

/// Iterator of shard IDs based on a shard scheme.
///
/// # Examples
///
/// Iterate over a shard scheme range from 0 to 4 with a total of 19 shards:
///
/// ```
/// # fn main() { try_main().unwrap() }
/// #
/// # fn try_main() -> Option<()> {
/// use twilight_gateway::cluster::ShardScheme;
///
/// let scheme = ShardScheme::Range {
///     from: 0,
///     to: 4,
///     total: 19,
/// };
/// let mut iter = scheme.iter();
/// assert_eq!(0, iter.next()?);
/// assert_eq!(1, iter.next()?);
/// assert_eq!(2, iter.next()?);
/// assert_eq!(3, iter.next()?);
/// assert_eq!(4, iter.next()?);
/// assert!(iter.next().is_none());
/// # Some(()) }
/// ```
#[derive(Clone, Debug)]
pub struct ShardSchemeIter {
    inner: StepBy<RangeInclusive<u64>>,
}

impl ShardSchemeIter {
    /// Create an iterator of shard IDs out of a scheme.
    fn new(scheme: &ShardScheme) -> Self {
        let (from, to, step) = match scheme {
            ShardScheme::Bucket {
                bucket_id,
                concurrency,
                total,
            } => {
                // It's reasonable to assume that no one will ever have a
                // concurrency size greater than even 16 bits.
                let concurrency = usize::try_from(*concurrency)
                    .expect("concurrency is larger than target pointer width");

                (*bucket_id, *total - 1, concurrency)
            }
            ShardScheme::Range { from, to, .. } => (*from, *to, 1),
        };

        Self {
            inner: (from..=to).step_by(step),
        }
    }
}

impl Iterator for ShardSchemeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// The method of sharding to use.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ShardScheme {
    /// Manage a single bucket's worth of shards within the cluster.
    ///
    /// This is primarily useful for bots in the [Sharding for Very Large Bots]
    /// program.
    ///
    /// [Sharding for Very Large Bots]: https://discord.com/developers/docs/topics/gateway#sharding-for-very-large-bots
    Bucket {
        /// ID of the first shard to start.
        ///
        /// This must be less than the maximum concurrency.
        ///
        /// For example, if you have a maximum concurrency of 16 and the bucket
        /// ID is 0, then shards 0, 16, 32, etc. will be managed by the cluster.
        bucket_id: u64,
        /// Number of shards allowed to be started simultaneously within a
        /// bucket, also known as the maximum concurrency.
        ///
        /// This is provided via [`SessionStartLimit::max_concurrency`].
        ///
        /// [`SessionStartLimit::max_concurrency`]: ::twilight_model::gateway::SessionStartLimit::max_concurrency
        concurrency: u64,
        /// Total number of shards used across all clusters.
        total: u64,
    },
    /// Specifies to start a range of shards.
    ///
    /// # Examples
    ///
    /// For example, if your bot uses 50 shards, then you might specify to start
    /// shards 0 through 24:
    ///
    /// ```
    /// use twilight_gateway::cluster::ShardScheme;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let range = ShardScheme::try_from((0..24, 50))?;
    /// # Ok(()) }
    /// ```
    Range {
        /// First shard ID to spawn.
        from: u64,
        /// Last shard ID to spawn.
        ///
        /// This doesn't necessarily have to be up to the `total`.
        to: u64,
        /// Total number of shards used by the bot.
        total: u64,
    },
}

impl ShardScheme {
    /// Returns an iterator over its shard IDs.
    pub fn iter(&self) -> ShardSchemeIter {
        ShardSchemeIter::new(self)
    }

    /// First shard ID that will be started.
    pub const fn from(&self) -> u64 {
        match *self {
            Self::Bucket { bucket_id, .. } => bucket_id,
            Self::Range { from, .. } => from,
        }
    }

    /// Total number of shards used by the bot across all clusters.
    pub const fn total(&self) -> u64 {
        match *self {
            Self::Bucket { total, .. } | Self::Range { total, .. } => total,
        }
    }

    /// Maximum shard ID across all clusters.
    pub const fn to(&self) -> u64 {
        match *self {
            Self::Bucket {
                bucket_id,
                concurrency,
                total,
            } => {
                let buckets = total / concurrency;

                // Total is 1-indexed but shards are 0-indexed, so we need to
                // subtract 1 here.
                total - (buckets - bucket_id) - 1
            }
            Self::Range { to, .. } => to,
        }
    }
}

impl<T: RangeBounds<u64>> TryFrom<(T, u64)> for ShardScheme {
    type Error = ShardSchemeRangeError;

    fn try_from((range, total): (T, u64)) -> Result<Self, Self::Error> {
        let start = match range.start_bound() {
            Bound::Excluded(num) => *num - 1,
            Bound::Included(num) => *num,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Excluded(num) => *num - 1,
            Bound::Included(num) => *num,
            Bound::Unbounded => total - 1,
        };

        if start > end {
            return Err(ShardSchemeRangeError {
                kind: ShardSchemeRangeErrorType::IdTooLarge { end, start, total },
            });
        }

        Ok(Self::Range {
            from: start,
            to: end,
            total,
        })
    }
}

/// Create a [`ShardScheme::Bucket`] shard scheme.
///
/// # Examples
///
/// Create a scheme for bucket 7 and with a maximum concurrency of 16 and a
/// total of 320 shards:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_gateway::cluster::ShardScheme;
///
/// let scheme = ShardScheme::try_from((7u64, 16, 320))?;
/// assert_eq!(7, scheme.from());
/// assert_eq!(306, scheme.to());
/// assert_eq!(320, scheme.total());
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns [`ShardSchemeRangeErrorType::BucketTooLarge`] if the provided bucket ID
/// is larger than the total number of buckets (`total / concurrency`).
impl TryFrom<(u64, u64, u64)> for ShardScheme {
    type Error = ShardSchemeRangeError;

    fn try_from((bucket_id, concurrency, total): (u64, u64, u64)) -> Result<Self, Self::Error> {
        let buckets = total / concurrency;

        if bucket_id >= buckets {
            return Err(ShardSchemeRangeError {
                kind: ShardSchemeRangeErrorType::BucketTooLarge {
                    bucket_id,
                    concurrency,
                    total,
                },
            });
        }

        Ok(ShardScheme::Bucket {
            bucket_id,
            concurrency,
            total,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ShardScheme, ShardSchemeIter, ShardSchemeRangeError, ShardSchemeRangeErrorType};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug, hash::Hash};

    assert_impl_all!(ShardSchemeIter: Clone, Debug, Send, Sync);
    assert_fields!(ShardSchemeRangeErrorType::IdTooLarge: end, start, total);
    assert_impl_all!(ShardSchemeRangeError: Error, Send, Sync);
    assert_fields!(ShardScheme::Range: from, to, total);
    assert_impl_all!(
        ShardScheme: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync,
        TryFrom<(u64, u64, u64)>,
    );

    #[test]
    fn test_scheme() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            ShardScheme::Range {
                from: 0,
                to: 9,
                total: 10,
            },
            ShardScheme::try_from((0..=9, 10))?
        );

        Ok(())
    }

    #[test]
    fn test_scheme_from() {
        assert_eq!(
            18,
            ShardScheme::Bucket {
                bucket_id: 18,
                concurrency: 16,
                total: 320,
            }
            .from()
        );
        assert_eq!(
            50,
            ShardScheme::Range {
                from: 50,
                to: 99,
                total: 200,
            }
            .from()
        );
    }

    #[test]
    fn test_scheme_total() {
        assert_eq!(
            160,
            ShardScheme::Bucket {
                bucket_id: 3,
                concurrency: 16,
                total: 160,
            }
            .total()
        );
        assert_eq!(
            17,
            ShardScheme::Range {
                from: 0,
                to: 9,
                total: 17,
            }
            .total()
        );
    }

    #[test]
    fn test_scheme_to() {
        assert_eq!(
            317,
            ShardScheme::Bucket {
                bucket_id: 18,
                concurrency: 16,
                total: 320,
            }
            .to()
        );
        assert_eq!(
            299,
            ShardScheme::Bucket {
                bucket_id: 0,
                concurrency: 16,
                total: 320,
            }
            .to()
        );
        assert_eq!(
            99,
            ShardScheme::Range {
                from: 50,
                to: 99,
                total: 200,
            }
            .to()
        );
    }

    /// Test that a [`BucketTooLarge`] error will return if the ID of the bucket
    /// is greater than the specified concurrency.
    ///
    /// [`BucketTooLarge`]: super::ShardSchemeRangeError::BucketTooLarge
    #[test]
    fn test_scheme_bucket_larger_than_concurrency() {
        assert!(matches!(
            ShardScheme::try_from((25, 16, 320)).unwrap_err(),
            ShardSchemeRangeError {
                kind: ShardSchemeRangeErrorType::BucketTooLarge { bucket_id, concurrency, total }}
            if bucket_id == 25 && concurrency == 16 && total == 320
        ));
    }
}
