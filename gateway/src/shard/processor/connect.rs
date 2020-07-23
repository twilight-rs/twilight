use super::error::{Error, Result};
use url::Url;

use super::super::ShardStream;

pub async fn connect(url: &str) -> Result<ShardStream> {
    let url = Url::parse(url).map_err(|source| Error::ParsingUrl {
        source,
        url: url.to_owned(),
    })?;

    let (stream, _) = async_tungstenite::tokio::connect_async(url)
        .await
        .map_err(|source| Error::Connecting { source })?;

    tracing::debug!("Shook hands with remote");

    Ok(stream)
}
