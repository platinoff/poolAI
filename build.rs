use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Генерируем информацию о версии
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "Beta_bolvanka_v1".to_string());
    let build_date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    
    // Получаем информацию о Git (если доступна)
    let git_commit = get_git_commit().unwrap_or_else(|| "unknown".to_string());
    let git_branch = get_git_branch().unwrap_or_else(|| "unknown".to_string());
    
    // Создаем файл с информацией о версии
    let version_info = format!(
        r#"// Auto-generated version information
pub const VERSION: &str = "{}";
pub const BUILD_DATE: &str = "{}";
pub const GIT_COMMIT: &str = "{}";
pub const GIT_BRANCH: &str = "{}";
pub const RUST_VERSION: &str = "{}";
"#,
        version,
        build_date,
        git_commit,
        git_branch,
        env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string())
    );
    
    // Записываем в файл
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let version_file = Path::new(&out_dir).join("version.rs");
    fs::write(&version_file, version_info).expect("Failed to write version.rs");
    
    // Указываем, что файл создан
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:version_file={}", version_file.display());
    
    // Создаем файл с информацией о сборке
    let build_info = format!(
        r#"// Auto-generated build information
pub const BUILD_TARGET: &str = "{}";
pub const BUILD_PROFILE: &str = "{}";
pub const BUILD_OPT_LEVEL: &str = "{}";
pub const BUILD_DEBUG: &str = "{}";
"#,
        env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()),
        env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string()),
        env::var("OPT_LEVEL").unwrap_or_else(|_| "0".to_string()),
        env::var("DEBUG").unwrap_or_else(|_| "false".to_string())
    );
    
    let build_file = Path::new(&out_dir).join("build.rs");
    fs::write(&build_file, build_info).expect("Failed to write build.rs");
    
    // Создаем файл с информацией о системе
    let system_info = format!(
        r#"// Auto-generated system information
pub const OS: &str = "{}";
pub const ARCH: &str = "{}";
pub const POINTER_WIDTH: &str = "{}";
pub const ENDIAN: &str = "{}";
"#,
        env::consts::OS,
        env::consts::ARCH,
        std::mem::size_of::<usize>() * 8,
        if cfg!(target_endian = "little") { "little" } else { "big" }
    );
    
    let system_file = Path::new(&out_dir).join("system.rs");
    fs::write(&system_file, system_info).expect("Failed to write system.rs");
    
    // Создаем основной файл с информацией
    let main_info = format!(
        r#"// Auto-generated information file
pub mod version {{
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}}

pub mod build {{
    include!(concat!(env!("OUT_DIR"), "/build.rs"));
}}

pub mod system {{
    include!(concat!(env!("OUT_DIR"), "/system.rs"));
}}

pub fn get_version_info() -> &'static str {{
    "PoolAI {} - Build: {} - Commit: {} - Branch: {}"
}}

pub fn get_full_version_info() -> String {{
    format!(
        "PoolAI v{} (Build: {} - Git: {} - Branch: {} - Target: {} - Profile: {})",
        version::VERSION,
        version::BUILD_DATE,
        version::GIT_COMMIT,
        version::GIT_BRANCH,
        build::BUILD_TARGET,
        build::BUILD_PROFILE
    )
}}
"#,
        version, build_date, git_commit, git_branch,
        version, build_date, git_commit, git_branch
    );
    
    let info_file = Path::new(&out_dir).join("info.rs");
    fs::write(&info_file, main_info).expect("Failed to write info.rs");
}

fn get_git_commit() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
}

fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
} 