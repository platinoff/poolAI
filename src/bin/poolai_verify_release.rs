//! Verify signed PoolAI release manifest + optional artifact SHA-256 (Galaxy §9.2, PH-S66).
//!
//! ```text
//! cargo run --bin poolai-verify-release -- \
//!   --manifest release.json \
//!   --signature release.json.sig \
//!   --trust-root maintainer_keys.json \
//!   --artifact ./poolai.exe \
//!   --artifact-name poolai
//! ```
//!
//! Exit `0` on success, `1` on verification failure, `2` on usage/IO errors.

use poolai::release::{verify_release, VerifyReleaseOptions};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn usage() -> &'static str {
    "poolai-verify-release — verify signed release manifest (Galaxy §9.2)\n\
\n\
Usage:\n\
  poolai-verify-release verify-release [options]\n\
  poolai-verify-release [options]\n\
\n\
Options:\n\
  --manifest PATH          Signed release manifest JSON (required)\n\
  --signature PATH         Detached signature envelope JSON (required)\n\
  --trust-root PATH        maintainer_keys.json (key_id → public_key_hex)\n\
  --public-key-hex HEX     Ed25519 public key (32 bytes hex); overrides trust-root lookup\n\
  --artifact PATH          Verify SHA-256 against manifest entry\n\
  --artifact-name NAME     Manifest artifact name (default: file name)\n\
  --json                   Print JSON report on success\n\
  -h, --help               Show help\n\
\n\
See docs/concept/POOLAI_GALAXY_GRID.md §9.2 and docs/security/SECURITY_HARDENING.md\n"
}

fn print_help() {
    eprintln!("{}", usage());
}

fn parse_args(args: &[String]) -> Result<VerifyReleaseOptions, String> {
    let mut i = 0usize;
    if !args.is_empty() && (args[0] == "verify-release" || args[0] == "verify_release") {
        i = 1;
    }

    let mut manifest_path: Option<PathBuf> = None;
    let mut signature_path: Option<PathBuf> = None;
    let mut trust_root_path: Option<PathBuf> = None;
    let mut public_key_hex: Option<String> = None;
    let mut artifact_path: Option<PathBuf> = None;
    let mut artifact_name: Option<String> = None;

    while i < args.len() {
        let arg = args[i].as_str();
        i += 1;
        match arg {
            "-h" | "--help" => return Err("help".into()),
            "--manifest" => {
                manifest_path = Some(PathBuf::from(next_value(arg, args, &mut i)?));
            }
            "--signature" => {
                signature_path = Some(PathBuf::from(next_value(arg, args, &mut i)?));
            }
            "--trust-root" => {
                trust_root_path = Some(PathBuf::from(next_value(arg, args, &mut i)?));
            }
            "--public-key-hex" => {
                public_key_hex = Some(next_value(arg, args, &mut i)?);
            }
            "--artifact" => {
                artifact_path = Some(PathBuf::from(next_value(arg, args, &mut i)?));
            }
            "--artifact-name" => {
                artifact_name = Some(next_value(arg, args, &mut i)?);
            }
            "--json" => {}
            other => return Err(format!("unknown flag: {other}")),
        }
    }

    let manifest_path = manifest_path.ok_or("missing --manifest")?;
    let signature_path = signature_path.ok_or("missing --signature")?;

    Ok(VerifyReleaseOptions {
        manifest_path,
        signature_path,
        trust_root_path,
        public_key_hex,
        artifact_path,
        artifact_name,
    })
}

fn next_value(flag: &str, args: &[String], i: &mut usize) -> Result<String, String> {
    if *i >= args.len() {
        return Err(format!("missing value for {flag}"));
    }
    let v = args[*i].clone();
    *i += 1;
    Ok(v)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return ExitCode::from(2);
    }

    let json_out = args.iter().any(|a| a == "--json");
    let opts = match parse_args(&args) {
        Ok(o) => o,
        Err(e) if e == "help" => {
            print_help();
            return ExitCode::from(2);
        }
        Err(e) => {
            eprintln!("error: {e}");
            print_help();
            return ExitCode::from(2);
        }
    };

    match verify_release(opts) {
        Ok(report) => {
            if json_out {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "ok": true,
                        "manifest_version": report.manifest_version,
                        "git_tag": report.git_tag,
                        "protocol_min": report.protocol_min,
                        "protocol_max": report.protocol_max,
                        "signature_key_id": report.signature_key_id,
                        "artifacts_verified": report.artifacts_verified,
                    }))
                    .unwrap_or_else(|_| "{}".into())
                );
            } else {
                println!("release verify: OK");
                println!("  version: {}", report.manifest_version);
                if let Some(tag) = &report.git_tag {
                    println!("  git_tag: {tag}");
                }
                if let Some(pmin) = &report.protocol_min {
                    println!("  protocol_min: {pmin}");
                }
                if let Some(pmax) = &report.protocol_max {
                    println!("  protocol_max: {pmax}");
                }
                println!("  signature_key_id: {}", report.signature_key_id);
                if !report.artifacts_verified.is_empty() {
                    println!(
                        "  artifacts_verified: {}",
                        report.artifacts_verified.join(", ")
                    );
                }
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("release verify: FAIL — {e}");
            ExitCode::from(1)
        }
    }
}
