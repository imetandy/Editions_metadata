# How to Share Your App on GitHub

## The Easy Way

GitHub is perfect for sharing your app. Here are the best ways to do it:

## Option 1: GitHub Releases (Recommended)

This is the easiest and most professional way:

### Step 1: Build Your Installer
```bash
cd metadata-generator
./scripts/create-dmg-installer.sh
```

### Step 2: Create a Release
1. **Go to your GitHub repository**
2. **Click "Releases"** on the right side
3. **Click "Create a new release"**
4. **Add a version tag** like `v1.0.0`
5. **Write a title** like "MetadataGenerator v1.0.0"
6. **Upload your DMG file**
7. **Write a simple description**
8. **Click "Publish release"**

That's it! Now anyone can download your app from the releases page.

### What to Write in the Description
```
## What's New
- Professional installer with custom icon
- Easy drag-and-drop installation
- Works on macOS 10.15+

## How to Install
1. Download the DMG file
2. Double-click to open
3. Drag to Applications
4. Launch from Applications

## System Requirements
- macOS 10.15 or later
- No other software needed
```

## Option 2: Automated Releases

If you want GitHub to build and release automatically:

### Set Up Automation
1. **The workflow file is already created** (`.github/workflows/release.yml`)
2. **Create a tag** to trigger the release:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. **GitHub will automatically**:
   - Build your app
   - Create the installer
   - Make a release
   - Upload the files

## Option 3: Simple File Upload

Just upload the DMG file directly:

1. **Upload** the DMG file to your repository
2. **Add a download link** to your README:
   ```markdown
   ## Download
   [Download MetadataGenerator-Installer.dmg](link-to-your-file)
   ```

## For Your Users

Tell them to:

1. **Go to** your GitHub repository
2. **Click "Releases"** on the right side
3. **Download** the DMG file
4. **Double-click** to open
5. **Drag** to Applications
6. **Launch** from Applications

## Making Updates

When you want to share a new version:

1. **Make your changes** to the code
2. **Build the new installer**
3. **Create a new release** on GitHub
4. **Upload the new DMG file**
5. **Tell your users** about the update

## Tips

- **Keep it simple** - users just want to download and use
- **Test first** - make sure the installer works
- **Clear instructions** - tell users exactly what to do
- **Version numbers** - use simple numbers like 1.0, 1.1, 1.2
- **Good descriptions** - explain what's new in each version

## If Something Goes Wrong

### Can't Upload
- Check the file size (should be around 2.7MB)
- Make sure you're logged into GitHub
- Try uploading again

### Release Won't Show
- Make sure you clicked "Publish release"
- Check that the tag is correct
- Wait a few minutes for GitHub to process

### Users Can't Download
- Make sure the release is published (not draft)
- Check that the file uploaded completely
- Try downloading it yourself first 