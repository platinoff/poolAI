# PoolAI Version Information

## Current Version: Beta_bolvanka_v1

### Version Details
- **Version**: Beta_bolvanka_v1
- **Release Date**: 2024-12-19
- **Build**: Beta Release
- **Status**: Development/Testing
- **Compatibility**: Rust 1.70+

### Version History

#### Beta_bolvanka_v1 (Current)
- **Release Date**: 2024-12-19
- **Type**: Beta Release
- **Major Features**:
  - Complete PoolAI architecture
  - AI model integration
  - GPU/ASIC/CPU optimization
  - Telegram bot integration
  - Web UI dashboard
  - RAID system
  - Monitoring and alerting
  - Reward system
  - REST API
  - Multi-platform support

#### 0.1.0
- **Release Date**: 2024-12-18
- **Type**: Alpha Release
- **Features**:
  - Initial project structure
  - Basic module architecture
  - Core interfaces and traits

### Version Naming Convention

PoolAI follows a custom versioning scheme:

- **Format**: `[Type]_[Codename]_[Version]`
- **Examples**:
  - `Beta_bolvanka_v1` - Beta release, codename "bolvanka", version 1
  - `Alpha_test_v1` - Alpha release, codename "test", version 1
  - `Release_stable_v1` - Stable release, codename "stable", version 1

### Version Types

- **Alpha**: Early development, unstable features
- **Beta**: Feature complete, testing phase
- **Release**: Stable, production ready
- **Hotfix**: Critical bug fixes

### Codename: "bolvanka"

The codename "bolvanka" represents:
- **B** - Beta release
- **O** - Optimization focused
- **L** - Language model integration
- **V** - Virtualization support
- **A** - AI/ML capabilities
- **N** - Network infrastructure
- **K** - Kernel-level optimizations
- **A** - Advanced features

### Build Information

```rust
const VERSION: &str = "Beta_bolvanka_v1";
const BUILD_DATE: &str = env!("VERGEN_BUILD_TIMESTAMP");
const GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");
const RUST_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");
```

### Compatibility Matrix

| Component | Minimum Version | Recommended Version |
|-----------|----------------|-------------------|
| Rust | 1.70.0 | 1.75.0+ |
| Tokio | 1.36.0 | 1.36.0+ |
| Actix-web | 4.4.0 | 4.4.0+ |
| PyTorch | 0.14.0 | 0.14.0+ |
| CUDA | 11.8 | 12.0+ |
| OS | Windows 10, Ubuntu 20.04 | Latest LTS |

### Migration Guide

#### From 0.1.0 to Beta_bolvanka_v1

**Breaking Changes:**
- Complete architecture rewrite
- New module structure
- Updated API endpoints
- Changed configuration format

**Migration Steps:**
1. Backup existing configuration
2. Update to new config format
3. Migrate API calls to new endpoints
4. Update dependencies
5. Test all functionality

### Roadmap

#### Upcoming Versions

- **Beta_bolvanka_v2** - Enhanced GPU optimization
- **Release_stable_v1** - Production ready release
- **Release_enterprise_v1** - Enterprise features

#### Planned Features

- **v2.0**: Advanced model management
- **v2.1**: Enhanced monitoring
- **v2.2**: Cloud integration
- **v3.0**: Distributed deployment

### Support

- **Current Version Support**: Until Beta_bolvanka_v2 release
- **Security Updates**: Immediate for critical issues
- **Bug Fixes**: Within 2 weeks for major issues
- **Feature Requests**: Considered for next version

### Release Notes

For detailed information about this version, see [CHANGELOG.md](./CHANGELOG.md).

### Contributing

To contribute to PoolAI development:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### License

PoolAI is licensed under the MIT License. See [LICENSE](./LICENSE) for details. 