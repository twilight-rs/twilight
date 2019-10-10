use super::error::{Connecting, ParsingUrl, Result};
use log::debug;
use snafu::ResultExt;
use std::str::FromStr;
use tokio_net::tcp::TcpStream;
use tokio_tungstenite::{tungstenite::handshake::client::Request, MaybeTlsStream, WebSocketStream};
use url::Url;

pub async fn connect(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let url = Url::from_str(url).with_context(|| ParsingUrl {
        url: url.to_owned(),
    })?;

    let request = Request::from(url);
    let (stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .context(Connecting)?;

    debug!("Shook hands with remote");

    Ok(stream)
}
