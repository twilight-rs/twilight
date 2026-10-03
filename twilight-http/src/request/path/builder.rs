use super::{Path, QueryParameter};
use std::{
    fmt::{Display, Write as _},
    marker::PhantomData,
};
use twilight_model::id::Id;

/// Marker indicating the most recent part is an action.
///
/// The only part that may come immediately after an action is a query
/// parameter.
#[derive(Debug)]
#[non_exhaustive]
pub struct ActionMarker;

/// Marker indicating the most recent part is an entity ID.
///
/// The only parts that may come immediately after an ID are a query parameter
/// and resource.
#[derive(Debug)]
#[non_exhaustive]
pub struct IdMarker;

/// Marker indicating the most recent part is a query parameter.
///
/// The only part that may come immediately after a query parameter is another
/// query parameter.
#[derive(Debug)]
#[non_exhaustive]
pub struct QueryMarker;

/// Marker indicating the most recent part is a resource name.
///
/// The only part that may come immediately after a resource is an action, ID,
/// or query parameter.
#[derive(Debug)]
#[non_exhaustive]
pub struct ResourceMarker;

/// Marker indicating the most recent part is a subresource name.
///
/// The only part that may come immediately after a subresource is an ID.
#[derive(Debug)]
#[non_exhaustive]
pub struct SubresourceMarker;

pub trait PathMarker {}

impl PathMarker for ActionMarker {}
impl PathMarker for IdMarker {}
impl PathMarker for QueryMarker {}
impl PathMarker for ResourceMarker {}
impl PathMarker for SubresourceMarker {}

#[derive(Debug)]
pub struct PathBuilder<T> {
    buffer: String,
    phantom: PhantomData<T>,
}

impl PathBuilder<()> {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            phantom: PhantomData,
        }
    }

    pub fn resource(mut self, resource: &str) -> PathBuilder<ResourceMarker> {
        self.buffer.push_str(resource);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }
}

impl PathBuilder<ActionMarker> {
    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        if let Some(value) = value {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn parameter(mut self, key: &str, value: impl Display) -> PathBuilder<QueryMarker> {
        write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }
}

impl PathBuilder<IdMarker> {
    /// Action on a resource by ID.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        self.buffer.push('/');
        self.buffer.push_str(action);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        if let Some(value) = value {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn parameter(mut self, key: &str, value: impl Display) -> PathBuilder<QueryMarker> {
        write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn resource(mut self, resource: &'static str) -> PathBuilder<ResourceMarker> {
        self.buffer.push('/');
        self.buffer.push_str(resource);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }
}

impl PathBuilder<QueryMarker> {
    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        if let Some(value) = value {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn parameter(mut self, key: &str, value: impl Display) -> PathBuilder<QueryMarker> {
        write!(self.buffer, "&{key}={value}").expect("formatting parameters never fails");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }
}

impl PathBuilder<ResourceMarker> {
    /// Action within a resource.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        self.buffer.push('/');
        self.buffer.push_str(action);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn id<T>(mut self, id: Id<T>) -> PathBuilder<IdMarker> {
        write!(self.buffer, "/{id}").expect("formatting IDs never fails");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn integer_id(mut self, id: u64) -> PathBuilder<IdMarker> {
        self.buffer.push('/');
        write!(self.buffer, "{id}").expect("formatting integers can't fail");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn me(mut self) -> PathBuilder<IdMarker> {
        self.buffer.push_str("/@me");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn optional_parameter<T: QueryParameter>(
        mut self,
        key: &str,
        value: Option<T>,
    ) -> PathBuilder<QueryMarker> {
        if let Some(value) = value {
            write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");
        }

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn parameter(mut self, key: &str, value: impl QueryParameter) -> PathBuilder<QueryMarker> {
        write!(self.buffer, "?{key}={value}").expect("formatting parameters never fails");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn subresource(mut self, subresource: &'static str) -> PathBuilder<SubresourceMarker> {
        self.buffer.push('/');
        self.buffer.push_str(subresource);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn string_id(mut self, id: &str) -> PathBuilder<IdMarker> {
        self.buffer.push('/');
        self.buffer.push_str(id);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }
}

impl PathBuilder<SubresourceMarker> {
    pub fn id<T>(mut self, id: Id<T>) -> PathBuilder<IdMarker> {
        write!(self.buffer, "/{id}").expect("formatting IDs never fails");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn integer_id(mut self, id: u64) -> PathBuilder<IdMarker> {
        self.buffer.push('/');
        write!(self.buffer, "{id}").expect("formatting integers can't fail");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn me(mut self) -> PathBuilder<IdMarker> {
        self.buffer.push_str("/@me");

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }

    pub fn string_id(mut self, id: &str) -> PathBuilder<IdMarker> {
        self.buffer.push('/');
        self.buffer.push_str(id);

        PathBuilder {
            buffer: self.buffer,
            phantom: PhantomData,
        }
    }
}

impl<T: PathMarker> PathBuilder<T> {
    pub fn build(self) -> Path {
        Path { inner: self.buffer }
    }
}

#[cfg(test)]
mod tests {
    use super::PathBuilder;
    use twilight_model::id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker},
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
}
