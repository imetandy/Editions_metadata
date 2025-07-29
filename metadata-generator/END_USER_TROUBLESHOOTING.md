# End User Troubleshooting Guide

If MetadataGenerator isn't launching or working properly, follow these steps in order:

## 🚨 Quick Fixes (Try These First)

### 1. Right-Click and "Open"
If the app doesn't launch when double-clicked:
1. Right-click on MetadataGenerator in Applications
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears

### 2. Check Security Settings
1. Go to **System Preferences** → **Security & Privacy**
2. Look for a message about MetadataGenerator being blocked
3. Click "Allow Anyway" or "Open Anyway"

### 3. Remove Quarantine Attribute
Open Terminal and run:
```bash
xattr -cr /Applications/MetadataGenerator.app
```

## 🔍 Detailed Diagnosis

### Step 1: Run the Diagnostic Tool
1. Download the diagnostic script from the project
2. Open Terminal
3. Run: `./diagnose-app.sh`
4. Follow the recommendations it provides

### Step 2: Try Running from Terminal
Open Terminal and run:
```bash
/Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
```

This will show any error messages that are hidden when double-clicking.

### Step 3: Check Console for Crash Logs
1. Open **Console.app** (Applications → Utilities → Console)
2. Look for entries related to "MetadataGenerator"
3. Check for any error messages or crash reports

## 🛠️ Common Solutions

### Missing FFmpeg Libraries
**Symptoms:** App launches but crashes when trying to analyze video/audio files

**Solution 1 (Recommended):**
```bash
# Install Homebrew (if you don't have it)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install FFmpeg
brew install ffmpeg
```

**Solution 2 (Alternative):**
1. Download FFmpeg from https://ffmpeg.org/download.html
2. Install it in your Applications folder
3. The app will automatically detect it there

### Architecture Mismatch
**Symptoms:** App won't launch at all

**Check your Mac's architecture:**
```bash
uname -m
```
- `x86_64` = Intel Mac
- `arm64` = Apple Silicon (M1/M2) Mac

**Solutions:**
- **Intel Mac with ARM64 app:** Not compatible, need x86_64 version
- **Apple Silicon Mac with x86_64 app:** Should work with Rosetta 2

### macOS Version Issues
**Symptoms:** App won't launch

**Check your macOS version:**
```bash
sw_vers -productVersion
```

**Requirements:** macOS 10.15 (Catalina) or later

## 📋 Step-by-Step Troubleshooting

### If the app doesn't appear in Activity Monitor:

1. **Check if it's actually running:**
   ```bash
   ps aux | grep MetadataGenerator
   ```

2. **Check for missing dependencies:**
   ```bash
   otool -L /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
   ```

3. **Look for "not found" libraries** in the output above

4. **Install missing dependencies:**
   ```bash
   brew install ffmpeg
   ```

### If the app launches but crashes:

1. **Check Console.app** for crash logs
2. **Try the CLI version** instead:
   ```bash
   # If you have the source code
   cargo run --features cli -- -p /path/to/your/folder
   ```

### If you get permission errors:

1. **Fix permissions:**
   ```bash
   chmod +x /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
   ```

2. **Check file ownership:**
   ```bash
   ls -la /Applications/MetadataGenerator.app/Contents/MacOS/
   ```

## 🆘 Still Having Issues?

### Contact Support
When reporting issues, please include:

1. **macOS version:** `sw_vers -productVersion`
2. **Mac architecture:** `uname -m`
3. **Error messages** from Terminal
4. **Console.app logs** related to MetadataGenerator
5. **Steps to reproduce** the problem

### Alternative Solutions

1. **Use the CLI version** if available
2. **Try running on a different Mac** to isolate the issue
3. **Check if the issue is specific to certain file types**

## 🔧 Advanced Troubleshooting

### Rebuilding the App Bundle
If you have the source code:

```bash
cd metadata-generator
./scripts/create-dmg-installer.sh
```

### Manual FFmpeg Installation
If Homebrew doesn't work:

1. Download FFmpeg from https://ffmpeg.org/download.html
2. Extract to `/usr/local/`
3. Add to PATH in your shell profile

### Checking Library Dependencies
```bash
# See all library dependencies
otool -L /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator

# Check for missing libraries
otool -L /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator | grep "not found"
```

## 📞 Getting Help

1. **Check this troubleshooting guide first**
2. **Run the diagnostic script** and include its output
3. **Try the CLI version** to see if it's a GUI-specific issue
4. **Include system information** when reporting problems

Remember: Most issues are related to missing FFmpeg dependencies or macOS security settings! 