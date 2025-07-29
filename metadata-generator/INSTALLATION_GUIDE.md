# Installation Guide for End Users

## Quick Installation

### Step 1: Download and Install
1. Download `MetadataGenerator-Installer.dmg`
2. Double-click the DMG file to mount it
3. Drag `MetadataGenerator` to your Applications folder
4. Eject the DMG

### Step 2: First Launch
1. Go to Applications and double-click `MetadataGenerator`
2. **If you get a "damaged" error:**
   - Right-click the app
   - Select "Open" from the context menu
   - Click "Open" in the dialog

### Step 3: Grant Permissions
1. If prompted, allow the app to run in System Preferences
2. Go to **System Preferences** → **Security & Privacy**
3. Look for MetadataGenerator and click "Allow"

## Troubleshooting

### App Won't Launch?
1. **Try right-clicking and selecting "Open"**
2. **Run the diagnostic tool:**
   ```bash
   # Download and run the diagnostic script
   ./diagnose-app.sh
   ```
3. **Try running from Terminal:**
   ```bash
   /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
   ```

### App Crashes When Analyzing Files?
This is usually due to missing FFmpeg libraries. Install them:

**Option 1 (Recommended):**
```bash
# Install Homebrew (if you don't have it)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install FFmpeg
brew install ffmpeg
```

**Option 2 (Alternative):**
1. Download FFmpeg from https://ffmpeg.org/download.html
2. Install it in your Applications folder
3. The app will automatically detect it there

### Still Having Issues?
1. Check the full troubleshooting guide: `END_USER_TROUBLESHOOTING.md`
2. Run the diagnostic script and include its output when asking for help
3. Try the CLI version if the GUI doesn't work

## System Requirements

- **macOS:** 10.15 (Catalina) or later
- **Architecture:** Intel (x86_64) or Apple Silicon (ARM64)
- **Dependencies:** FFmpeg (automatically installed if needed)

## Quick Test

To test if the app works:
1. Create a folder with some image files (JPG, PNG)
2. Launch MetadataGenerator
3. Click "Browse for folder" and select your test folder
4. Fill in the artwork details
5. Click "Generate metadata"

If this works, the app is properly installed! 