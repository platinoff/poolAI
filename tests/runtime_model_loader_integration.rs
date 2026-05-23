//! FM-035 integration: real weight load path for runtime model_loader.

use poolai::core::model_interface::{ModelParameters, ModelRequest};
use poolai::libs::LibraryInfo;
use poolai::runtime::model_loader::{load_from_library, scan_and_load, ModelBackendKind};
use std::io::Write;

#[tokio::test]
async fn onnx_library_loads_and_infers() {
    let tmp = tempfile::tempdir().unwrap();
    let weights = tmp.path().join("model.onnx");
    let mut f = std::fs::File::create(&weights).unwrap();
    f.write_all(&[
        0x08, 0x03, 0x12, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
        0x0C,
    ])
    .unwrap();

    let lib = LibraryInfo {
        name: "fm035-onnx".to_string(),
        version: "1.0.0".to_string(),
        path: tmp.path().to_path_buf(),
        dependencies: vec![],
        metadata: Default::default(),
        artifact_ref: None,
    };

    let handle = load_from_library("fm035-onnx", &lib).await.unwrap();
    assert_eq!(handle.report.backend, ModelBackendKind::Onnx);
    assert!(handle.report.bytes_loaded > 0);

    let resp = handle
        .model
        .process_request(ModelRequest {
            input: "poolai fm035".to_string(),
            parameters: ModelParameters::default(),
            session_id: None,
            priority: 5,
            timeout: Some(30),
        })
        .await
        .unwrap();

    assert!(resp.output.contains("[onnx inference"));
    assert!(!resp.output.contains("would process"));
}

#[tokio::test]
async fn scan_detects_libtorch_by_extension() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(
        tmp.path().join("weights.pth"),
        b"PK\x03\x04mock-torch-weights-payload",
    )
    .unwrap();

    let report = scan_and_load("custom-torch", tmp.path()).await.unwrap();
    assert_eq!(report.backend, ModelBackendKind::LibTorch);
}
