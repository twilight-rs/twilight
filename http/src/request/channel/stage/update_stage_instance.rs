use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::ChannelId;

/// The request can not be created as configured.
#[derive(Debug)]
pub struct UpdateStageInstanceError {
    kind: UpdateStageInstanceErrorType,
}

impl UpdateStageInstanceError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &UpdateStageInstanceErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        UpdateStageInstanceErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for UpdateStageInstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            UpdateStageInstanceErrorType::InvalidTopic { .. } => {
                f.write_fmt(format_args!("invalid topic"))
            }
        }
    }
}

impl Error for UpdateStageInstanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum UpdateStageInstanceErrorType {
    /// Topic is not between 1 and 120 characters in length.
    InvalidTopic {
        /// Invalid topic.
        topic: String,
    },
}

#[derive(Serialize)]
struct UpdateStageInstanceFields {
    topic: String,
}

/// Update fields of an existing stage instance.
///
/// Requires the user to be a moderator of the stage channel. The topic must
/// be between 1 and 120 characters in length.
pub struct UpdateStageInstance<'a> {
    channel_id: ChannelId,
    fields: UpdateStageInstanceFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> UpdateStageInstance<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        topic: impl Into<String>,
    ) -> Result<Self, UpdateStageInstanceError> {
        Self::_new(http, channel_id, topic.into())
    }

    fn _new(
        http: &'a Client,
        channel_id: ChannelId,
        topic: String,
    ) -> Result<Self, UpdateStageInstanceError> {
        if !validate::stage_topic(&topic) {
            return Err(UpdateStageInstanceError {
                kind: UpdateStageInstanceErrorType::InvalidTopic { topic },
            });
        }

        Ok(Self {
            channel_id,
            fields: UpdateStageInstanceFields { topic },
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::UpdateStageInstance {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateStageInstance<'_>, ());
