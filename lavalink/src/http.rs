//! Models to deserialize responses into and functions to create `http` crate
//! requests.

use http::{
    header::{HeaderValue, AUTHORIZATION},
    Error as HttpError, Request,
};
use percent_encoding::NON_ALPHANUMERIC;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoadType {
    LoadFailed,
    NoMatches,
    PlaylistLoaded,
    SearchResult,
    TrackLoaded,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub info: TrackInfo,
    pub track: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackInfo {
    pub author: String,
    pub identifier: String,
    pub is_seekable: bool,
    pub is_stream: bool,
    pub length: u64,
    pub position: u64,
    pub title: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    pub name: Option<String>,
    pub selected_track: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct LoadedTracks {
    pub load_type: LoadType,
    pub playlist_info: PlaylistInfo,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FailingAddress {
    pub address: String,
    pub failing_timestamp: u64,
    pub failing_time: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum IpBlockType {
    #[serde(rename = "Inet4Address")]
    Inet4,
    #[serde(rename = "Inet6Address")]
    Inet6,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct IpBlock {
    pub kind: IpBlockType,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "PascalCase")]
pub enum RoutePlannerType {
    NanoIp,
    RotatingIp,
    RotatingNanoIp,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum RoutePlanner {
    NanoIp(NanoIpRoutePlanner),
    RotatingIp(RotatingIpRoutePlanner),
    RotatingNanoIp(RotatingNanoIpRoutePlanner),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NanoIpRoutePlanner {
    pub class: RoutePlannerType,
    pub details: NanoIpDetails,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NanoIpDetails {
    pub failing_addresses: Vec<FailingAddress>,
    pub ip_block: IpBlock,
    pub current_address_index: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingIpRoutePlanner {
    pub class: RoutePlannerType,
    pub details: RotatingIpDetails,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingIpDetails {
    pub current_address: String,
    pub failing_addresses: Vec<FailingAddress>,
    pub ip_block: IpBlock,
    pub ip_index: u64,
    pub rotate_index: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingNanoIpRoutePlanner {
    pub class: RoutePlannerType,
    pub details: RotatingNanoIpDetails,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingNanoIpDetails {
    pub block_index: String,
    pub current_address_index: u64,
    pub failing_addresses: Vec<FailingAddress>,
    pub ip_block: IpBlock,
}

/// Get a list of tracks that match an identifier.
///
/// The response will include a body which can be deserialized into a
/// [`LoadedTracks`].
///
/// [`LoadedTracks`]: struct.LoadedTracks.html
pub fn load_track(
    address: SocketAddr,
    identifier: impl AsRef<str>,
    authorization: impl AsRef<str>,
) -> Result<Request<&'static [u8]>, HttpError> {
    let identifier =
        percent_encoding::percent_encode(identifier.as_ref().as_bytes(), NON_ALPHANUMERIC);
    let url = format!("http://{}/loadtracks?identifier={}", address, identifier);

    let mut req = Request::get(url);

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(b"")
}

/// Get the configured route planner for a node by address.
///
/// The response will include a body which can be deserialized into a
/// [`RoutePlanner`].
///
/// [`RoutePlanner`]: enum.RoutePlanner.html
pub fn get_route_planner(
    address: SocketAddr,
    authorization: impl AsRef<str>,
) -> Result<Request<&'static [u8]>, HttpError> {
    let mut req = Request::get(format!("{}/routeplanner/status", address));

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(b"")
}

pub fn unmark_failed_address(
    node_address: impl Into<SocketAddr>,
    authorization: impl AsRef<str>,
    route_address: impl Into<IpAddr>,
) -> Result<Request<Vec<u8>>, HttpError> {
    let mut req = Request::post(format!("{}/routeplanner/status", node_address.into()));

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(
        serde_json::to_vec(&serde_json::json!({
            "address": route_address.into(),
        }))
        .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        FailingAddress, IpBlock, IpBlockType, NanoIpDetails, NanoIpRoutePlanner, RotatingIpDetails,
        RotatingIpRoutePlanner, RotatingNanoIpDetails, RotatingNanoIpRoutePlanner, RoutePlanner,
        RoutePlannerType,
    };
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        FailingAddress: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        IpBlockType: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        IpBlock: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        NanoIpDetails: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        NanoIpRoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        RotatingIpDetails: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        RotatingIpRoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        RotatingNanoIpDetails: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        RotatingNanoIpRoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        RoutePlannerType: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
    assert_impl_all!(
        RoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );
}
