# CI/CD Pipeline for Metadata Generator

This document explains the Continuous Integration and Continuous Deployment (CI/CD) pipeline setup for the Metadata Generator project.

## Overview

The CI/CD pipeline consists of two main workflows:

1. **Test and Build** (`test.yml`) - Runs on pull requests and pushes to main/develop branches
2. **Build and Release** (`release.yml`) - Creates releases when tags are pushed

## Workflows

### Test and Build Workflow

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch

**What it does:**
- Runs tests on Ubuntu, Windows, and macOS
- Checks code formatting with `cargo fmt`
- Runs Clippy linting with `cargo clippy`
- Builds both CLI and GUI versions to ensure they compile

### Build and Release Workflow

**Triggers:**
- Push of tags starting with `v` (e.g., `v1.0.0`, `v2.1.3`)
- Manual trigger via GitHub Actions UI

**What it does:**
- Builds CLI and GUI versions for multiple platforms:
  - Windows (x64)
  - macOS (Intel x64 and Apple Silicon ARM64)
- Creates zip archives for each platform
- Automatically creates a GitHub release with the built artifacts
- Generates release notes automatically

## Supported Platforms

| Platform | Architecture | Target |
|----------|--------------|---------|
| Windows | x64 | `x86_64-pc-windows-msvc` |
| macOS | Intel x64 | `x86_64-apple-darwin` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |

## Local Development

### Prerequisites

1. **Rust Toolchain**: Install Rust via [rustup](https://rustup.rs/)
2. **Cross-compilation targets**: Install required targets:
   ```bash
   rustup target add x86_64-pc-windows-msvc
   rustup target add x86_64-apple-darwin
   rustup target add aarch64-apple-darwin
   ```

### Building Locally

#### On macOS/Linux:
```bash
# Make the build script executable (first time only)
chmod +x scripts/build.sh

# Run the build script
./scripts/build.sh
```

#### On Windows:
```cmd
# Run the build script
scripts\build.bat
```

The build scripts will:
1. Clean previous builds
2. Build CLI and GUI versions for all supported platforms
3. Create distribution packages in the `dist/` directory
4. Generate zip files for each platform

### Manual Building

You can also build manually for specific targets:

```bash
cd metadata-generator

# Build CLI version
cargo build --release --features cli --target x86_64-pc-windows-msvc

# Build GUI version
cargo build --release --features gui --target x86_64-apple-darwin
```

## Creating Releases

### Automatic Releases

1. **Update version** in `metadata-generator/Cargo.toml`:
   ```toml
   [package]
   version = "1.0.0"  # Update this
   ```

2. **Create and push a tag**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

3. **The workflow will automatically**:
   - Build all versions
   - Create a GitHub release
   - Upload the built artifacts
   - Generate release notes

### Manual Releases

You can also trigger releases manually:

1. Go to the **Actions** tab in your GitHub repository
2. Select the **Build and Release** workflow
3. Click **Run workflow**
4. Choose the branch and click **Run workflow**

## Release Artifacts

Each release will contain the following files:

- `MetadataGenerator-windows-x64.zip` - Windows executable (CLI + GUI)
- `MetadataGenerator-macos-x64.zip` - macOS Intel executable (CLI + GUI)
- `MetadataGenerator-macos-arm64.zip` - macOS Apple Silicon executable (CLI + GUI)

Each zip file contains:
- `cli.exe` / `cli` - Command-line interface
- `MetadataGenerator.exe` / `MetadataGenerator` - Graphical user interface

## Troubleshooting

### Common Issues

1. **Build fails on specific target**:
   - Ensure you have the required target installed: `rustup target add <target>`
   - Check that all dependencies are available for the target platform

2. **GUI build fails on Windows**:
   - Ensure you have the Windows SDK installed
   - Some GUI dependencies might require additional system libraries

3. **Release not created**:
   - Check that the tag follows the `v*` pattern (e.g., `v1.0.0`)
   - Ensure the workflow has permission to create releases
   - Check the Actions tab for any error messages

### Debugging

1. **Check workflow logs**: Go to Actions tab and click on the failed workflow
2. **Test locally**: Use the build scripts to test builds locally
3. **Check dependencies**: Ensure all dependencies in `Cargo.toml` are compatible with all targets

## Configuration

### Workflow Configuration

The workflows are configured in:
- `.github/workflows/test.yml` - Test and build workflow
- `.github/workflows/release.yml` - Release workflow

### Build Configuration

Build settings are controlled by:
- `metadata-generator/Cargo.toml` - Package configuration and dependencies
- Feature flags (`cli` and `gui`) control which binaries are built

## Security

- The workflows use `GITHUB_TOKEN` for authentication
- No sensitive data is stored in the workflows
- All builds run in isolated environments
- Dependencies are cached to improve build speed and security

## Performance

- Builds are parallelized across different platforms
- Dependencies are cached between builds
- Only changed files trigger rebuilds
- Release builds are optimized with `--release` flag

## Contributing

When contributing to the CI/CD pipeline:

1. Test changes locally first
2. Update this documentation if needed
3. Ensure workflows work on all supported platforms
4. Test both CLI and GUI builds
5. Verify release creation process

## Support

If you encounter issues with the CI/CD pipeline:

1. Check the GitHub Actions logs
2. Test builds locally
3. Review this documentation
4. Create an issue in the repository with detailed error information 