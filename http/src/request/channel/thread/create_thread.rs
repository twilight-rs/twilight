use super::{ThreadValidationError, ThreadValidationErrorType};
use crate::{
    client::Client,
    error::Error as HttpError,
    request::{validate, Request},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel, ChannelType},
    id::ChannelId,
};

#[derive(Serialize)]
struct CreateThreadFields<'a> {
    auto_archive_duration: AutoArchiveDuration,
    #[serde(rename = "type")]
    kind: ChannelType,
    name: &'a str,
}

/// Start a thread that is not connected to a message.
///
/// To use auto archive durations of [`ThreeDays`] or [`Week`], the guild must
/// be boosted.
///
/// [`ThreeDays`]: twilight_model::channel::thread::AutoArchiveDuration::ThreeDays
/// [`Week`]: twilight_model::channel::thread::AutoArchiveDuration::Week
pub struct CreateThread<'a> {
    channel_id: ChannelId,
    fields: CreateThreadFields<'a>,
    http: &'a Client,
}

impl<'a> CreateThread<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        name: &'a str,
        auto_archive_duration: AutoArchiveDuration,
        kind: ChannelType,
    ) -> Result<Self, ThreadValidationError> {
        if !validate::channel_name(name) {
            return Err(ThreadValidationError {
                kind: ThreadValidationErrorType::NameInvalid,
            });
        }

        if !validate::is_thread(kind) {
            return Err(ThreadValidationError {
                kind: ThreadValidationErrorType::TypeInvalid { kind },
            });
        }

        Ok(Self {
            channel_id,
            fields: CreateThreadFields {
                auto_archive_duration,
                kind,
                name,
            },
            http,
        })
    }

    fn request(&self) -> Result<Request, HttpError> {
        let request = Request::builder(&Route::CreateThread {
            channel_id: self.channel_id.0,
        })
        .json(&self.fields)?;

        Ok(request.build())
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Channel> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}
