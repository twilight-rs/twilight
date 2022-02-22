use crate::{error::Error as HttpError, request::Request};
use serde::Serialize;

/// Trait for supplying a payload to a request builder.
pub trait Payload<'a>: private::Sealed {
    /// Supply a payload to a request builder, building the request.
    fn payload(self, payload: &'a impl Serialize) -> Result<Request, HttpError>;
}

mod private {
    use crate::request::application::command::{CreateGlobalCommand, CreateGuildCommand};
    pub trait Sealed {}

    impl Sealed for CreateGlobalCommand<'_> {}
    impl Sealed for CreateGuildCommand<'_> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::application::command::{CreateGlobalCommand, CreateGuildCommand};
    use static_assertions::assert_impl_all;

    assert_impl_all!(CreateGlobalCommand<'_>: Payload<'static>);
    assert_impl_all!(CreateGuildCommand<'_>: Payload<'static>);
}
