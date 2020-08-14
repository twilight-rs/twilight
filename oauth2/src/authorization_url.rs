use super::{Client, Prompt, Scope};
use std::fmt::Write;
use url::{ParseError, Url};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RedirectUriInvalidError<'a> {
    /// The provided URI string isn't a valid URI.
    Invalid {
        /// Reason for the error.
        source: ParseError,
        /// Provided URI string.
        uri: &'a str,
    },
    /// The provided redirect URI is valid, but isn't in the client's list of
    /// configured redirect URIs.
    Unconfigured {
        /// Parsed URI.
        uri: Url,
    },
}

pub struct AuthorizationUrlBuilder<'a> {
    client: &'a Client,
    prompt: Option<Prompt>,
    redirect_uri: Url,
    scopes: Option<&'a [Scope]>,
    state: Option<&'a str>,
}

impl<'a> AuthorizationUrlBuilder<'a> {
    pub(crate) fn new(
        client: &'a Client,
        redirect_uri: &'a str,
    ) -> Result<Self, RedirectUriInvalidError<'a>> {
        let redirect_uri =
            Url::parse(redirect_uri).map_err(|source| RedirectUriInvalidError::Invalid {
                source,
                uri: redirect_uri,
            })?;
        if !client
            .redirect_uris()
            .iter()
            .any(|uri| *uri == redirect_uri)
        {
            return Err(RedirectUriInvalidError::Unconfigured { uri: redirect_uri });
        }

        Ok(Self {
            client,
            prompt: None,
            redirect_uri,
            scopes: None,
            state: None,
        })
    }

    pub fn build(&self) -> String {
        let mut url = Client::BASE_AUTHORIZATION_URL.to_owned();
        url.push('?');
        url.push_str("response_type=code");
        url.push_str("&client_id=");
        write!(url, "{}", self.client.client_id().0).expect("client id write can't error");

        if let Some(scopes) = self.scopes.as_ref() {
            url.push_str("&scope=");

            let scope_count = scopes.len().saturating_sub(1);

            for (idx, scope) in scopes.iter().enumerate() {
                url.push_str(scope.name());

                if idx < scope_count {
                    url.push_str("%20");
                }
            }
        }

        if let Some(state) = self.state.as_ref() {
            url.push_str("&state=");
            url.push_str(state);
        }

        url.push_str("&redirect_uri=");
        url.push_str(&urlencoding::encode(self.redirect_uri.as_ref()));

        if let Some(prompt) = self.prompt.as_ref().map(Prompt::name) {
            url.push_str("&prompt=");
            url.push_str(prompt);
        }

        url
    }

    /// Set how to prompt the user for authorization.
    ///
    /// Read the documentation for [`Prompt`] for information on what meaning
    /// each variant has.
    ///
    /// For the [`Scope::Bot`] and [`Scope::WebhookIncoming`] scopes the prompt
    /// will always be [`Prompt::Consent`]. Defaults to Discord's default.
    ///
    /// [`Prompt`]: enum.Prompt.html
    /// [`Scope::Bot`]: enum.Scope.html#variant.Bot
    /// [`Scope::WebhookIncoming`]: enum.Scope.html#variant.WebhookIncoming
    pub fn prompt(&mut self, prompt: Prompt) -> &mut Self {
        self.prompt.replace(prompt);

        self
    }

    /// Set the scopes for the authorization request.
    ///
    /// Read about Discord's [scope documentation].
    ///
    /// [RFC 6749 § 3.3] on access token scopes.
    ///
    /// [RFC 6749 § 3.3]: https://tools.ietf.org/html/rfc6749#section-3.3
    /// [scope documentation]: https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
    pub fn scopes(&mut self, scopes: &'a [Scope]) -> &mut Self {
        self.scopes.replace(scopes);

        self
    }

    /// Set the state for the authorization request.
    ///
    /// Read about Discord's recommendations for [state and security] for more
    /// information.
    ///
    /// [RFC 6749 § 4.1.1] on access token scopes.
    ///
    /// [RFC 6749 § 4.1.1]: https://tools.ietf.org/html/rfc6749#section-4.1.1
    /// [state and security]: https://discord.com/developers/docs/topics/oauth2#state-and-security
    pub fn state(&mut self, state: &'a str) -> &mut Self {
        self.state.replace(state);

        self
    }
}
