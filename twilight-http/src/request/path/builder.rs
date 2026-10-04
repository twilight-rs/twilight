//! Builder for paths using typestates to ensure structure of paths acceptable
//! for use with Discord's REST API.

use super::{Path, QueryParameter};
use std::{fmt::Write as _, marker::PhantomData};
use twilight_model::id::Id;

/// Marker indicating the most recent part is an action.
#[derive(Debug)]
#[non_exhaustive]
pub struct ActionMarker;

/// Marker indicating the most recent part is an entity ID.
#[derive(Debug)]
#[non_exhaustive]
pub struct IdMarker;

/// Marker indicating the most recent part is a query parameter.
#[derive(Debug)]
#[non_exhaustive]
pub struct QueryMarker;

/// Marker indicating the most recent part is a resource name.
#[derive(Debug)]
#[non_exhaustive]
pub struct ResourceMarker;

/// Marker indicating the most recent part is a subresource name.
#[derive(Debug)]
#[non_exhaustive]
pub struct SubresourceMarker;

#[derive(Debug)]
pub struct PathBuilder<T> {
    buffer: String,
    has_query_parameter: bool,
    phantom: PhantomData<T>,
}

impl PathBuilder<()> {
    /// Create a builder for a path.
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            has_query_parameter: false,
            phantom: PhantomData,
        }
    }

    pub fn resource(mut self, resource: &'static str) -> PathBuilder<ResourceMarker> {
        self.buffer.push_str(resource);

        self.cast()
    }
}

impl<T> PathBuilder<T> {
    fn cast<To>(self) -> PathBuilder<To> {
        PathBuilder {
            buffer: self.buffer,
            has_query_parameter: self.has_query_parameter,
            phantom: PhantomData,
        }
    }
}

impl PathBuilder<ActionMarker> {
    pub fn build(self) -> Path {
        Path { inner: self.buffer }
    }

    pub fn csv_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: impl IntoIterator<Item = T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('?');
        self.buffer.push_str(key);
        self.buffer.push('=');

        for (index, id) in value.into_iter().enumerate() {
            if index > 0 {
                self.buffer.push(',');
            }

            write!(self.buffer, "{id}").expect("formatting IDs never fails");
        }

        self.has_query_parameter = true;

        self.cast()
    }

    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);

        if let Some(value) = value.as_ref() {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        self.has_query_parameter = self.has_query_parameter || value.is_some();

        self.cast()
    }

    pub fn parameter(mut self, key: &str, value: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        self.has_query_parameter = true;

        self.cast()
    }
}

impl PathBuilder<IdMarker> {
    pub fn build(self) -> Path {
        Path { inner: self.buffer }
    }

    /// Action on a resource by ID.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        self.buffer.push_str(action);

        self.cast()
    }

    pub fn csv_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: impl IntoIterator<Item = T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('?');
        self.buffer.push_str(key);
        self.buffer.push('=');

        for (index, id) in value.into_iter().enumerate() {
            if index > 0 {
                self.buffer.push(',');
            }

            write!(self.buffer, "{id}").expect("formatting IDs never fails");
        }

        self.has_query_parameter = true;

        self.cast()
    }

    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);

        if let Some(value) = value.as_ref() {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        self.has_query_parameter |= value.is_some();

        self.cast()
    }

    pub fn parameter(mut self, key: &str, value: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        self.has_query_parameter = true;

        self.cast()
    }

    pub fn resource(mut self, resource: &'static str) -> PathBuilder<ResourceMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        self.buffer.push_str(resource);

        self.cast()
    }
}

impl PathBuilder<QueryMarker> {
    pub fn build(self) -> Path {
        Path { inner: self.buffer }
    }

    pub fn csv_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: impl IntoIterator<Item = T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('?');
        self.buffer.push_str(key);
        self.buffer.push('=');

        for (index, id) in value.into_iter().enumerate() {
            if index > 0 {
                self.buffer.push(',');
            }

            write!(self.buffer, "{id}").expect("formatting IDs never fails");
        }

        self.has_query_parameter = true;

        self.cast()
    }

    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        if let Some(value) = value.as_ref() {
            write!(self.buffer, "&{key}={value}").expect("formatting parameters never fails");
        }

        self.has_query_parameter |= value.is_some();

        self.cast()
    }

    pub fn parameter(mut self, key: &str, value: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(true, self.has_query_parameter);
        write!(self.buffer, "&{key}={value}").expect("formatting parameters never fails");
        self.has_query_parameter = true;

        self.cast()
    }
}

impl PathBuilder<ResourceMarker> {
    pub fn build(self) -> Path {
        Path { inner: self.buffer }
    }

    /// Action within a resource.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        self.buffer.push_str(action);

        self.cast()
    }

    pub fn csv_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: impl IntoIterator<Item = T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('?');
        self.buffer.push_str(key);
        self.buffer.push('=');

        for (index, id) in value.into_iter().enumerate() {
            if index > 0 {
                self.buffer.push(',');
            }

            write!(self.buffer, "{id}").expect("formatting IDs never fails");
        }

        self.has_query_parameter = true;

        self.cast()
    }

    pub fn id<T>(mut self, id: Id<T>) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        write!(self.buffer, "/{id}").expect("formatting IDs never fails");

        self.cast()
    }

    pub fn integer_id(mut self, id: u64) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        write!(self.buffer, "{id}").expect("formatting integers can't fail");

        self.cast()
    }

    pub fn me(mut self) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push_str("/@me");
        self.has_query_parameter = true;

        self.cast()
    }

    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);

        if let Some(value) = value.as_ref() {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        self.has_query_parameter |= value.is_some();

        self.cast()
    }

    pub fn parameter(mut self, key: &str, value: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        self.has_query_parameter = true;

        self.cast()
    }

    pub fn subresource(mut self, subresource: &'static str) -> PathBuilder<SubresourceMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        self.buffer.push_str(subresource);

        self.cast()
    }

    pub fn string_id(mut self, id: &str) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        self.buffer.push_str(id);

        self.cast()
    }
}

impl PathBuilder<SubresourceMarker> {
    pub fn build(self) -> Path {
        Path { inner: self.buffer }
    }

    pub fn id<T>(mut self, id: Id<T>) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        write!(self.buffer, "/{id}").expect("formatting IDs never fails");

        self.cast()
    }

    pub fn integer_id(mut self, id: u64) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        write!(self.buffer, "{id}").expect("formatting integers can't fail");

        self.cast()
    }

    pub fn me(mut self) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push_str("/@me");

        self.cast()
    }

    pub fn string_id(mut self, id: &str) -> PathBuilder<IdMarker> {
        debug_assert_eq!(false, self.has_query_parameter);
        self.buffer.push('/');
        self.buffer.push_str(id);

        self.cast()
    }
}

#[cfg(test)]
mod tests {
    use super::PathBuilder;
    use twilight_model::id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, ScheduledEventMarker, UserMarker},
    };

    /// Test a path operating on a resource.
    #[test]
    fn resource() {
        assert_eq!("gateway", PathBuilder::new().resource("gateway").build());
    }

    /// Test a path operating on a resource by ID.
    #[test]
    fn resource_id() {
        assert_eq!(
            "channels/1",
            PathBuilder::new()
                .resource("channels")
                .id(Id::<ChannelMarker>::new(1))
                .build()
        );
    }

    /// Test a path operating on a resource with the current user as the entity.
    #[test]
    fn resource_me() {
        assert_eq!(
            "oauth2/@me",
            PathBuilder::new().resource("oauth2").me().build()
        );
    }

    /// Test a path operating on a resource within a resource by ID.
    #[test]
    fn resource_id_resource() {
        assert_eq!(
            "channels/1/messages",
            PathBuilder::new()
                .resource("channels")
                .id(Id::<ChannelMarker>::new(1))
                .resource("messages")
                .build()
        );
    }

    /// Test a path operating on a resource by ID within another resource by ID.
    #[test]
    fn resource_id_resource_id() {
        assert_eq!(
            "channels/1/messages/2",
            PathBuilder::new()
                .resource("channels")
                .id(Id::<ChannelMarker>::new(1))
                .resource("messages")
                .id(Id::<MessageMarker>::new(2))
                .build()
        );
    }

    /// Test building a guild bans path with a query parameter set to an
    /// integer.
    #[test]
    fn query_parameter_integer_value() {
        assert_eq!(
            "guilds/1/bans?limit=100",
            PathBuilder::new()
                .resource("guilds")
                .id(Id::<GuildMarker>::new(1))
                .resource("bans")
                .parameter("limit", 100u64)
                .build()
        );
    }

    /// Test building a guild ID path with a query parameter set to true.
    #[test]
    fn query_parameter_true_value() {
        assert_eq!(
            "guilds/1?with_counts=true",
            PathBuilder::new()
                .resource("guilds")
                .id(Id::<GuildMarker>::new(1))
                .parameter("with_counts", true)
                .build()
        );
    }

    /// Test building a guild ID path with a query parameter set to false.
    #[test]
    fn query_parameter_false_value() {
        assert_eq!(
            "guilds/1?with_counts=false",
            PathBuilder::new()
                .resource("guilds")
                .id(Id::<GuildMarker>::new(1))
                .parameter("with_counts", false)
                .build()
        );
    }

    /// Test building a guild member search path with a string query parameter.
    #[test]
    fn query_parameter_string_value() {
        assert_eq!(
            "guilds/1/members/search?query=abc",
            PathBuilder::new()
                .resource("guilds")
                .id(Id::<GuildMarker>::new(1))
                .resource("members")
                .action("search")
                .parameter("query", "abc")
                .build()
        );
    }

    /// Test appending an optional query parameter to a path with an existing
    /// query parameter.
    #[test]
    fn optional_query_parameter_after_query_parameter() {
        let path = PathBuilder::new()
            .resource("guilds")
            .id(Id::<GuildMarker>::new(1))
            .resource("scheduled-events")
            .id(Id::<ScheduledEventMarker>::new(2))
            .resource("users")
            .parameter("after", Id::<UserMarker>::new(3))
            .optional_parameter("limit", Some(10u64))
            .build();

        assert_eq!("guilds/1/scheduled-events/2/users?after=3&limit=10", path);
    }

    /// Test appending a Some optional parameter after a None optional
    /// parameter.
    ///
    /// This ensures that the second query parameter isn't appended to a path as
    /// `foo/1/bar&baz=qux`.
    #[test]
    fn multiple_optional_query_parameter() {
        let path = PathBuilder::new()
            .resource("foo")
            .optional_parameter("after", None::<Id<UserMarker>>)
            .optional_parameter("limit", Some(10u64))
            .build();

        assert_eq!("foo?limit=10", path);
    }
}
