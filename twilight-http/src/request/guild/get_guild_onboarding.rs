use std::future::IntoFuture;

use twilight_model::{
    guild::onboarding::Onboarding,
    id::{Id, marker::GuildMarker},
};

use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::ResponseFuture,
};

struct GetGuildOnboardingPath {
    guild_id: Id<GuildMarker>,
}

/// Get the onboarding information for a guild.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("token".to_owned());
///
/// let guild_id = Id::new(101);
/// let onboarding = client.guild_onboarding(guild_id).await?.model().await?;
///
/// for prompt in onboarding.prompts {
///     println!("Prompt: {}", prompt.title);
/// }
/// # Ok(()) }
/// ```
pub struct GetGuildOnboarding<'a> {
    path: GetGuildOnboardingPath,
    http: &'a Client,
}

impl<'a> GetGuildOnboarding<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildOnboardingPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildOnboarding<'_> {
    type Output = Result<Response<Onboarding>, Error>;

    type IntoFuture = ResponseFuture<Onboarding>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildOnboarding<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildOnboardingPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("onboarding")
            .build()
            .map_err(Error::path)?;

        let request = Request::new(Method::Get, path);

        Ok(request)
    }
}
