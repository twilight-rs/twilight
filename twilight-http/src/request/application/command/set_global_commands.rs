use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::command::Command,
    id::{marker::ApplicationMarker, Id},
};

/// Set global commands.
///
/// This method is idempotent: it can be used on every start, without being
/// ratelimited if there aren't changes to the commands.
///
/// The [`Command`] struct has an [associated builder] in the
/// [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/struct.CommandBuilder.html
#[must_use = "requests must be configured and executed"]
pub struct SetGlobalCommands<'a> {
    commands: &'a [Command],
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
}

impl<'a> SetGlobalCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        commands: &'a [Command],
    ) -> Self {
        Self {
            commands,
            application_id,
            http,
        }
    }
}

impl IntoFuture for SetGlobalCommands<'_> {
    type Output = Result<Response<ListBody<Command>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Command>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SetGlobalCommands<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::SetGlobalCommands {
            application_id: self.application_id.get(),
        })
        .json(&self.commands)
        .build()
    }
}
