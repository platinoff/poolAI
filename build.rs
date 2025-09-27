use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let build_time = chrono::Utc::now().to_rfc3339();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("build_time.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "pub const BUILD_TIME: &str = \"{}\";", build_time).unwrap();
} 