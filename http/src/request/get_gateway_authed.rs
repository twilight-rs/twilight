use dawn_model::gateway::connection_info::BotConnectionInfo;
use super::prelude::*;

#[derive(Serialize)]
pub struct GetGatewayAuthed<'a> {
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<BotConnectionInfo>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> GetGatewayAuthed<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self {
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(Route::GetGatewayBot))));

        Ok(())
    }
}

poll_req!(GetGatewayAuthed<'_>, BotConnectionInfo);
