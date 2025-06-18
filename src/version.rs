//! Version information for PoolAI
//! Auto-generated build information

/// Current version of PoolAI
pub const VERSION: &str = "Beta_bolvanka_v1";

/// Build date and time
pub const BUILD_DATE: &str = env!("VERGEN_BUILD_TIMESTAMP");

/// Git commit hash
pub const GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");

/// Git branch name
pub const GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");

/// Rust compiler version
pub const RUST_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");

/// Build target triple
pub const BUILD_TARGET: &str = env!("VERGEN_TARGET_TRIPLE");

/// Build profile (debug/release)
pub const BUILD_PROFILE: &str = env!("VERGEN_CARGO_PROFILE");

/// Cargo package name
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

/// Cargo package version
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Cargo package description
pub const PACKAGE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Cargo package authors
pub const PACKAGE_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Cargo package repository
pub const PACKAGE_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Cargo package license
pub const PACKAGE_LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// Cargo package keywords
pub const PACKAGE_KEYWORDS: &str = env!("CARGO_PKG_KEYWORDS");

/// Cargo package categories
pub const PACKAGE_CATEGORIES: &str = env!("CARGO_PKG_CATEGORIES");

/// System information
pub const OS: &str = env!("VERGEN_SYSINFO_OS");
pub const ARCH: &str = env!("VERGEN_SYSINFO_ARCH");
pub const POINTER_WIDTH: &str = env!("VERGEN_SYSINFO_POINTER_WIDTH");
pub const ENDIAN: &str = env!("VERGEN_SYSINFO_ENDIAN");

/// Version information structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub build_date: String,
    pub git_commit: String,
    pub git_branch: String,
    pub rust_version: String,
    pub build_target: String,
    pub build_profile: String,
    pub package_name: String,
    pub package_version: String,
    pub package_description: String,
    pub package_authors: String,
    pub package_repository: String,
    pub package_license: String,
    pub os: String,
    pub arch: String,
    pub pointer_width: String,
    pub endian: String,
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            build_date: BUILD_DATE.to_string(),
            git_commit: GIT_COMMIT.to_string(),
            git_branch: GIT_BRANCH.to_string(),
            rust_version: RUST_VERSION.to_string(),
            build_target: BUILD_TARGET.to_string(),
            build_profile: BUILD_PROFILE.to_string(),
            package_name: PACKAGE_NAME.to_string(),
            package_version: PACKAGE_VERSION.to_string(),
            package_description: PACKAGE_DESCRIPTION.to_string(),
            package_authors: PACKAGE_AUTHORS.to_string(),
            package_repository: PACKAGE_REPOSITORY.to_string(),
            package_license: PACKAGE_LICENSE.to_string(),
            os: OS.to_string(),
            arch: ARCH.to_string(),
            pointer_width: POINTER_WIDTH.to_string(),
            endian: ENDIAN.to_string(),
        }
    }
}

impl VersionInfo {
    /// Get full version string
    pub fn get_full_version(&self) -> String {
        format!(
            "PoolAI v{} (Build: {} - Git: {} - Branch: {} - Target: {} - Profile: {})",
            self.version,
            self.build_date,
            self.git_commit,
            self.git_branch,
            self.build_target,
            self.build_profile
        )
    }
    
    /// Get short version string
    pub fn get_short_version(&self) -> String {
        format!("PoolAI v{}", self.version)
    }
    
    /// Get build information
    pub fn get_build_info(&self) -> String {
        format!(
            "Build: {} - Git: {} - Branch: {}",
            self.build_date,
            self.git_commit,
            self.git_branch
        )
    }
    
    /// Get system information
    pub fn get_system_info(&self) -> String {
        format!(
            "OS: {} - Arch: {} - Pointer: {} - Endian: {}",
            self.os,
            self.arch,
            self.pointer_width,
            self.endian
        )
    }
    
    /// Check if this is a development build
    pub fn is_development(&self) -> bool {
        self.build_profile == "debug"
    }
    
    /// Check if this is a release build
    pub fn is_release(&self) -> bool {
        self.build_profile == "release"
    }
    
    /// Check if this is a beta version
    pub fn is_beta(&self) -> bool {
        self.version.contains("Beta")
    }
    
    /// Check if this is a stable version
    pub fn is_stable(&self) -> bool {
        !self.version.contains("Beta") && !self.version.contains("Alpha")
    }
}

/// Get current version info
pub fn get_version_info() -> VersionInfo {
    VersionInfo::default()
}

/// Get full version string
pub fn get_full_version() -> String {
    get_version_info().get_full_version()
}

/// Get short version string
pub fn get_short_version() -> String {
    get_version_info().get_short_version()
}

/// Get build information
pub fn get_build_info() -> String {
    get_version_info().get_build_info()
}

/// Get system information
pub fn get_system_info() -> String {
    get_version_info().get_system_info()
}

/// Check if this is a development build
pub fn is_development() -> bool {
    get_version_info().is_development()
}

/// Check if this is a release build
pub fn is_release() -> bool {
    get_version_info().is_release()
}

/// Check if this is a beta version
pub fn is_beta() -> bool {
    get_version_info().is_beta()
}

/// Check if this is a stable version
pub fn is_stable() -> bool {
    get_version_info().is_stable()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_info() {
        let info = get_version_info();
        assert!(!info.version.is_empty());
        assert!(!info.build_date.is_empty());
        assert!(!info.git_commit.is_empty());
        assert!(!info.git_branch.is_empty());
        assert!(!info.rust_version.is_empty());
        assert!(!info.build_target.is_empty());
        assert!(!info.build_profile.is_empty());
    }
    
    #[test]
    fn test_version_strings() {
        assert!(!get_full_version().is_empty());
        assert!(!get_short_version().is_empty());
        assert!(!get_build_info().is_empty());
        assert!(!get_system_info().is_empty());
    }
    
    #[test]
    fn test_version_checks() {
        // These should not panic
        let _ = is_development();
        let _ = is_release();
        let _ = is_beta();
        let _ = is_stable();
    }
} 