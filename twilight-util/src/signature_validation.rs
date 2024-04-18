//! Provides signature validation as is required for bots which work by giving Discord
//! an HTTPS endpoint to send Interactions to.
//!
//! See <https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization>
//! for more details.

use ed25519_dalek::{Signature, SignatureError, VerifyingKey};
#[cfg(feature = "signature-validation-extract-interaction")]
use twilight_model::application::interaction::Interaction;

/// Parsing a hexadecimal string failed.
#[derive(Debug)]
pub struct FromHexError(hex::FromHexError);

impl std::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for FromHexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

/// A signature or public key was invalid.
#[derive(Debug)]
pub struct SigError(SignatureError);

impl std::fmt::Display for SigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

/// Signature validation failed. If you successfully gave your program
/// the public key provided by Discord, this is almost definitely because
/// you received an invalid request.
#[derive(Debug)]
pub enum SignatureValidationFailure {
    /// The request signature was invalid hexadecimal.
    Hex(FromHexError),
    /// Request had invalid signature for the given public key.
    InvalidSignature(SigError),
}

impl std::fmt::Display for SignatureValidationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SignatureValidationFailure {}

/// Parsing the public key failed.
#[derive(Debug)]
pub enum KeyError {
    /// The public key was invalid hexadecimal.
    Hex(FromHexError),
    /// The public key was malformed.
    MalformedKey(SignatureError),
}

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for KeyError {}

/// The key you are meant to get from the Discord Developer Portal,
/// on your Application. It is currently listed on the General Information page,
/// labeled "Public Key", at the time of this writing (January 19th, 2024).
pub struct Key(VerifyingKey);
impl Key {
    /// This function consumes the hexadecimal string which Discord
    /// provides public keys in. Use `.as_bytes()` on a `&str`, or otherwise
    /// obtain a byte-string of that text, to use with this function.
    ///
    /// # Errors
    /// This will fail if given invalid hexadecimal, or if the public key fails to
    /// meet mathematical requirements.
    pub fn from_hex(pub_key: &[u8]) -> Result<Self, KeyError> {
        let mut key = [0; 32];
        hex::decode_to_slice(pub_key, &mut key).map_err(|e| KeyError::Hex(FromHexError(e)))?;
        VerifyingKey::from_bytes(&key)
            .map(Self)
            .map_err(KeyError::MalformedKey)
    }
    /// Validate a signature for a given message body, timestamp, and signing key.
    ///
    /// (This method is a duplicate of [`check_signature`].)
    pub fn verify(
        &self,
        signature: &[u8],
        timestamp: &[u8],
        body: &[u8],
    ) -> Result<(), SignatureValidationFailure> {
        check_signature(signature, timestamp, body, self)
    }
}

/// The name of the HTTP header Discord wants us to read for the signature.
pub const SIGNATURE_HEADER: &str = "x-signature-ed25519";
/// The name of the HTTP header Discord wants us to read for the signature timestamp.
pub const TIMESTAMP_HEADER: &str = "x-signature-timestamp";

/// Validates that a signature is valid for a given message body, timestamp, and signing key.
///
/// # Errors
/// This will fail if the request being validated has an invalid signature, or if it
/// was given the wrong key.
pub fn check_signature(
    signature: &[u8],
    timestamp: &[u8],
    body: &[u8],
    key: &Key,
) -> Result<(), SignatureValidationFailure> {
    let mut sig_buf = [0; 64];
    hex::decode_to_slice(signature, &mut sig_buf)
        .map_err(|e| SignatureValidationFailure::Hex(FromHexError(e)))?;
    let sig = Signature::from_bytes(&sig_buf);

    let mut buf = Vec::with_capacity(timestamp.len() + body.len());
    buf.extend_from_slice(timestamp);
    buf.extend_from_slice(body);
    match key.0.verify_strict(&buf, &sig) {
        Ok(()) => Ok(()),
        Err(e) => Err(SignatureValidationFailure::InvalidSignature(SigError(e))),
    }
}

/// Extracting the body of an Interaction failed.
#[cfg(feature = "signature-validation-extract-interaction")]
pub enum ExtractFailure {
    /// The failure was due to the Interaction having an invalid signature.
    Signature(SignatureValidationFailure),
    /// The failure was due to the Interaction being incorrect or invalid JSON.
    Deserialize(serde_json::Error),
}

#[cfg(feature = "signature-validation-extract-interaction")]
impl From<SignatureValidationFailure> for ExtractFailure {
    fn from(value: SignatureValidationFailure) -> Self {
        Self::Signature(value)
    }
}

#[cfg(feature = "signature-validation-extract-interaction")]
impl From<serde_json::Error> for ExtractFailure {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialize(value)
    }
}

/// Validate an Interaction's signature, and deserialize it from JSON.
///
/// # Errors
/// This will fail in the following cases:
/// - The request has an invalid signature
/// - The wrong key was given
/// - Deserialization of the Interaction fails
#[cfg(feature = "signature-validation-extract-interaction")]
pub fn extract_interaction(
    signature: &[u8],
    timestamp: &[u8],
    body: &[u8],
    key: &Key,
) -> Result<Interaction, ExtractFailure> {
    check_signature(signature, timestamp, body, key)?;
    Ok(serde_json::from_slice(body)?)
}
