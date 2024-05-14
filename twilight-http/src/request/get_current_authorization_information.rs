use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::oauth::CurrentAuthorizationInformation;

/// Retrieve information about the current OAuth2 authorization.
///
/// Returns the application's, authorization's, and if applicable the user's
/// details.
///
/// Refer to [Discord Docs/Get Current Authorization Information][1].
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
///
/// let bearer_token = env::var("BEARER_TOKEN")?;
///
/// let client = Client::new(bearer_token);
/// let response = client.current_authorization().await?;
/// let authorization = response.model().await?;
///
/// println!("Application: {}", authorization.application.name);
///
/// if let Some(user) = authorization.user {
///     println!("User: {}", user.name);
/// }
/// # Ok(()) }
/// ```
///
/// [1]: https://discord.com/developers/docs/topics/oauth2#get-current-authorization-information
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentAuthorizationInformation<'a> {
    http: &'a Client,
}

impl<'a> GetCurrentAuthorizationInformation<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetCurrentAuthorizationInformation<'_> {
    type Output = Result<Response<CurrentAuthorizationInformation>, Error>;

    type IntoFuture = ResponseFuture<CurrentAuthorizationInformation>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentAuthorizationInformation<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(
            &Route::GetCurrentAuthorizationInformation,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::GetCurrentAuthorizationInformation;
    use crate::{client::Client, request::TryIntoRequest};
    use static_assertions::assert_impl_all;
    use std::{error::Error, future::IntoFuture};
    use twilight_http_ratelimiting::{Method, Path};

    assert_impl_all!(GetCurrentAuthorizationInformation<'_>: IntoFuture, Send, Sync, TryIntoRequest);

    #[test]
    fn get_current_authorization_information() -> Result<(), Box<dyn Error>> {
        let client = Client::new(String::new());
        let req = client.current_authorization().try_into_request()?;

        assert!(req.use_authorization_token());
        assert!(req.body().is_none());
        assert!(req.form().is_none());
        assert!(req.headers().is_none());
        assert_eq!(Method::Get, req.method());
        assert_eq!(&Path::OauthMe, req.ratelimit_path());

        Ok(())
    }
}
