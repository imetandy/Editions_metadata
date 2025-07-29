# Troubleshooting Guide

This guide helps you resolve common issues with the CI/CD pipeline.

## 🚨 Common Issues

### **1. Release Permission Error (403)**

**Error:**
```
GitHub release failed with status: 403
Resource not accessible by integration
```

**Cause:** The GitHub token doesn't have sufficient permissions to create releases.

**Solution:**
1. **Check workflow permissions** - Ensure the workflow has `contents: write` permission (required by [GitHub API](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#create-a-release))
2. **Verify repository settings** - Go to Settings → Actions → General → Workflow permissions
3. **Use Personal Access Token** (if needed) - Create a PAT with `repo` scope
4. **Check authentication** - Ensure using GitHub App installation tokens, fine-grained PATs, or standard PATs

**Repository Settings Check:**
1. Go to your repository on GitHub
2. Click **Settings** → **Actions** → **General**
3. Under **Workflow permissions**, select:
   - ✅ **Read and write permissions**
   - ✅ **Allow GitHub Actions to create and approve pull requests**

### **2. Build Failures**

**Common causes:**
- Missing dependencies
- Rust toolchain issues
- Platform-specific build problems

**Solutions:**
```bash
# Test locally first
cd metadata-generator
cargo build --release --features cli
cargo build --release --features gui

# Check for missing targets
rustup target list --installed
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### **3. PowerShell Syntax Errors**

**Error:** `Missing '(' after 'if' in if statement`

**Cause:** Bash syntax running in PowerShell

**Solution:** Ensure all steps use `shell: bash` for cross-platform compatibility.

### **4. Deprecated Action Warnings**

**Error:** `set-output command is deprecated`

**Cause:** Using outdated GitHub Actions

**Solution:** Update to latest action versions:
- `actions/upload-artifact@v4`
- `actions/download-artifact@v4`
- `actions/cache@v4`
- `dtolnay/rust-toolchain@stable`

### **5. Release Not Created**

**Causes:**
- Tag doesn't follow `v*` pattern
- Workflow condition not met
- Permission issues

**Solutions:**
1. **Check tag format:** Must be `v1.0.0`, `v2.1.3`, etc.
2. **Verify workflow trigger:** Should run on tag push or manual trigger
3. **Check permissions:** Ensure `contents: write` permission

## 🔧 Manual Release Creation

If the automated release fails, you can create it manually:

### **Using GitHub CLI:**
```bash
# Install GitHub CLI if not installed
# brew install gh (macOS)
# choco install gh (Windows)

# Login to GitHub
gh auth login

# Create release
gh release create v1.0.0 \
  --title "Release v1.0.0" \
  --notes "Release notes here" \
  ./dist/*.zip
```

### **Using GitHub Web Interface:**
1. Go to your repository on GitHub
2. Click **Releases** → **Create a new release**
3. Choose a tag or create a new one
4. Upload the built artifacts
5. Write release notes
6. Click **Publish release**

## 🛠️ Debugging Workflows

### **Enable Debug Logging:**
Add this to your workflow to get more detailed logs:
```yaml
env:
  ACTIONS_STEP_DEBUG: true
  ACTIONS_RUNNER_DEBUG: true
```

### **Check Workflow Logs:**
1. Go to **Actions** tab in your repository
2. Click on the failed workflow run
3. Click on the failed job
4. Click on the failed step
5. Review the logs for error details

### **Test Locally:**
```bash
# Test the build process locally
./scripts/build.sh

# Test the release script
./scripts/release.sh --version 1.0.0 --dry-run
```

## 🔐 Permission Reference

### **Required Permissions:**

| Action | Permission | Scope |
|--------|------------|-------|
| Create releases | `contents: write` | Repository |
| Upload artifacts | `contents: write` | Repository |
| Create PRs | `pull-requests: write` | Repository |
| Read code | `contents: read` | Repository |

### **Setting Permissions:**

**In workflow file:**
```yaml
permissions:
  contents: write
  pull-requests: write
```

**In repository settings:**
1. Settings → Actions → General
2. Workflow permissions → Read and write permissions
3. Allow GitHub Actions to create and approve pull requests

## 📞 Getting Help

If you're still having issues:

1. **Check the logs** - Look for specific error messages
2. **Test locally** - Try building and releasing locally first
3. **Review this guide** - Check if your issue is covered here
4. **Create an issue** - Open an issue in the repository with:
   - Error message
   - Workflow run link
   - Steps to reproduce
   - Expected vs actual behavior

## 🔄 Workflow Status

Monitor your workflows:
- **Green checkmark** ✅ - Success
- **Red X** ❌ - Failure
- **Yellow dot** 🟡 - In progress
- **Gray dot** ⚪ - Skipped

Click on any status to view detailed logs and troubleshoot issues. 