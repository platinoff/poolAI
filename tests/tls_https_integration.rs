//! PH-S08 / FM-044: TLS 1.3 rustls policy and certificate reload.

use poolai::network::tls_config::{CertificatePaths, TlsConfig, TlsServeContext, TlsVersion};
use std::sync::Arc;

#[tokio::test]
async fn tls_serve_context_reload_roundtrip() {
    let paths = CertificatePaths {
        cert: "certs/cert.pem".into(),
        key: "certs/key.pem".into(),
    };
    if !paths.cert_path().exists() {
        return;
    }

    let policy = TlsConfig::default();
    let ctx = TlsServeContext::from_pem_files(paths, policy)
        .await
        .expect("load dev PEM");

    let before = Arc::as_ptr(&ctx.rustls.get_inner());
    ctx.reload_certificates()
        .await
        .expect("reload same PEM files");
    let after = Arc::as_ptr(&ctx.rustls.get_inner());
    assert_ne!(before, after, "reload should install a new ServerConfig");
}

#[test]
fn tls12_backward_compat_policy_versions() {
    let policy = TlsConfig::new(TlsVersion::Tls1_2, TlsVersion::Tls1_3, true);
    assert_eq!(policy.min_version, TlsVersion::Tls1_2);
    assert_eq!(policy.max_version, TlsVersion::Tls1_3);
}
