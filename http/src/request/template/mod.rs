pub mod create_guild_from_template;
pub mod create_template;
pub mod update_template;

mod delete_template;
mod get_template;
mod get_templates;
mod sync_template;

pub use self::{
    create_guild_from_template::CreateGuildFromTemplate, create_template::CreateTemplate,
    delete_template::DeleteTemplate, get_template::GetTemplate, get_templates::GetTemplates,
    sync_template::SyncTemplate, update_template::UpdateTemplate,
};
