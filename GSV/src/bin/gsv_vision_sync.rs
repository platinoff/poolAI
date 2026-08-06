//! GSV vision sync bin — mirrors the poolAI vision canon into `GSV/data/`.
//!
//! Reads `docs/vision/manifest.json` + `docs/vision/feed.json` at the poolAI
//! root and persists `GSV/data/gsv_manifest.json` + `GSV/data/gsv_feed.json`.
//!
//! ```text
//! cargo run --bin gsv-vision-sync                # write snapshots
//! cargo run --bin gsv-vision-sync -- --check     # drift gate, no write
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use gsv::boxes::vision;

fn parse_args() -> (bool, Option<PathBuf>, Option<PathBuf>) {
    let mut check_only = false;
    let mut root = None;
    let mut data_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check_only = true,
            "--repo-root" => root = args.next().map(PathBuf::from),
            "--data-dir" => data_dir = args.next().map(PathBuf::from),
            "--help" | "-h" => {
                println!("Usage: gsv-vision-sync [--check] [--repo-root PATH] [--data-dir PATH]");
                std::process::exit(0);
            }
            _ => {}
        }
    }
    (check_only, root, data_dir)
}

fn main() -> ExitCode {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (check_only, root, data_dir) = parse_args();
    let repo_root = root.unwrap_or_else(|| manifest_dir.parent().map(PathBuf::from).unwrap());
    let data = data_dir.unwrap_or_else(|| manifest_dir.join("data"));

    if check_only {
        let issues = vision::collect_drift(&repo_root, &data);
        if issues.is_empty() {
            match vision::read_manifest(&repo_root) {
                Ok(m) => {
                    println!(
                        "vision drift check: ok (revision {}, next {})",
                        m.revision, m.next_sprint
                    );
                }
                Err(_) => {
                    println!("vision drift check: ok (no revision)");
                }
            }
            ExitCode::SUCCESS
        } else {
            eprintln!("vision drift check: {} issue(s)", issues.len());
            for issue in &issues {
                eprintln!("  - {issue}");
            }
            ExitCode::FAILURE
        }
    } else {
        match vision::sync(&repo_root, &data) {
            Ok(report) => {
                println!(
                    "vision sync: revision {}, {} nodes, {} edges, {} feed items (git {}), next {}",
                    report.revision,
                    report.nodes_count,
                    report.edges_count,
                    report.feed_items,
                    report.git_head,
                    report.next_sprint
                );
                println!("  -> {}", report.manifest_target);
                println!("  -> {}", report.feed_target);
                println!("  -> {}", report.extensions_target);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("gsv-vision-sync: {e}");
                ExitCode::FAILURE
            }
        }
    }
}
