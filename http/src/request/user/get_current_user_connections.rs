use crate::request::prelude::*;
use dawn_model::user::Connection;

pub struct GetCurrentUserConnections<'a> {
    fut: Option<Pending<'a, Vec<Connection>>>,
    http: &'a Client,
}

impl<'a> GetCurrentUserConnections<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http.request(Request::from(Route::GetUserConnections)),
        ));

        Ok(())
    }
}

poll_req!(GetCurrentUserConnections<'_>, Vec<Connection>);
