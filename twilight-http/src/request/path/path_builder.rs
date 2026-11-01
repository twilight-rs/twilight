//! Builder with a typestate for constructing valid [`Path`s].

use super::{
    Path,
    query_parameter::{QueryParameter, QueryParameterWriter},
};
use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter, Write as _},
    marker::PhantomData,
};
use twilight_model::id::Id;

/// Error occurred while building a [Path].
#[derive(Debug)]
pub struct PathBuilderError {
    /// Type of error.
    pub(super) kind: PathBuilderErrorType,
    /// Source error if available.
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
}

impl PathBuilderError {
    /// Construct a new error from a source formatting error.
    fn formatting(source: FmtError) -> Self {
        Self {
            kind: PathBuilderErrorType::Formatting,
            source: Some(Box::new(source)),
        }
    }

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &PathBuilderErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (PathBuilderErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for PathBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self.kind {
            PathBuilderErrorType::Formatting => {
                f.write_str("unable to format a segment of the path")
            }
        }
    }
}

impl Error for PathBuilderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|source| &**source as _)
    }
}

/// Type of [`Error`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum PathBuilderErrorType {
    /// Path is unable to be formatted.
    Formatting,
}

/// Sentinel for referring to the current application in place of an ID.
///
/// This writes "@me" to ID segments in the path.
pub struct Me;

impl Display for Me {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("@me")
    }
}

/// Format a representation of an identifier into the ID portion of a path.
///
/// The most notable type this is implemented for is [Id], which is the most
/// common representation of IDs. Some IDs, such as invite codes, are strings,
/// which this is also implemented for. Some IDs are integers, such as poll
/// answer IDs.
pub trait PathId: Display {}

impl<T> PathId for Id<T> {}
impl<T: PathId> PathId for &T {}
impl PathId for Me {}

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

/// Builder to construct a valid [Path].
#[derive(Debug)]
pub struct PathBuilder<T> {
    /// Internal string buffer segments are written into.
    buffer: Result<String, FmtError>,
    /// Whether a query parameter has been written into the buffer.
    ///
    /// This is used to determine whether to write the beginning symbol (`?`) or
    /// joiner symbol (`&`) to a URI.
    has_query_parameter: bool,
    /// Constrain the typestate marker.
    phantom: PhantomData<T>,
}

impl PathBuilder<()> {
    /// Create a builder to construct a valid path.
    pub const fn new() -> Self {
        Self {
            buffer: Ok(String::new()),
            has_query_parameter: false,
            phantom: PhantomData,
        }
    }

    /// Specify the root resource of a path.
    ///
    /// Common root resources are "channels" and "guilds", as in the paths
    /// "channels/123/messages" and "guilds/123".
    pub fn resource(mut self, resource: &'static str) -> PathBuilder<ResourceMarker> {
        if let Ok(buffer) = self.buffer.as_mut() {
            buffer.push_str(resource);
        }

        self.cast()
    }
}

impl<T> PathBuilder<T> {
    /// Reconstruct the builder with a new typestate.
    fn cast<To>(self) -> PathBuilder<To> {
        PathBuilder {
            buffer: self.buffer,
            has_query_parameter: self.has_query_parameter,
            phantom: PhantomData,
        }
    }
}

impl Default for PathBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl PathBuilder<ActionMarker> {
    /// Finalize the builder, returning the formatted path.
    ///
    /// # Errors
    ///
    /// Returns a [`PathBuilderError`] with a type of
    /// [`PathBuilderErrorType::Formatting`] if a value provided to a method or
    /// query parameter was unable to be formatted. This should only happen with
    /// user-defined types.
    pub fn build(self) -> Result<Path, PathBuilderError> {
        self.buffer
            .map(Path::new)
            .map_err(PathBuilderError::formatting)
    }

    /// Append a query parameter to the path.
    ///
    /// Once a query parameter has been appended then only more query parameters
    /// can be appended or the builder can be finalized.
    ///
    /// Refer to [`query_parameter`] for more information on how to use query
    /// parameters.
    ///
    /// [`query_parameter`]: super::query_parameter
    pub fn parameter(self, query_parameter: impl QueryParameter) -> PathBuilder<QueryMarker> {
        PathBuilder::<QueryMarker>::parameter(self.cast(), query_parameter)
    }
}

impl PathBuilder<IdMarker> {
    /// Finalize the builder, returning the formatted path.
    ///
    /// # Errors
    ///
    /// Returns a [`PathBuilderError`] with a type of
    /// [`PathBuilderErrorType::Formatting`] if a value provided to a method or
    /// query parameter was unable to be formatted. This should only happen with
    /// user-defined types.
    pub fn build(self) -> Result<Path, PathBuilderError> {
        self.buffer
            .map(Path::new)
            .map_err(PathBuilderError::formatting)
    }

    /// Action on a resource by ID.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        debug_assert!(!self.has_query_parameter);

        if let Ok(buffer) = self.buffer.as_mut() {
            buffer.push('/');
            buffer.push_str(action);
        }

        self.cast()
    }

    /// Explicitly state that the next segment has no resource.
    ///
    /// This is mostly only useful for webhook routes, which contain the ID
    /// followed by the authentication token in the path.
    pub fn no_resource(self) -> PathBuilder<ResourceMarker> {
        debug_assert!(!self.has_query_parameter);

        self.cast()
    }

    /// Append a query parameter to the path.
    ///
    /// Once a query parameter has been appended then only more query parameters
    /// can be appended or the builder can be finalized.
    ///
    /// Refer to [`query_parameter`] for more information on how to use query
    /// parameters.
    ///
    /// [`query_parameter`]: super::query_parameter
    pub fn parameter(self, query_parameter: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert!(!self.has_query_parameter);

        PathBuilder::<QueryMarker>::parameter(self.cast(), query_parameter)
    }

    /// Append a resource to the path.
    pub fn resource(mut self, resource: &'static str) -> PathBuilder<ResourceMarker> {
        debug_assert!(!self.has_query_parameter);

        if let Ok(buffer) = self.buffer.as_mut() {
            buffer.push('/');
            buffer.push_str(resource);
        }

        self.cast()
    }
}

impl PathBuilder<QueryMarker> {
    /// Finalize the builder, returning the formatted path.
    ///
    /// # Errors
    ///
    /// Returns a [`PathBuilderError`] with a type of
    /// [`PathBuilderErrorType::Formatting`] if a value provided to a method or
    /// query parameter was unable to be formatted. This should only happen with
    /// user-defined types.
    pub fn build(self) -> Result<Path, PathBuilderError> {
        self.buffer
            .map(Path::new)
            .map_err(PathBuilderError::formatting)
    }

    /// Append a query parameter to the path.
    ///
    /// Once a query parameter has been appended then only more query parameters
    /// can be appended or the builder can be finalized.
    ///
    /// Refer to [`query_parameter`] for more information on how to use query
    /// parameters.
    ///
    /// [`query_parameter`]: super::query_parameter
    #[must_use = "builder must be finalized to retrieve the path"]
    pub fn parameter<T: QueryParameter>(mut self, query_parameter: T) -> PathBuilder<QueryMarker> {
        struct QueryParameterDisplay<T> {
            has_query_parameter: bool,
            query_parameter: T,
        }

        impl<T: QueryParameter> Display for QueryParameterDisplay<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                T::write(
                    &self.query_parameter,
                    QueryParameterWriter::new(f, self.has_query_parameter),
                )
            }
        }

        if query_parameter.will_write() {
            self.buffer = self.buffer.and_then(|mut buffer| {
                let buffer_length = buffer.len();

                write!(
                    buffer,
                    "{}",
                    QueryParameterDisplay {
                        has_query_parameter: self.has_query_parameter,
                        query_parameter,
                    }
                )?;
                self.has_query_parameter |= buffer.len() > buffer_length;

                Ok(buffer)
            });
        }

        self.cast()
    }
}

impl PathBuilder<ResourceMarker> {
    /// Finalize the builder, returning the formatted path.
    ///
    /// # Errors
    ///
    /// Returns a [`PathBuilderError`] with a type of
    /// [`PathBuilderErrorType::Formatting`] if a value provided to a method or
    /// query parameter was unable to be formatted. This should only happen with
    /// user-defined types.
    pub fn build(self) -> Result<Path, PathBuilderError> {
        self.buffer
            .map(Path::new)
            .map_err(PathBuilderError::formatting)
    }

    /// Action within a resource.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        debug_assert!(!self.has_query_parameter);

        if let Ok(buffer) = self.buffer.as_mut() {
            buffer.push('/');
            buffer.push_str(action);
        }

        self.cast()
    }

    /// Append an entity ID to the path.
    ///
    /// This is usually an [`Id`].
    pub fn id(mut self, id: impl PathId) -> PathBuilder<IdMarker> {
        debug_assert!(!self.has_query_parameter);

        self.buffer = self.buffer.and_then(|mut buffer| {
            write!(buffer, "/{id}")?;

            Ok(buffer)
        });

        self.cast()
    }

    /// Append a query parameter to the path.
    ///
    /// Once a query parameter has been appended then only more query parameters
    /// can be appended or the builder can be finalized.
    ///
    /// Refer to [`query_parameter`] for more information on how to use query
    /// parameters.
    ///
    /// [`query_parameter`]: super::query_parameter
    pub fn parameter(self, query_parameter: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert!(!self.has_query_parameter);

        PathBuilder::<QueryMarker>::parameter(self.cast(), query_parameter)
    }

    /// Append a subresource of the current resource to the path.
    ///
    /// # Examples
    ///
    /// Build a route for fetching the pins of a channel, where the pins are a
    /// subresource of the channel's messages:
    ///
    /// ```rust
    /// use twilight_http::request::path::Path;
    /// use twilight_model::id::{Id, marker::ChannelMarker};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let path = Path::builder()
    ///     .resource("channels")
    ///     .id(Id::<ChannelMarker>::new(123))
    ///     .resource("messages")
    ///     .subresource("pins")
    ///     .build()?;
    ///
    /// assert_eq!("channels/123/messages/pins", path);
    /// # Ok(()) }
    /// ```
    pub fn subresource(mut self, subresource: &'static str) -> PathBuilder<SubresourceMarker> {
        debug_assert!(!self.has_query_parameter);

        if let Ok(buffer) = self.buffer.as_mut() {
            buffer.push('/');
            buffer.push_str(subresource);
        }

        self.cast()
    }
}

impl PathBuilder<SubresourceMarker> {
    /// Finalize the builder, returning the formatted path.
    ///
    /// # Errors
    ///
    /// Returns a [`PathBuilderError`] with a type of
    /// [`PathBuilderErrorType::Formatting`] if a value provided to a method or
    /// query parameter was unable to be formatted. This should only happen with
    /// user-defined types.
    pub fn build(self) -> Result<Path, PathBuilderError> {
        self.buffer
            .map(Path::new)
            .map_err(PathBuilderError::formatting)
    }

    /// Action within a subresource.
    pub fn action(mut self, action: &'static str) -> PathBuilder<ActionMarker> {
        debug_assert!(!self.has_query_parameter);

        if let Ok(buffer) = self.buffer.as_mut() {
            buffer.push('/');
            buffer.push_str(action);
        }

        self.cast()
    }

    /// Append an entity ID to the path.
    ///
    /// This is usually an [`Id`].
    pub fn id(mut self, id: impl PathId) -> PathBuilder<IdMarker> {
        debug_assert!(!self.has_query_parameter);

        self.buffer = self.buffer.and_then(|mut buffer| {
            write!(buffer, "/{id}")?;

            Ok(buffer)
        });

        self.cast()
    }

    /// Append a query parameter to the path.
    ///
    /// Once a query parameter has been appended then only more query parameters
    /// can be appended or the builder can be finalized.
    ///
    /// Refer to [`query_parameter`] for more information on how to use query
    /// parameters.
    ///
    /// [`query_parameter`]: super::query_parameter
    pub fn parameter(self, query_parameter: impl QueryParameter) -> PathBuilder<QueryMarker> {
        debug_assert!(!self.has_query_parameter);

        PathBuilder::<QueryMarker>::parameter(self.cast(), query_parameter)
    }
}

#[cfg(test)]
mod tests {
    use super::{Me, PathBuilder};
    use twilight_model::id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, ScheduledEventMarker, UserMarker},
    };

    #[test]
    fn me_display() {
        assert_eq!("@me", Me.to_string());
    }

    /// Test a path operating on a resource.
    #[test]
    fn resource() {
        assert_eq!(
            "gateway",
            PathBuilder::new().resource("gateway").build().unwrap()
        );
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
                .unwrap()
        );
    }

    /// Test a path operating on a resource with the current user as the entity.
    #[test]
    fn resource_me() {
        assert_eq!(
            "oauth2/@me",
            PathBuilder::new()
                .resource("oauth2")
                .id(Me)
                .build()
                .unwrap()
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
                .unwrap()
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
                .unwrap()
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
                .parameter(("limit", 100u64))
                .build()
                .unwrap()
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
                .parameter(("with_counts", true))
                .build()
                .unwrap()
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
                .parameter(("with_counts", false))
                .build()
                .unwrap()
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
                .parameter(("query", "abc"))
                .build()
                .unwrap()
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
            .parameter(("after", Id::<UserMarker>::new(3)))
            .parameter(("before", Id::<UserMarker>::new(10)))
            .build()
            .unwrap();

        assert_eq!("guilds/1/scheduled-events/2/users?after=3&before=10", path);
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
            .parameter(("after", None::<Id<UserMarker>>))
            .parameter(("limit", Some(10u64)))
            .build()
            .unwrap();

        assert_eq!("foo?limit=10", path);
    }
}
