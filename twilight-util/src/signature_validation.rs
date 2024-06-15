//! Provides signature validation as is required for bots which work by giving Discord
//! an HTTPS endpoint to send Interactions to.
//!
//! See <https://discord.com/developers/docs/interactions/overview#preparing-for-interactions>
//! for more details.

use ed25519_dalek::{Signature as DalekSignature, VerifyingKey as DalekVerifyingKey};
#[cfg(feature = "signature-validation-extract-interaction")]
use twilight_model::application::interaction::Interaction;

/// The name of the HTTP header Discord wants us to read for the signature.
pub const SIGNATURE_HEADER: &str = "x-signature-ed25519";
/// The name of the HTTP header Discord wants us to read for the signature timestamp.
pub const TIMESTAMP_HEADER: &str = "x-signature-timestamp";

/// The key you are meant to get from the Discord Developer Portal,
/// on your Application. It is currently listed on the General Information page,
/// labeled "Public Key", at the time of this writing (January 19th, 2024).
pub struct Key {
    inner: DalekVerifyingKey,
}

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
        hex::decode_to_slice(pub_key, &mut key).map_err(|e| KeyError {
            kind: KeyErrorKind::Hex,
            source: Some(e.into()),
        })?;
        DalekVerifyingKey::from_bytes(&key)
            .map(|inner| Key { inner })
            .map_err(|err| KeyError {
                kind: KeyErrorKind::MalformedKey,
                source: Some(err.into()),
            })
    }
    /// Verify a signature for a given message body, timestamp, and signing key.
    ///
    /// (This method is a duplicate of [`check_signature`].)
    ///
    /// # Errors
    /// This will fail if the request being verified was given the wrong key.
    pub fn verify(
        &self,
        signature: &Signature,
        timestamp: &[u8],
        body: &[u8],
    ) -> Result<(), SignatureValidationFailure> {
        check_signature(signature, timestamp, body, self)
    }
}

/// Signature extracted from the header of the incoming request.
///
/// The specific header can be found in [`SIGNATURE_HEADER`].
pub struct Signature {
    inner: DalekSignature,
}

impl Signature {
    /// Create a signature from a slice.
    ///
    /// # Errors
    /// This will fail if the hex slice is invalid.
    pub fn from_slice(signature: &[u8]) -> Result<Signature, SignatureValidationFailure> {
        let mut sig_buf = [0; 64];
        hex::decode_to_slice(signature, &mut sig_buf).map_err(|e| SignatureValidationFailure {
            kind: SignatureValidationFailureKind::Hex,
            source: Some(e.into()),
        })?;
        let sig = DalekSignature::from_bytes(&sig_buf);
        Ok(Signature { inner: sig })
    }
}

/// Validates that a signature is valid for a given message body, timestamp, and signing key.
///
/// # Errors
/// This will fail if the request being validated has the wrong key.
pub fn check_signature(
    signature: &Signature,
    timestamp: &[u8],
    body: &[u8],
    key: &Key,
) -> Result<(), SignatureValidationFailure> {
    let mut buf = Vec::with_capacity(timestamp.len() + body.len());
    buf.extend_from_slice(timestamp);
    buf.extend_from_slice(body);
    match key.inner.verify_strict(&buf, &signature.inner) {
        Ok(()) => Ok(()),
        Err(e) => Err(SignatureValidationFailure {
            kind: SignatureValidationFailureKind::InvalidSignature,
            source: Some(e.into()),
        }),
    }
}

/// Signature validation failed. If you successfully gave your program
/// the public key provided by Discord, this is almost definitely because
/// you received an invalid request.
#[derive(Debug)]
pub struct SignatureValidationFailure {
    kind: SignatureValidationFailureKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

/// The kind of [`SignatureValidationFailure`] that occurred.
#[derive(Debug)]
pub enum SignatureValidationFailureKind {
    /// The request signature was invalid hexadecimal.
    Hex, //(KeyParseError),
    /// Request had invalid signature for the given public key.
    InvalidSignature, //(SigError),
}

impl SignatureValidationFailure {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &SignatureValidationFailureKind {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        SignatureValidationFailureKind,
        Option<Box<dyn std::error::Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl std::fmt::Display for SignatureValidationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            SignatureValidationFailureKind::Hex => f.write_str("signature hex is invalid"),
            SignatureValidationFailureKind::InvalidSignature => f.write_str("signature is invalid"),
        }
    }
}

impl std::error::Error for SignatureValidationFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn std::error::Error + 'static))
    }
}

/// Error occurring when the key cannot be parsed.
#[derive(Debug)]
pub struct KeyError {
    kind: KeyErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

/// Type of [`KeyError`] that occurred.
#[derive(Debug)]
pub enum KeyErrorKind {
    /// The public key was invalid hexadecimal.
    Hex,
    /// The public key was malformed.
    MalformedKey,
}

impl KeyError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &KeyErrorKind {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        KeyErrorKind,
        Option<Box<dyn std::error::Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            KeyErrorKind::Hex => f.write_str("could not parse hex string"),
            KeyErrorKind::MalformedKey => f.write_str("key was malformed"),
        }
    }
}

impl std::error::Error for KeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn std::error::Error + 'static))
    }
}
