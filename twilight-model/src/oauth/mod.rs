pub mod team;

pub mod current_application_info {
    #[deprecated(since = "0.10.2", note = "use `oauth::ApplicationFlags` instead")]
    pub type ApplicationFlags = super::ApplicationFlags;

    #[deprecated(since = "0.10.2", note = "use `oauth::Application` instead")]
    pub type CurrentApplicationInfo = super::Application;
}

mod application;
mod application_flags;
mod current_authorization_information;
mod install_params;
mod partial_application;
mod scope;

pub use self::{
    application::Application, application_flags::ApplicationFlags,
    current_authorization_information::CurrentAuthorizationInformation,
    install_params::InstallParams, partial_application::PartialApplication, scope::Scope,
};

#[allow(deprecated)]
pub use self::current_application_info::CurrentApplicationInfo;
