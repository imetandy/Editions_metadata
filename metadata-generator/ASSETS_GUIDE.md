# App Icon Guide

## What You Need

Your app needs an icon to look professional. Here's how to add one:

## Icon Requirements

### File Format
- **Best**: `.icns` file (macOS icon format)
- **Also works**: `.png` file (1024x1024 pixels)

### Size
- **Minimum**: 1024x1024 pixels
- **Bigger is better** - it will look sharper

### Where to Put It
Put your icon in: `assets/icons/AppIcon.icns` or `assets/icons/AppIcon.png`

## How to Create an Icon

### Option 1: Online Tools (Easiest)
- **Icon Kitchen**: https://icon.kitchen/
- **MakeAppIcon**: https://makeappicon.com/
- **App Icon Generator**: https://appicon.co/

### Option 2: Design Software
- **Sketch**, **Figma**, **Photoshop** - create a 1024x1024 PNG
- **Icon Composer** (if you have Xcode)

### Option 3: Convert Existing Image
If you have an image you want to use:
```bash
# Resize to 1024x1024
magick your-image.png -resize 1024x1024 assets/icons/AppIcon.png
```

## Icon Design Tips

### Keep It Simple
- **Clean design** - should look good at small sizes
- **High contrast** - works on light and dark backgrounds
- **No small text** - becomes unreadable when small
- **Square format** - macOS will round the corners

### Colors
- **Solid background** - white, black, or a solid color
- **Avoid** too many colors or complex gradients
- **Test** on both light and dark themes

## How to Add Your Icon

### Step 1: Create Your Icon
Make a 1024x1024 PNG file with your design.

### Step 2: Save It
Save as `assets/icons/AppIcon.png`

### Step 3: Convert to ICNS (Optional)
```bash
./scripts/convert-icon.sh
```

### Step 4: Build Your Installer
```bash
./scripts/create-dmg-installer.sh
```

Your installer will now have your custom icon!

## Common Problems

### Icon Not Showing
- Make sure the file is named correctly
- Check that it's 1024x1024 pixels
- Try rebuilding the installer

### Icon Looks Blurry
- Use a higher resolution image
- Make sure it's 1024x1024 pixels
- Don't scale up a small image

### Build Fails
- Check the file exists in the right place
- Make sure the file isn't corrupted
- Try a different image format

## Quick Start

1. **Create** a 1024x1024 PNG icon
2. **Save** as `assets/icons/AppIcon.png`
3. **Run**: `./scripts/create-dmg-installer.sh`
4. **Done** - your installer has a custom icon!

## Tips

- **Start simple** - you can always improve it later
- **Test it** - make sure it looks good at different sizes
- **Keep a backup** - save your original design file
- **Be consistent** - match your app's style 