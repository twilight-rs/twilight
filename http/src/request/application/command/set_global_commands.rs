use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request, RequestBuilder},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{application::command::Command, id::ApplicationId};

/// Set global commands.
///
/// This method is idempotent: it can be used on every start, without being
/// ratelimited if there aren't changes to the commands.
///
/// The [`Command`] struct has an [associated builder] in the
/// [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/builder/command/struct.CommandBuilder.html
#[must_use = "requests must be configured and executed"]
pub struct SetGlobalCommands<'a> {
    commands: &'a [Command],
    application_id: ApplicationId,
    http: &'a Client,
}

impl<'a> SetGlobalCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        commands: &'a [Command],
    ) -> Self {
        Self {
            commands,
            application_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Command>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for SetGlobalCommands<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::SetGlobalCommands {
            application_id: self.application_id.get(),
        })
        .json(&self.commands)
        .map(RequestBuilder::build)
    }
}
