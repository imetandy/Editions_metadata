# Troubleshooting Guide

## FFmpeg Library Issues

If you see an error like this when trying to run MetadataGenerator:

```
Library not loaded: /opt/homebrew/*/libavutil.59.dylib
```

This means your system is missing FFmpeg libraries that the app needs to analyze video and audio files.

### Quick Fix (Recommended)

Run this command in Terminal to install the required dependencies:

```bash
./scripts/install-ffmpeg-deps.sh
```

This script will:
1. Check if Homebrew is installed
2. Install FFmpeg if it's missing
3. Verify the installation

### Manual Installation

If the script doesn't work, you can install FFmpeg manually:

1. **Install Homebrew** (if you don't have it):
   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

2. **Install FFmpeg**:
   ```bash
   brew install ffmpeg
   ```

3. **Restart your terminal** and try running MetadataGenerator again.

### Alternative Solutions

#### Option 1: Use the CLI Version
If the GUI version has issues, try the command-line version:
```bash
cargo run --features cli -- -p /path/to/your/artwork/folder
```

#### Option 2: Build from Source
If you're a developer, you can build the app with bundled libraries:
```bash
./scripts/create-dmg-installer.sh
```

This will create a new DMG with all dependencies included.

### Still Having Issues?

1. **Check your macOS version** - The app requires macOS 10.15 or later
2. **Try running from Terminal** - Sometimes double-clicking doesn't show error messages
3. **Check System Preferences** - Make sure the app is allowed to run (Security & Privacy settings)

### Contact Support

If none of these solutions work, please:
1. Note your macOS version
2. Include the full error message
3. Try running the app from Terminal and include any output

## Other Common Issues

### App Shows as "Damaged"
This is usually a macOS security feature. To fix:

1. Right-click the app
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears

### Permission Denied
If you get permission errors:

```bash
chmod +x /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
```

### App Won't Start
Try running from Terminal to see error messages:

```bash
/Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
``` 