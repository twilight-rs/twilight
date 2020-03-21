use super::CreateMessage;
use dawn_model::id::{RoleId, UserId};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Parsed;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExplicitUser(Vec<UserId>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExplicitRole(Vec<RoleId>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Unspecified;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParseTypes {
    Users,
    Roles,
    Everyone,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Default, Debug, Eq, Hash, PartialEq)]
#[must_use = "It will not be added unless `build()` is called."]
pub struct AllowedMentions {
    parse: Vec<ParseTypes>,
    users: Option<Vec<UserId>>,
    roles: Option<Vec<RoleId>>,
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

pub struct AllowedMentionsBuilder<'a, E, U, R> {
    create_message: CreateMessage<'a>,
    e: E,
    u: U,
    r: R,
}

impl<'a> AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
    pub(crate) fn new(create_message: CreateMessage<'a>) -> Self {
        Self {
            create_message,
            e: Unspecified,
            u: Unspecified,
            r: Unspecified,
        }
    }
}

impl<'a, U, R> AllowedMentionsBuilder<'a, Unspecified, U, R> {
    pub fn parse_everyone(self) -> AllowedMentionsBuilder<'a, Parsed, U, R> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: Parsed,
            u: self.u,
            r: self.r,
        }
    }
}

impl<'a, E, R> AllowedMentionsBuilder<'a, E, Unspecified, R> {
    pub fn parse_users(self) -> AllowedMentionsBuilder<'a, E, Parsed, R> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: self.e,
            u: Parsed,
            r: self.r,
        }
    }

    pub fn parse_specific_users(
        self,
        u: impl IntoIterator<Item = UserId>,
    ) -> AllowedMentionsBuilder<'a, E, ExplicitUser, R> {
        let vec = u.into_iter().collect::<Vec<_>>();
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: self.e,
            u: ExplicitUser(vec),
            r: self.r,
        }
    }
}

impl<'a, E, U> AllowedMentionsBuilder<'a, E, U, Unspecified> {
    pub fn parse_roles(self) -> AllowedMentionsBuilder<'a, E, U, Parsed> {
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: self.e,
            u: self.u,
            r: Parsed,
        }
    }

    pub fn parse_specific_roles(
        self,
        r: impl IntoIterator<Item = RoleId>,
    ) -> AllowedMentionsBuilder<'a, E, U, ExplicitRole> {
        let vec = r.into_iter().collect::<Vec<_>>();
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: self.e,
            u: self.u,
            r: ExplicitRole(vec),
        }
    }
}

impl<'a, E, U> AllowedMentionsBuilder<'a, E, U, ExplicitRole> {
    pub fn parse_specific_roles(mut self, r: impl IntoIterator<Item = RoleId>) -> Self {
        self.r.0.extend(r);
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: self.e,
            u: self.u,
            r: self.r,
        }
    }
}

impl<'a, E, R> AllowedMentionsBuilder<'a, E, ExplicitUser, R> {
    pub fn parse_specific_users(mut self, u: impl IntoIterator<Item = UserId>) -> Self {
        self.u.0.extend(u);
        AllowedMentionsBuilder {
            create_message: self.create_message,
            e: self.e,
            u: self.u,
            r: self.r,
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
    pub fn build(mut self) -> CreateMessage<'a> {
        let mut m = AllowedMentions::default();

        self.e.visit(&mut m);
        self.u.visit(&mut m);
        self.r.visit(&mut m);

        self.create_message.fields.allowed_mentions.replace(m);
        self.create_message
    }
}
