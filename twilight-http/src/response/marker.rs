//! Markers denoting the type of response body.
//!
//! Markers are used depending on the type of response that Twilight expects
//! from an endpoint. For example, [`DeleteRole`] responses have no body, so an
//! [`EmptyBody`] marker is used in the [`Response`].
//!
//! [`DeleteRole`]: super::super::request::guild::role::DeleteRole
//! [`Response`]: super::Response

use std::marker::PhantomData;

/// Marker that a response has no body. Responses with this marker can't be
/// deserialized.
///
/// Requests like [`AddRoleToMember`] or [`DeleteRole`] use this.
///
/// [`AddRoleToMember`]: crate::request::guild::member::AddRoleToMember
/// [`DeleteRole`]: crate::request::guild::role::DeleteRole
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct EmptyBody;

/// Marker that a response has a list of something.
///
/// May be used via the [`Response::models`].
///
/// [`Response::models`]: super::Response::<ListBody<T>>::models
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListBody<T> {
    phantom: PhantomData<T>,
}

#[cfg(test)]
mod tests {
    use super::{EmptyBody, ListBody};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(EmptyBody: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(ListBody<String>: Clone, Debug, Eq, PartialEq, Send, Sync);
}
