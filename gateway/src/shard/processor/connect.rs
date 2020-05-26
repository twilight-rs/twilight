use super::super::error::{Error, Result};
use log::debug;
use std::str::FromStr;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use url::Url;

use super::super::ShardStream;

pub async fn connect(url: &str) -> Result<ShardStream> {
    let url = Url::from_str(url).map_err(|source| Error::ParsingUrl {
        source,
        url: url.to_owned(),
    })?;

    let request = url
        .into_client_request()
        .map_err(|source| Error::Connecting { source })?;
    let (stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .map_err(|source| Error::Connecting { source })?;

    debug!("Shook hands with remote");

    Ok(stream)
}
