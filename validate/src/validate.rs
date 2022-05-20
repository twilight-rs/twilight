#![allow(unused)]

use std::marker::PhantomData;

/// Validated value for a specific marker.
pub struct Validated<T, M> {
    inner: T,
    marker: PhantomData<M>,
}

impl<T, M> Validated<T, M> {
    pub(crate) fn new(inner: T) -> Self {
        Validated {
            inner,
            marker: PhantomData,
        }
    }

    /// Get the inner value that is validated
    pub fn get(self) -> T {
        self.inner
    }
}

/// Trait that specifies how a specific marker is validated.
pub trait Validate<Marker>
where
    Self: Sized,
{
    /// The error type validation returns.
    type Error;

    /// Verify the data.
    fn validate(self) -> Result<Validated<Self, Marker>, Self::Error>;

    /// Skip verification of data.
    fn validate_unchecked(self) -> Validated<Self, Marker> {
        Validated {
            inner: self,
            marker: PhantomData,
        }
    }
}
