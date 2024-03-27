//! Models to deserialize responses into and functions to create `http` crate
//! requests.

use crate::model::incoming::{Exception, Track};
use http::{
    header::{HeaderValue, AUTHORIZATION},
    Error as HttpError, Request,
};
use percent_encoding::NON_ALPHANUMERIC;
use serde::{Deserialize, Deserializer, Serialize};
use std::net::{IpAddr, SocketAddr};

/// Information about a playlist from a search result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    /// The name of the playlist
    pub name: String,
    /// The selected track within the playlist, if available.
    #[serde(default, deserialize_with = "deserialize_selected_track")]
    pub selected_track: Option<u64>,
}

// Any negative value should be treated as None.
fn deserialize_selected_track<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<i64>::deserialize(deserializer)
        .ok()
        .flatten()
        .and_then(|selected| u64::try_from(selected).ok()))
}

/// The type of search result given.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum LoadResultName {
    /// There has been no matches for your identifier.
    Empty,
    /// Loading has failed with an error.
    Error,
    /// A playlist has been loaded.
    Playlist,
    /// A search result has been loaded.
    Search,
    /// A track has been loaded.
    Track,
}

/// The type of search result given.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum LoadResultData {
    /// Empty data response.
    Empty(),
    /// The exception that was thrown when searching.
    Error(Exception),
    /// The playlist results with the play list info and tracks in the playlist.
    Playlist(PlaylistResult),
    /// The list of tracks based on the search.
    Search(Vec<Track>),
    /// Track result with the track info.
    Track(Track),
}

/// The playlist with the provided tracks. Currently plugin info isn't supported
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResult {
    /// The info of the playlist.
    pub info: PlaylistInfo,
    /// The tracks of the playlist.
    pub tracks: Vec<Track>,
}

/// Possible track results for a query.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct LoadedTracks {
    /// The data of the result.
    pub data: LoadResultData,
    /// The type of search result, such as a list of tracks or a playlist.
    pub load_type: LoadResultName,
}

/// A failing IP address within the planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FailingAddress {
    /// The IP address.
    pub address: String,
    /// The time that the address started failing in unix time.
    pub failing_timestamp: u64,
    /// The time that the address started failing as a timestamp.
    pub failing_time: String,
}

/// The IP version in use by the block.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum IpBlockType {
    /// An IPv4 block type.
    #[serde(rename = "Inet4Address")]
    Inet4,
    /// An IPv6 block type.
    #[serde(rename = "Inet6Address")]
    Inet6,
}

/// A block of IP addresses.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct IpBlock {
    /// The IP version of the IP block.
    pub kind: IpBlockType,
    /// The size of the block's addresses.
    pub size: u64,
}

/// The type of route planner in use.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "PascalCase")]
pub enum RoutePlannerType {
    /// A Nano IP route planner.
    NanoIp,
    /// A Rotating IP route planner.
    RotatingIp,
    /// A Rotating Nano IP route planner.
    RotatingNanoIp,
}

/// The route planner in use.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum RoutePlanner {
    /// Information about a Nano IP route planner.
    NanoIp(NanoIpRoutePlanner),
    /// Information about a Rotating IP route planner.
    RotatingIp(RotatingIpRoutePlanner),
    /// Information about a Rotating Nano IP route planner.
    RotatingNanoIp(RotatingNanoIpRoutePlanner),
}

/// A Nano IP planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NanoIpRoutePlanner {
    /// The type of planner that is currently active.
    ///
    /// For this planner, this is always [`RoutePlannerType::NanoIp`]
    pub class: RoutePlannerType,
    /// The details of the currently active Nano IP route planner.
    pub details: NanoIpDetails,
}

/// Information about a Nano IP planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NanoIpDetails {
    /// The active offset within the IP block.
    pub current_address_index: u64,
    /// A list of IP addresses in the range that are failing.
    pub failing_addresses: Vec<FailingAddress>,
    /// The associated IP block.
    pub ip_block: IpBlock,
}

/// A Rotating IP planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingIpRoutePlanner {
    /// The type of planner that is currently active.
    ///
    /// For this planner, this is always [`RoutePlannerType::RotatingIp`]
    pub class: RoutePlannerType,
    /// The details of the currently active rotating IP route planner.
    pub details: RotatingIpDetails,
}

/// Information about a Rotating IP planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingIpDetails {
    /// The currently used IP address.
    pub current_address: String,
    /// A list of IP addresses in the range that are failing.
    pub failing_addresses: Vec<FailingAddress>,
    /// The associated IP block.
    pub ip_block: IpBlock,
    /// The current offset used within the IP block.
    pub ip_index: u64,
    /// The number of rotations that have happened since the server started.
    pub rotate_index: u64,
}

/// A Rotating Nano IP planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingNanoIpRoutePlanner {
    /// The type of planner that is currently active.
    ///
    /// For this planner, this is always [`RoutePlannerType::RotatingNanoIp`]
    pub class: RoutePlannerType,
    /// The details of the currently active rotating nano IP route planner.
    pub details: RotatingNanoIpDetails,
}

/// Information about a Rotating Nano IP planner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RotatingNanoIpDetails {
    /// The block IPs that are chosen.
    pub block_index: String,
    /// The current IP address on rotation.
    pub current_address_index: u64,
    /// A list of IP addresses in the range that are failing.
    pub failing_addresses: Vec<FailingAddress>,
    /// The associated IP block.
    pub ip_block: IpBlock,
}

/// Get a list of tracks that match an identifier.
///
/// The response will include a body which can be deserialized into a
/// [`LoadedTracks`].
///
/// # Errors
///
/// See the documentation for [`http::Error`].
pub fn load_track(
    address: SocketAddr,
    identifier: impl AsRef<str>,
    authorization: impl AsRef<str>,
) -> Result<Request<&'static [u8]>, HttpError> {
    let identifier =
        percent_encoding::percent_encode(identifier.as_ref().as_bytes(), NON_ALPHANUMERIC);
    let url = format!("http://{address}/v4/loadtracks?identifier={identifier}");

    let mut req = Request::get(url);

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(b"")
}

/// Decode a single track into its info
///
/// The response will include a body which can be deserialized into a
/// [`Track`].
///
/// # Errors
///
/// See the documentation for [`http::Error`].
pub fn decode_track(
    address: SocketAddr,
    encoded: impl AsRef<str>,
    authorization: impl AsRef<str>,
) -> Result<Request<&'static [u8]>, HttpError> {
    let identifier =
        percent_encoding::percent_encode(encoded.as_ref().as_bytes(), NON_ALPHANUMERIC);
    let url = format!("http://{address}/v4/decodetrack?encodedTrack={identifier}");

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
/// # Errors
///
/// See the documentation for [`http::Error`].
pub fn get_route_planner(
    address: SocketAddr,
    authorization: impl AsRef<str>,
) -> Result<Request<&'static [u8]>, HttpError> {
    let mut req = Request::get(format!("{address}/v4/routeplanner/status"));

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(b"")
}

/// Unmark an IP address as being failed, meaning that it can be used again.
///
/// The response will not include a body on success.
///
/// # Errors
///
/// See the documentation for [`http::Error`].
#[allow(clippy::missing_panics_doc)]
pub fn unmark_failed_address(
    node_address: impl Into<SocketAddr>,
    authorization: impl AsRef<str>,
    route_address: impl Into<IpAddr>,
) -> Result<Request<Vec<u8>>, HttpError> {
    let mut req = Request::post(format!("{}/v4/routeplanner/status", node_address.into()));

    let auth_value = HeaderValue::from_str(authorization.as_ref())?;
    req = req.header(AUTHORIZATION, auth_value);

    req.body(
        serde_json::to_vec(&serde_json::json!({
            "address": route_address.into(),
        }))
        .expect("valid json"),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        FailingAddress, IpBlock, IpBlockType, LoadedTracks, NanoIpDetails, NanoIpRoutePlanner,
        PlaylistInfo, RotatingIpDetails, RotatingIpRoutePlanner, RotatingNanoIpDetails,
        RotatingNanoIpRoutePlanner, RoutePlanner, RoutePlannerType, Track,
    };
    use crate::model::incoming::TrackInfo;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(FailingAddress: address, failing_timestamp, failing_time);
    assert_impl_all!(
        FailingAddress: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(
        IpBlockType: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(IpBlock: kind, size);
    assert_impl_all!(
        IpBlock: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(
        LoadedTracks: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(
        NanoIpDetails: current_address_index,
        failing_addresses,
        ip_block
    );
    assert_impl_all!(
        NanoIpDetails: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(NanoIpRoutePlanner: class, details);
    assert_impl_all!(
        NanoIpRoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(PlaylistInfo: name, selected_track);
    assert_impl_all!(
        PlaylistInfo: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(
        RotatingIpDetails: current_address,
        failing_addresses,
        ip_block,
        ip_index,
        rotate_index
    );
    assert_impl_all!(
        RotatingIpDetails: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(RotatingIpRoutePlanner: class, details);
    assert_impl_all!(
        RotatingIpRoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(
        RotatingNanoIpDetails: block_index,
        current_address_index,
        failing_addresses,
        ip_block
    );
    assert_impl_all!(
        RotatingNanoIpDetails: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(RotatingNanoIpRoutePlanner: class, details);
    assert_impl_all!(
        RotatingNanoIpRoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(
        RoutePlannerType: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(
        RoutePlanner: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(
        TrackInfo: author,
        identifier,
        is_seekable,
        is_stream,
        length,
        position,
        title,
        uri
    );
    assert_impl_all!(
        TrackInfo: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_fields!(Track: encoded, info);
    assert_impl_all!(
        Track: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    pub fn test_deserialize_playlist_info_negative_selected_track() {
        let value = PlaylistInfo {
            name: "Test Playlist".to_owned(),
            selected_track: None,
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PlaylistInfo",
                    len: 13,
                },
                Token::Str("name"),
                Token::Str("Test Playlist"),
                Token::Str("selectedTrack"),
                Token::I64(-1),
                Token::StructEnd,
            ],
        );
    }
}
