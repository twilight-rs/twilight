use crate::request::prelude::*;
use twilight_model::invite::Invite;

#[derive(Default)]
struct GetInviteFields {
    with_counts: bool,
    with_expiration: bool,
}

/// Get information about an invite by its code.
///
/// If [`with_counts`] is called, the returned invite will contain approximate
/// member counts. If [`with_expiration`] is called, it will contain the
/// expiration date.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let invite = client
///     .invite("code")
///     .with_counts()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`with_counts`]: Self::with_counts
/// [`with_expiration`]: Self::with_expiration
pub struct GetInvite<'a> {
    code: String,
    fields: GetInviteFields,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
}

impl<'a> GetInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            fields: GetInviteFields::default(),
            fut: None,
            http,
        }
    }

    /// Whether the invite returned should contain approximate member counts.
    pub fn with_counts(mut self) -> Self {
        self.fields.with_counts = true;

        self
    }

    /// Whether the invite returned should contain its expiration date.
    pub fn with_expiration(mut self) -> Self {
        self.fields.with_expiration = true;

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetInvite {
                    code: self.code.clone(),
                    with_counts: self.fields.with_counts,
                    with_expiration: self.fields.with_expiration,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetInvite<'_>, Invite);
