macro_rules! poll_req {
    ($ty: ty, $ret: ty) => {
        impl std::future::Future for $ty {
            type Output = $crate::error::Result<$ret>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
                let this = self.get_mut();

                loop {
                    match this.fut {
                        Some(ref mut fut) => return std::pin::Pin::new(fut).poll(cx),
                        None => {
                            if let Err(why) = this.start() {
                                return std::task::Poll::Ready(Err(why));
                            }
                        }
                    }
                }
            }
        }
    };
}

mod create_ban;
mod create_emoji;
mod create_guild;
mod create_guild_channel;
mod create_guild_prune;
mod create_invite;
mod create_message;
mod create_role;
mod create_webhook;
mod get_current_user_guilds;
mod delete_webhook;
mod execute_webhook;
mod get_audit_log;
mod get_channel_messages;
mod get_channel_messages_configured;
mod get_gateway;
mod get_gateway_authed;
mod get_guild_members;
mod get_guild_members_iter;
mod get_guild_prune_count;
mod get_invite;
mod get_reactions;
mod get_webhook;
mod prelude;
mod update_channel;
mod update_channel_permission;
mod update_channel_permission_configured;
mod update_current_user;
mod update_emoji;
mod update_guild;
mod update_guild_embed;
mod update_guild_member;
mod update_message;
mod update_role;
mod update_webhook;
mod update_webhook_with_token;

pub use self::{
    create_ban::CreateBan,
    create_emoji::CreateEmoji,
    create_guild::CreateGuild,
    create_guild_channel::CreateGuildChannel,
    create_guild_prune::CreateGuildPrune,
    create_invite::CreateInvite,
    create_message::CreateMessage,
    create_role::CreateRole,
    create_webhook::CreateWebhook,
    delete_webhook::DeleteWebhook,
    execute_webhook::ExecuteWebhook,
    get_audit_log::GetAuditLog,
    get_channel_messages::GetChannelMessages,
    get_channel_messages_configured::GetChannelMessagesConfigured,
    get_current_user_guilds::GetCurrentUserGuilds,
    get_gateway::GetGateway,
    get_gateway_authed::GetGatewayAuthed,
    get_guild_members::GetGuildMembers,
    get_guild_members_iter::GetGuildMembersIter,
    get_guild_prune_count::GetGuildPruneCount,
    get_invite::GetInvite,
    get_reactions::GetReactions,
    get_webhook::GetWebhook,
    update_channel::UpdateChannel,
    update_channel_permission::UpdateChannelPermission,
    update_channel_permission_configured::UpdateChannelPermissionConfigured,
    update_current_user::UpdateCurrentUser,
    update_emoji::UpdateEmoji,
    update_guild::UpdateGuild,
    update_guild_embed::UpdateGuildEmbed,
    update_guild_member::UpdateGuildMember,
    update_message::UpdateMessage,
    update_role::UpdateRole,
    update_webhook::UpdateWebhook,
    update_webhook_with_token::UpdateWebhookWithToken,
};

use crate::routing::{Path, Route};
use http::{
    header::{HeaderMap, HeaderValue},
    method::Method,
};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub body: Option<Vec<u8>>,
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub method: Method,
    pub path: Path,
    pub path_str: Cow<'a, str>,
}

impl<'a> Request<'a> {
    pub fn new(
        body: Option<Vec<u8>>,
        headers: Option<HeaderMap<HeaderValue>>,
        route: Route<'a>,
    ) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body,
            headers,
            method,
            path,
            path_str,
        }
    }
}

impl<'a> From<Route<'a>> for Request<'a> {
    fn from(route: Route<'a>) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: None,
            headers: None,
            method,
            path,
            path_str,
        }
    }
}

impl<'a> From<(Vec<u8>, Route<'a>)> for Request<'a> {
    fn from((body, route): (Vec<u8>, Route<'a>)) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: Some(body),
            headers: None,
            method,
            path,
            path_str,
        }
    }
}

impl<'a> From<(Vec<u8>, HeaderMap<HeaderValue>, Route<'a>)> for Request<'a> {
    fn from(
        (body, headers, route): (Vec<u8>, HeaderMap<HeaderValue>, Route<'a>),
    ) -> Self {
        let (method, path, path_str) = route.into_parts();

        Self {
            body: Some(body),
            headers: Some(headers),
            method,
            path,
            path_str,
        }
    }
}
