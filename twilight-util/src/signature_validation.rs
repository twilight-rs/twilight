//! Provides signature validation as is required for bots which work by giving Discord
//! an HTTPS endpoint to send Interactions to.

use ed25519_dalek::{Signature, SignatureError, VerifyingKey};

#[derive(Debug)]
pub struct FromHexError(hex::FromHexError);
#[derive(Debug)]
pub struct SigError(SignatureError);

#[derive(Debug)]
pub enum SignatureValidationFailure {
    Hex(FromHexError),
    InvalidSignature(SigError),
}
impl std::fmt::Display for SignatureValidationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for SignatureValidationFailure {}

#[derive(Debug)]
pub enum KeyError {
    Hex(FromHexError),
    Sig(SignatureError),
}
impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for KeyError {}

pub struct Key(VerifyingKey);
impl Key {
    fn from_hex(pub_key: &[u8]) -> Result<Self, KeyError> {
        let mut key = [0; 32];
        hex::decode_to_slice(pub_key, &mut key).map_err(|e| KeyError::Hex(FromHexError(e)))?;
        VerifyingKey::from_bytes(&key)
            .map(|key| Self(key))
            .map_err(|e| KeyError::Sig(e))
    }
}

pub const SIGNATURE_HEADER: &str = "x-signature-ed25519";
pub const TIMESTAMP_HEADER: &str = "x-signature-timestamp";

/// Validates that a signature is valid for a given message body, timestamp, and signing key.
pub fn check_signature(
    sig: &[u8],
    timestamp: &[u8],
    body: &[u8],
    key: &Key,
) -> Result<(), SignatureValidationFailure> {
    let mut sig_buf = [0; 64];
    hex::decode_to_slice(sig, &mut sig_buf)
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
