use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::User,
};

/// Get a user's information by id.
#[must_use = "requests must be configured and executed"]
pub struct GetUser<'a> {
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: Id<UserMarker>) -> Self {
        Self { http, user_id }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<User> {
        let request = Request::from_route(&Route::GetUser {
            user_id: self.user_id.get(),
        });

        self.http.request(request)
    }
}
