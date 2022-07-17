//! Request builders for interacting with scheduled events.

mod create_guild_scheduled_event;
mod delete_guild_scheduled_event;
mod get_guild_scheduled_event;
mod get_guild_scheduled_event_users;
mod get_guild_scheduled_events;
mod update_guild_scheduled_event;

pub use self::{
    create_guild_scheduled_event::{
        CreateGuildExternalScheduledEvent, CreateGuildScheduledEvent,
        CreateGuildStageInstanceScheduledEvent, CreateGuildVoiceScheduledEvent,
    },
    delete_guild_scheduled_event::DeleteGuildScheduledEvent,
    get_guild_scheduled_event::GetGuildScheduledEvent,
    get_guild_scheduled_event_users::GetGuildScheduledEventUsers,
    get_guild_scheduled_events::GetGuildScheduledEvents,
    update_guild_scheduled_event::UpdateGuildScheduledEvent,
};

use serde::Serialize;

#[derive(Serialize)]
struct EntityMetadataFields<'a> {
    location: Option<&'a str>,
}
