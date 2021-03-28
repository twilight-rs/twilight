use super::{message::CreateMessage, webhook::ExecuteWebhook};
use twilight_model::id::{RoleId, UserId};

/// Whether or not the section will be parsed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Parsed;

/// A vec of explicit users to be parsed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExplicitUser(Vec<UserId>);

/// A vec of explicit roles to be parsed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExplicitRole(Vec<RoleId>);

/// Not currently specified.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Unspecified;

/// Parse types.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum ParseTypes {
    Users,
    Roles,
    Everyone,
}

/// Stores the allowed mentions.
#[derive(serde::Deserialize, serde::Serialize, Clone, Default, Debug, Eq, Hash, PartialEq)]
#[must_use = "It will not be added unless `build()` is called."]
pub struct AllowedMentions {
    parse: Vec<ParseTypes>,
    users: Option<Vec<UserId>>,
    roles: Option<Vec<RoleId>>,
    replied_user: bool,
}

pub trait VisitAllowedMentionsEveryone: Sized {
    fn visit(self, _: &mut AllowedMentions) {}
}

pub trait VisitAllowedMentionsUsers: Sized {
    fn visit(self, _: &mut AllowedMentions) {}
}

pub trait VisitAllowedMentionsRoles: Sized {
    fn visit(self, _: &mut AllowedMentions) {}
}

impl VisitAllowedMentionsEveryone for Unspecified {}
impl VisitAllowedMentionsUsers for Unspecified {}
impl VisitAllowedMentionsRoles for Unspecified {}

impl VisitAllowedMentionsEveryone for Parsed {
    fn visit(self, d: &mut AllowedMentions) {
        d.parse.push(ParseTypes::Everyone);
    }
}

impl VisitAllowedMentionsUsers for Parsed {
    fn visit(self, d: &mut AllowedMentions) {
        d.parse.push(ParseTypes::Users);
    }
}

impl VisitAllowedMentionsRoles for Parsed {
    fn visit(self, d: &mut AllowedMentions) {
        d.parse.push(ParseTypes::Roles);
    }
}

impl VisitAllowedMentionsUsers for ExplicitUser {
    fn visit(self, d: &mut AllowedMentions) {
        d.users = Some(self.0)
    }
}

impl VisitAllowedMentionsRoles for ExplicitRole {
    fn visit(self, d: &mut AllowedMentions) {
        d.roles = Some(self.0)
    }
}

/// A builder for allowed mentions.
///
/// # Example
///
/// ```rust,no_run
/// use twilight_http::request::channel::allowed_mentions::AllowedMentionsBuilder;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let mut allowed_mentions = AllowedMentionsBuilder::new()
///     .parse_everyone()
///     .parse_users()
///     .build_solo();
/// # Ok(()) }
/// ```
pub struct AllowedMentionsBuilder<'a, E, U, R> {
    create_message: Option<CreateMessage<'a>>,
    execute_webhook: Option<ExecuteWebhook<'a>>,
    e: E,
    u: U,
    r: R,
    reply: bool,
}

impl<'a> AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
    pub(crate) fn for_builder(create_message: CreateMessage<'a>) -> Self {
        Self {
            create_message: Some(create_message),
            execute_webhook: None,
            e: Unspecified,
            u: Unspecified,
            r: Unspecified,
            reply: false,
        }
    }

    pub(crate) fn for_webhook(execute_webhook: ExecuteWebhook<'a>) -> Self {
        Self {
            create_message: None,
            execute_webhook: Some(execute_webhook),
            e: Unspecified,
            u: Unspecified,
            r: Unspecified,
            reply: false,
        }
    }

    /// Create the builder.
    pub fn new() -> Self {
        Self {
            create_message: None,
            execute_webhook: None,
            e: Unspecified,
            u: Unspecified,
            r: Unspecified,
            reply: false,
        }
    }
}

impl<'a> Default for AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, E, U, R> AllowedMentionsBuilder<'a, E, U, R> {
    /// Whether to mention the user being replied to.
    ///
    /// Defaults to false.
    pub fn replied_user(self, reply: bool) -> AllowedMentionsBuilder<'a, E, U, R> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: self.u,
            r: self.r,
            reply,
        }
    }
}

impl<'a, U, R> AllowedMentionsBuilder<'a, Unspecified, U, R> {
    /// Enable parsing for the `@everyone` and `@here` tags.
    pub fn parse_everyone(self) -> AllowedMentionsBuilder<'a, Parsed, U, R> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: Parsed,
            u: self.u,
            r: self.r,
            reply: self.reply,
        }
    }
}

impl<'a, E, R> AllowedMentionsBuilder<'a, E, Unspecified, R> {
    /// Enable parsing for all user tags.
    pub fn parse_users(self) -> AllowedMentionsBuilder<'a, E, Parsed, R> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: Parsed,
            r: self.r,
            reply: self.reply,
        }
    }

    /// Enable parsing for specific user tags.
    pub fn parse_specific_users(
        self,
        u: impl IntoIterator<Item = UserId>,
    ) -> AllowedMentionsBuilder<'a, E, ExplicitUser, R> {
        let vec = u.into_iter().collect::<Vec<_>>();
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: ExplicitUser(vec),
            r: self.r,
            reply: self.reply,
        }
    }
}

impl<'a, E, U> AllowedMentionsBuilder<'a, E, U, Unspecified> {
    /// Enable parsing for all role tags.
    pub fn parse_roles(self) -> AllowedMentionsBuilder<'a, E, U, Parsed> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: self.u,
            r: Parsed,
            reply: self.reply,
        }
    }

    /// Enable parsing for specific role tags.
    pub fn parse_specific_roles(
        self,
        r: impl IntoIterator<Item = RoleId>,
    ) -> AllowedMentionsBuilder<'a, E, U, ExplicitRole> {
        let vec = r.into_iter().collect::<Vec<_>>();
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: self.u,
            r: ExplicitRole(vec),
            reply: self.reply,
        }
    }
}

impl<'a, E, U> AllowedMentionsBuilder<'a, E, U, ExplicitRole> {
    /// Enable parsing for more specific role tags.
    ///
    /// If there are already some specific `RoleId`s in this builder, extend them with the content
    /// of `r`.
    pub fn parse_specific_roles(mut self, r: impl IntoIterator<Item = RoleId>) -> Self {
        self.r.0.extend(r);
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: self.u,
            r: self.r,
            reply: self.reply,
        }
    }
}

impl<'a, E, R> AllowedMentionsBuilder<'a, E, ExplicitUser, R> {
    /// Enable parsing for more specific user tags.
    ///
    /// If there are already some specific `UserId`s in this builder, extend them with the content
    /// of `u`.
    pub fn parse_specific_users(mut self, u: impl IntoIterator<Item = UserId>) -> Self {
        self.u.0.extend(u);
        AllowedMentionsBuilder {
            create_message: self.create_message,
            execute_webhook: self.execute_webhook,
            e: self.e,
            u: self.u,
            r: self.r,
            reply: self.reply,
        }
    }
}

impl<
        'a,
        E: VisitAllowedMentionsEveryone,
        U: VisitAllowedMentionsUsers,
        R: VisitAllowedMentionsRoles,
    > AllowedMentionsBuilder<'a, E, U, R>
{
    /// Return a [`CreateMessage`] struct with the specified `allowed_mentions`.
    ///
    /// # Panics
    ///
    /// Panics when no message builder was provided.
    pub fn build(self) -> CreateMessage<'a> {
        match self.create_message {
            Some(mut builder) => {
                let mut m = AllowedMentions::default();
                self.e.visit(&mut m);
                self.u.visit(&mut m);
                self.r.visit(&mut m);
                m.replied_user = self.reply;
                builder.fields.allowed_mentions.replace(m);
                builder
            }
            None => panic!(
                "Tried to build to a messagebuilder but none was provided during construction"
            ),
        }
    }

    /// Return a [`ExecuteWebhook`] struct with the specified `allowed_mentions`.
    ///
    /// # Panics
    ///
    /// Panics when no message builder was provided.
    pub fn build_webhook(self) -> ExecuteWebhook<'a> {
        match self.execute_webhook {
            Some(mut builder) => {
                let mut m = AllowedMentions::default();
                self.e.visit(&mut m);
                self.u.visit(&mut m);
                self.r.visit(&mut m);
                m.replied_user = self.reply;
                builder.fields.allowed_mentions.replace(m);
                builder
            }
            None => panic!(
                "Tried to build to a messagebuilder but none was provided during construction"
            ),
        }
    }

    /// Build a raw [`AllowedMentions`] for use in [`ClientBuilder::default_allowed_mentions`].
    ///
    /// [`ClientBuilder::default_allowed_mentions`]: crate::client::ClientBuilder::default_allowed_mentions
    pub fn build_solo(self) -> AllowedMentions {
        let mut m = AllowedMentions::default();

        self.e.visit(&mut m);
        self.u.visit(&mut m);
        self.r.visit(&mut m);
        m.replied_user = self.reply;
        m
    }
}
