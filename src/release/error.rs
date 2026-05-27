use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerifyReleaseError {
    #[error("manifest not found: {0}")]
    ManifestNotFound(PathBuf),
    #[error("signature not found: {0}")]
    SignatureNotFound(PathBuf),
    #[error("failed to read {path}: {source}")]
    IoRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid manifest JSON: {0}")]
    InvalidManifestJson(String),
    #[error("invalid signature file JSON: {0}")]
    InvalidSignatureJson(String),
    #[error("unsupported signature algorithm: {0} (expected ed25519)")]
    UnsupportedAlgorithm(String),
    #[error("trust root not found: {0}")]
    TrustRootNotFound(PathBuf),
    #[error("invalid trust root JSON: {0}")]
    InvalidTrustRootJson(String),
    #[error("public key not found for key_id={0}")]
    UnknownKeyId(String),
    #[error("no public key: pass --public-key-hex or --trust-root")]
    MissingPublicKey,
    #[error("invalid public key hex: {0}")]
    InvalidPublicKeyHex(String),
    #[error("invalid signature hex: {0}")]
    InvalidSignatureHex(String),
    #[error("manifest signature verification failed")]
    BadSignature,
    #[error("artifact not found: {0}")]
    ArtifactNotFound(PathBuf),
    #[error("artifact {name} sha256 mismatch (expected {expected}, got {actual})")]
    ArtifactSha256Mismatch {
        name: String,
        expected: String,
        actual: String,
    },
    #[error("artifact {0} not listed in manifest")]
    ArtifactNotInManifest(String),
}
