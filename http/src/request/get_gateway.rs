use dawn_model::gateway::connection_info::ConnectionInfo;
use super::{
    GetGatewayAuthed,
    prelude::*,
};

pub struct GetGateway<'a> {
    fut: Option<PendingBody<'a, ConnectionInfo>>,
    http: &'a Client,
}

impl<'a> GetGateway<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fut: None,
            http,
        }
    }

    pub fn authed(self) -> GetGatewayAuthed<'a> {
        GetGatewayAuthed::new(self.http)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request::from(Route::GetGateway))?);

        Ok(())
    }
}

poll_req!(GetGateway<'_>, ConnectionInfo);
