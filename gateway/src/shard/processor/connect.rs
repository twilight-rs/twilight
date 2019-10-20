use super::super::error::{Error, Result};
use log::debug;
use std::str::FromStr;
use tokio_net::tcp::TcpStream;
use tokio_tungstenite::{tungstenite::handshake::client::Request, MaybeTlsStream, WebSocketStream};
use url::Url;

pub async fn connect(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let url = Url::from_str(url).map_err(|source| Error::ParsingUrl {
        source,
        url: url.to_owned(),
    })?;

    let request = Request::from(url);
    let (stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .map_err(|source| Error::Connecting {
            source,
        })?;

    debug!("Shook hands with remote");

    Ok(stream)
}
