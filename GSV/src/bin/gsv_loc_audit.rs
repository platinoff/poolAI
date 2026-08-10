//! GSV LOC/ratio audit bin — writes `GSV/data/rust_ratio.json` (canon: Rust 95–100%).
//!
//! ```text
//! cargo run --bin gsv-loc-audit
//! cargo run --bin gsv-loc-audit -- --print
//! cargo run --bin gsv-loc-audit -- --min-ratio 0.95 --advisory
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use gsv::boxes::ratio::{self, AuditConfig};

fn parse_args() -> (AuditConfig, PathBuf) {
    let mut config = AuditConfig::default();
    let mut data_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--print" => {
                config.print = true;
                config.write_output = false;
            }
            "--no-write" => config.write_output = false,
            "--advisory" => config.advisory = true,
            "--stretch-96" => config.stretch_96 = true,
            "--min-ratio" => {
                config.min_ratio = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(ratio::DEFAULT_MIN_RATIO)
            }
            "--output" => {
                config.write_output = true;
                config.output = args.next().map(PathBuf::from);
            }
            "--data-dir" => data_dir = args.next().map(PathBuf::from),
            "--help" | "-h" => {
                println!(
                    "Usage: gsv-loc-audit [--print] [--no-write] [--advisory] [--stretch-96] [--min-ratio 0.95] [--output PATH] [--data-dir PATH]"
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }
    let data = data_dir.unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data"));
    (config, data)
}

fn main() -> ExitCode {
    let (config, data_dir) = parse_args();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    match ratio::audit(&root) {
        Ok(report) => {
            let ratio_ok = report.rust_ratio >= config.min_ratio;
            if config.print {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).unwrap_or_default()
                );
            }
            if config.write_output {
                let output = config
                    .output
                    .unwrap_or_else(|| data_dir.join("rust_ratio.json"));
                if let Err(e) = std::fs::create_dir_all(output.parent().unwrap_or(&data_dir)) {
                    eprintln!("gsv-loc-audit: create dir: {e}");
                    return ExitCode::FAILURE;
                }
                if let Err(e) = std::fs::write(
                    &output,
                    serde_json::to_string_pretty(&report).unwrap_or_default(),
                ) {
                    eprintln!("gsv-loc-audit: write {}: {e}", output.display());
                    return ExitCode::FAILURE;
                }
                println!(
                    "rust_ratio {:.2}% (rust {} / product {}) -> {}",
                    report.rust_ratio_pct,
                    report.rust_loc,
                    report.product_loc_total,
                    output.display()
                );
            } else {
                println!(
                    "rust_ratio {:.2}% (rust {} / product {})",
                    report.rust_ratio_pct, report.rust_loc, report.product_loc_total
                );
            }
            if config.stretch_96 {
                if report.meets_stretch_96 {
                    println!(
                        "stretch-96: meets >= {:.2}%",
                        ratio::STRETCH_96_TARGET * 100.0
                    );
                    ExitCode::SUCCESS
                } else {
                    println!(
                        "stretch-96 advisory: rust_ratio {:.2}% below {:.2}%",
                        report.rust_ratio_pct,
                        ratio::STRETCH_96_TARGET * 100.0
                    );
                    ExitCode::SUCCESS
                }
            } else if !ratio_ok {
                let msg = format!(
                    "rust_ratio {:.2}% below min {:.2}%",
                    report.rust_ratio_pct,
                    config.min_ratio * 100.0
                );
                if config.advisory {
                    println!("advisory: {msg}");
                    ExitCode::SUCCESS
                } else {
                    eprintln!("gate: {msg}");
                    ExitCode::FAILURE
                }
            } else {
                println!("ratio hold: meets >= {:.2}%", config.min_ratio * 100.0);
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("gsv-loc-audit: {e}");
            ExitCode::FAILURE
        }
    }
}
