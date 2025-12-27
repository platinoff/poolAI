use std::env;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Use SystemTime instead of chrono to avoid external dependencies
    let build_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("build_time.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "pub const BUILD_TIME: u64 = {};", build_time).unwrap();
}
