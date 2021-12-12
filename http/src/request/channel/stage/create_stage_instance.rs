use crate::{
    client::Client,
    error::Error as HttpError,
    request::{validate_inner, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::stage_instance::PrivacyLevel,
    id::{marker::ChannelMarker, Id},
};

/// The request can not be created as configured.
#[derive(Debug)]
pub struct CreateStageInstanceError {
    kind: CreateStageInstanceErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CreateStageInstanceError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CreateStageInstanceErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CreateStageInstanceErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl Display for CreateStageInstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CreateStageInstanceErrorType::InvalidTopic => f.write_str("invalid topic"),
        }
    }
}

impl Error for CreateStageInstanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

#[derive(Debug)]
pub enum CreateStageInstanceErrorType {
    /// Topic is not between 1 and 120 characters in length.
    InvalidTopic,
}

#[derive(Serialize)]
struct CreateStageInstanceFields<'a> {
    channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_level: Option<PrivacyLevel>,
    topic: &'a str,
}

/// Create a new stage instance associated with a stage channel.
///
/// Requires the user to be a moderator of the stage channel.
#[must_use = "requests must be configured and executed"]
pub struct CreateStageInstance<'a> {
    fields: CreateStageInstanceFields<'a>,
    http: &'a Client,
}

impl<'a> CreateStageInstance<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        topic: &'a str,
    ) -> Result<Self, CreateStageInstanceError> {
        if !validate_inner::stage_topic(topic) {
            return Err(CreateStageInstanceError {
                kind: CreateStageInstanceErrorType::InvalidTopic,
                source: None,
            });
        }

        Ok(Self {
            fields: CreateStageInstanceFields {
                channel_id,
                privacy_level: None,
                topic,
            },
            http,
        })
    }

    /// Set the [`PrivacyLevel`] of the instance.
    pub const fn privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        self.fields.privacy_level = Some(privacy_level);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateStageInstance<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::CreateStageInstance);

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
