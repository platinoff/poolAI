use ed25519_dalek::{Signature, VerifyingKey};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use super::error::VerifyReleaseError;
use super::manifest::ReleaseManifest;
use super::trust::TrustRoot;

#[derive(Debug, Clone)]
pub struct VerifyReleaseOptions {
    pub manifest_path: PathBuf,
    pub signature_path: PathBuf,
    pub trust_root_path: Option<PathBuf>,
    pub public_key_hex: Option<String>,
    pub artifact_path: Option<PathBuf>,
    pub artifact_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyReleaseReport {
    pub manifest_version: String,
    pub git_tag: Option<String>,
    pub protocol_min: Option<String>,
    pub protocol_max: Option<String>,
    pub signature_key_id: String,
    pub artifacts_verified: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SignatureEnvelope {
    algorithm: String,
    key_id: String,
    signature_hex: String,
}

pub fn verify_release(
    opts: VerifyReleaseOptions,
) -> Result<VerifyReleaseReport, VerifyReleaseError> {
    let manifest_path = &opts.manifest_path;
    if !manifest_path.exists() {
        return Err(VerifyReleaseError::ManifestNotFound(manifest_path.clone()));
    }
    let sig_path = &opts.signature_path;
    if !sig_path.exists() {
        return Err(VerifyReleaseError::SignatureNotFound(sig_path.clone()));
    }

    let manifest_bytes = read_file(manifest_path)?;
    let sig_bytes = read_file(sig_path)?;
    let envelope: SignatureEnvelope = serde_json::from_slice(&sig_bytes)
        .map_err(|e| VerifyReleaseError::InvalidSignatureJson(e.to_string()))?;

    if envelope.algorithm != "ed25519" {
        return Err(VerifyReleaseError::UnsupportedAlgorithm(
            envelope.algorithm.clone(),
        ));
    }

    let pk_hex = resolve_public_key_hex(
        &envelope.key_id,
        opts.trust_root_path.as_deref(),
        opts.public_key_hex.as_deref(),
    )?;
    verify_ed25519(&manifest_bytes, &envelope.signature_hex, &pk_hex)?;

    let manifest = ReleaseManifest::parse_json(&manifest_bytes)
        .map_err(|e| VerifyReleaseError::InvalidManifestJson(e.to_string()))?;

    let mut artifacts_verified = Vec::new();
    if let Some(artifact_path) = &opts.artifact_path {
        let name = opts
            .artifact_name
            .clone()
            .or_else(|| {
                artifact_path
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "artifact".to_string());
        verify_artifact_sha256(&manifest, &name, artifact_path)?;
        artifacts_verified.push(name);
    }

    Ok(VerifyReleaseReport {
        manifest_version: manifest.version,
        git_tag: manifest.git_tag,
        protocol_min: manifest.protocol_min,
        protocol_max: manifest.protocol_max,
        signature_key_id: envelope.key_id,
        artifacts_verified,
    })
}

fn read_file(path: &Path) -> Result<Vec<u8>, VerifyReleaseError> {
    std::fs::read(path).map_err(|e| VerifyReleaseError::IoRead {
        path: path.to_path_buf(),
        source: e,
    })
}

fn resolve_public_key_hex(
    key_id: &str,
    trust_root_path: Option<&Path>,
    public_key_hex: Option<&str>,
) -> Result<String, VerifyReleaseError> {
    if let Some(hex) = public_key_hex {
        return Ok(hex.trim().to_string());
    }
    let Some(path) = trust_root_path else {
        return Err(VerifyReleaseError::MissingPublicKey);
    };
    let root = TrustRoot::load(path)?;
    let map = root.key_map();
    map.get(key_id)
        .map(|h| (*h).to_string())
        .ok_or_else(|| VerifyReleaseError::UnknownKeyId(key_id.to_string()))
}

fn verify_ed25519(
    message: &[u8],
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<(), VerifyReleaseError> {
    let pk_bytes = hex::decode(public_key_hex.trim())
        .map_err(|e| VerifyReleaseError::InvalidPublicKeyHex(e.to_string()))?;
    let vk = VerifyingKey::from_bytes(
        pk_bytes
            .as_slice()
            .try_into()
            .map_err(|_| VerifyReleaseError::InvalidPublicKeyHex("expected 32 bytes".into()))?,
    )
    .map_err(|e| VerifyReleaseError::InvalidPublicKeyHex(e.to_string()))?;

    let sig_bytes = hex::decode(signature_hex.trim())
        .map_err(|e| VerifyReleaseError::InvalidSignatureHex(e.to_string()))?;
    let signature = Signature::from_bytes(
        sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| VerifyReleaseError::InvalidSignatureHex("expected 64 bytes".into()))?,
    );

    vk.verify_strict(message, &signature)
        .map_err(|_| VerifyReleaseError::BadSignature)
}

fn verify_artifact_sha256(
    manifest: &ReleaseManifest,
    name: &str,
    artifact_path: &Path,
) -> Result<(), VerifyReleaseError> {
    if !artifact_path.exists() {
        return Err(VerifyReleaseError::ArtifactNotFound(
            artifact_path.to_path_buf(),
        ));
    }
    let entry = manifest
        .find_artifact(name)
        .ok_or_else(|| VerifyReleaseError::ArtifactNotInManifest(name.to_string()))?;
    let bytes = read_file(artifact_path)?;
    let digest = Sha256::digest(&bytes);
    let actual = hex::encode(digest);
    let expected = entry.sha256.trim().to_ascii_lowercase();
    if actual != expected {
        return Err(VerifyReleaseError::ArtifactSha256Mismatch {
            name: name.to_string(),
            expected,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signer;
    use ed25519_dalek::SigningKey;

    fn test_signing_key() -> SigningKey {
        let bytes = hex::decode("9d61b19deffd5a60ba844af492ec2ccaa8484dd16824b92852c9392b506ec0e5")
            .unwrap();
        SigningKey::from_bytes(bytes.as_slice().try_into().unwrap())
    }

    fn test_public_hex() -> String {
        hex::encode(test_signing_key().verifying_key().to_bytes())
    }

    fn write_signature(dir: &tempfile::TempDir, manifest_bytes: &[u8], key_id: &str) -> PathBuf {
        let sig = test_signing_key().sign(manifest_bytes);
        let envelope = serde_json::json!({
            "algorithm": "ed25519",
            "key_id": key_id,
            "signature_hex": hex::encode(sig.to_bytes()),
        });
        let path = dir.path().join("manifest.json.sig");
        std::fs::write(&path, serde_json::to_vec_pretty(&envelope).unwrap()).unwrap();
        path
    }

    fn sample_manifest_json() -> Vec<u8> {
        br#"{
  "version": "0.2.2",
  "git_tag": "v0.2.2",
  "protocol_min": "1.0",
  "protocol_max": "1.2",
  "artifacts": [
    { "name": "poolai", "path": "poolai.exe", "sha256": "PLACEHOLDER" }
  ]
}"#
        .to_vec()
    }

    #[test]
    fn verify_release_ok() {
        let dir = tempfile::tempdir().unwrap();
        let artifact_path = dir.path().join("poolai.exe");
        std::fs::write(&artifact_path, b"poolai-test-artifact-v1").unwrap();
        let hash = hex::encode(Sha256::digest(b"poolai-test-artifact-v1"));

        let mut manifest = sample_manifest_json();
        let manifest_str = String::from_utf8(manifest.clone()).unwrap();
        let manifest_str = manifest_str.replace("PLACEHOLDER", &hash);
        manifest = manifest_str.into_bytes();

        let manifest_path = dir.path().join("manifest.json");
        std::fs::write(&manifest_path, &manifest).unwrap();

        let sig_path = write_signature(&dir, &manifest, "poolai-dev");
        let trust_path = dir.path().join("trust_root.json");
        std::fs::write(
            &trust_path,
            format!(
                r#"{{"maintainer_keys":[{{"key_id":"poolai-dev","public_key_hex":"{}"}}]}}"#,
                test_public_hex()
            ),
        )
        .unwrap();

        let report = verify_release(VerifyReleaseOptions {
            manifest_path: manifest_path.clone(),
            signature_path: sig_path,
            trust_root_path: Some(trust_path),
            public_key_hex: None,
            artifact_path: Some(artifact_path),
            artifact_name: Some("poolai".into()),
        })
        .unwrap();

        assert_eq!(report.manifest_version, "0.2.2");
        assert_eq!(report.artifacts_verified, vec!["poolai"]);
    }

    #[test]
    fn verify_release_bad_signature() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = sample_manifest_json();
        let manifest_path = dir.path().join("manifest.json");
        std::fs::write(&manifest_path, &manifest).unwrap();
        let sig_path = write_signature(&dir, b"tampered manifest bytes", "poolai-dev");
        let trust_path = dir.path().join("trust_root.json");
        std::fs::write(
            &trust_path,
            format!(
                r#"{{"maintainer_keys":[{{"key_id":"poolai-dev","public_key_hex":"{}"}}]}}"#,
                test_public_hex()
            ),
        )
        .unwrap();

        let err = verify_release(VerifyReleaseOptions {
            manifest_path,
            signature_path: sig_path,
            trust_root_path: Some(trust_path),
            public_key_hex: None,
            artifact_path: None,
            artifact_name: None,
        })
        .unwrap_err();
        assert!(matches!(err, VerifyReleaseError::BadSignature));
    }

    /// Regenerate `tests/fixtures/release/dev/` (PH-S85). Run:
    /// `cargo test --lib release::verify::tests::write_dev_release_fixtures -- --ignored --exact`
    #[test]
    #[ignore]
    fn write_dev_release_fixtures() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/release/dev");
        std::fs::create_dir_all(&root).unwrap();

        let artifact_path = root.join("poolai-sample.bin");
        std::fs::write(&artifact_path, b"poolai-test-artifact-v1").unwrap();
        let hash = hex::encode(Sha256::digest(b"poolai-test-artifact-v1"));

        let manifest = format!(
            r#"{{
  "version": "0.2.2-dev",
  "git_tag": "v0.2.2-dev-fixture",
  "protocol_min": "1.0",
  "protocol_max": "1.2",
  "artifacts": [
    {{ "name": "poolai", "path": "poolai-sample.bin", "sha256": "{hash}" }}
  ]
}}
"#
        );
        let manifest_path = root.join("release-manifest.json");
        std::fs::write(&manifest_path, &manifest).unwrap();

        let sig = test_signing_key().sign(manifest.as_bytes());
        let envelope = serde_json::json!({
            "algorithm": "ed25519",
            "key_id": "poolai-dev",
            "signature_hex": hex::encode(sig.to_bytes()),
        });
        std::fs::write(
            root.join("release-manifest.json.sig"),
            serde_json::to_vec_pretty(&envelope).unwrap(),
        )
        .unwrap();

        std::fs::write(
            root.join("maintainer_keys.json"),
            format!(
                r#"{{"maintainer_keys":[{{"key_id":"poolai-dev","public_key_hex":"{}"}}]}}"#,
                test_public_hex()
            ),
        )
        .unwrap();
    }

    #[test]
    fn verify_release_artifact_hash_mismatch() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = sample_manifest_json();
        let manifest_path = dir.path().join("manifest.json");
        std::fs::write(&manifest_path, &manifest).unwrap();
        let sig_path = write_signature(&dir, &manifest, "poolai-dev");
        let artifact_path = dir.path().join("poolai.exe");
        std::fs::write(&artifact_path, b"wrong-bytes").unwrap();

        let err = verify_release(VerifyReleaseOptions {
            manifest_path,
            signature_path: sig_path,
            trust_root_path: None,
            public_key_hex: Some(test_public_hex()),
            artifact_path: Some(artifact_path),
            artifact_name: Some("poolai".into()),
        })
        .unwrap_err();
        assert!(matches!(
            err,
            VerifyReleaseError::ArtifactSha256Mismatch { .. }
        ));
    }
}
