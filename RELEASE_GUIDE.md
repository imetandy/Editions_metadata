# Release Guide

This guide explains how to create releases for the Metadata Generator project.

## 🚀 Creating Your First Release

### **Option 1: Using the Release Script (Recommended)**

1. **Update the version** in `metadata-generator/Cargo.toml`:
   ```toml
   [package]
   version = "1.0.0"  # Change this to your desired version
   ```

2. **Run the release script**:
   ```bash
   # Test what will happen (dry run)
   ./scripts/release.sh --version 1.0.0 --dry-run
   
   # Actually create the release
   ./scripts/release.sh --version 1.0.0
   ```

3. **The script will automatically**:
   - Update the version in `Cargo.toml`
   - Commit the changes
   - Create a git tag (`v1.0.0`)
   - Push the changes and tag to GitHub
   - Trigger the CI/CD pipeline to build and create a release

### **Option 2: Manual Process**

1. **Update version** in `metadata-generator/Cargo.toml`
2. **Commit and push** the changes:
   ```bash
   git add metadata-generator/Cargo.toml
   git commit -m "chore: bump version to 1.0.0"
   git push origin master
   ```
3. **Create and push a tag**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

### **Option 3: Manual GitHub Actions Trigger**

1. **Go to GitHub Actions** in your repository
2. **Select "Build and Release"** workflow
3. **Click "Run workflow"**
4. **Choose the branch** (usually `master`)
5. **Click "Run workflow"**

This will build the project and create a release using the current version in `Cargo.toml`.

## 📋 Release Process

When you create a release, the CI/CD pipeline will:

1. **Build** the application for all platforms:
   - Windows (x64)
   - macOS (Intel x64)
   - macOS (Apple Silicon ARM64)

2. **Create artifacts**:
   - `MetadataGenerator-windows-x64.zip`
   - `MetadataGenerator-macos-x64.zip`
   - `MetadataGenerator-macos-arm64.zip`

3. **Create a GitHub release** with:
   - All built artifacts
   - Auto-generated release notes
   - Tag matching the version

## 🎯 Version Guidelines

Use [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., `1.0.0`)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

Examples:
- `1.0.0` - First stable release
- `1.1.0` - New features added
- `1.1.1` - Bug fixes
- `2.0.0` - Breaking changes

## 🔍 Monitoring the Release

1. **Check GitHub Actions**: Go to Actions tab to monitor build progress
2. **Check Releases**: Go to Releases tab to see the created release
3. **Download artifacts**: Users can download the zip files from the release

## 🛠️ Troubleshooting

### **Release not created**
- Check that the tag follows the `v*` pattern (e.g., `v1.0.0`)
- Ensure the workflow has permission to create releases
- Check the Actions tab for any error messages

### **Build fails**
- Check that all dependencies are compatible
- Verify the Rust toolchain is working
- Look at the specific platform that failed

### **Manual trigger not working**
- Ensure you're running the "Build and Release" workflow
- Check that the version in `Cargo.toml` is valid
- Verify the workflow has the necessary permissions

## 📦 Release Artifacts

Each release contains:

- **CLI version**: Command-line interface
- **GUI version**: Graphical user interface
- **Multiple platforms**: Windows, macOS Intel, macOS ARM64

Users can download the appropriate zip file for their platform and extract it to get both the CLI and GUI versions. 