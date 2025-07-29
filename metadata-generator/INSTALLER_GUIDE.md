# Installer Guide - Choose the Right Installer

This guide helps you choose the best installer type for your needs.

## 🎯 **Which Installer Should You Use?**

### **For All Users (Recommended)**
**Use: Simple Installer**
- **File:** `MetadataGenerator-Simple-Installer.dmg`
- **Best for:** All users, zero dependencies
- **What it includes:**
  - Self-contained app with no external dependencies
  - No Xcode CLI tools required
  - No FFmpeg required
  - Simplified file analysis (format only)
  - Smallest file size

### **For End Users (Full Features)**
**Use: Universal Installer**
- **File:** `MetadataGenerator-Universal-Installer.dmg`
- **Best for:** Non-technical users who need full video/audio analysis
- **What it includes:**
  - App with bundled FFmpeg libraries
  - Automatic installation script
  - FFmpeg installation via Homebrew
  - Diagnostic and troubleshooting tools
  - Desktop shortcut creation

### **For Developers/Advanced Users**
**Use: Standard Installer**
- **File:** `MetadataGenerator-Installer.dmg`
- **Best for:** Users who already have FFmpeg or can install it manually
- **What it includes:**
  - App with bundled FFmpeg libraries
  - Manual FFmpeg installation required
  - Smaller file size

### **For Minimal Dependencies**
**Use: Static Bundle**
- **File:** `MetadataGenerator-Static-Bundle.dmg`
- **Best for:** Users who want minimal external dependencies
- **What it includes:**
  - Self-contained app
  - May still need FFmpeg for video/audio analysis
  - Smallest file size

## 📋 **Installation Comparison**

| Feature | Simple | Universal | Standard | Static |
|---------|--------|-----------|----------|--------|
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **File Size** | Smallest | Large | Medium | Small |
| **Dependencies** | None | Auto-install | FFmpeg | FFmpeg (for video/audio) |
| **Auto-Install** | ❌ | ✅ | ❌ | ❌ |
| **Troubleshooting** | ❌ | ✅ | ❌ | ❌ |
| **Best For** | All Users | End Users | Developers | Minimal Setup |

## 🚀 **Quick Start Guide**

### **Universal Installer (Recommended)**
```bash
# 1. Download the DMG
# 2. Double-click to mount
# 3. Double-click install.sh
# 4. Done! Everything is set up automatically
```

### **Standard Installer**
```bash
# 1. Download the DMG
# 2. Double-click to mount
# 3. Drag app to Applications
# 4. Install FFmpeg manually: brew install ffmpeg
```

### **Static Bundle**
```bash
# 1. Download the DMG
# 2. Double-click to mount
# 3. Drag app to Applications
# 4. May need FFmpeg for video/audio files
```

## 🔧 **Building Your Own Installer**

### **Universal Installer (Recommended)**
```bash
cd metadata-generator
./scripts/create-universal-installer.sh
```

### **Standard Installer**
```bash
cd metadata-generator
./scripts/create-dmg-installer.sh
```

### **Static Bundle**
```bash
cd metadata-generator
./scripts/create-static-bundle.sh
```

## 🎯 **Recommendations by Use Case**

### **For Company Distribution**
- **Use:** Universal Installer
- **Reason:** Zero technical knowledge required, handles all dependencies

### **For Technical Users**
- **Use:** Standard Installer
- **Reason:** Smaller size, users can handle FFmpeg installation

### **For Minimal Setup**
- **Use:** Static Bundle
- **Reason:** Smallest size, works for basic image analysis

### **For Testing/Development**
- **Use:** Static Bundle
- **Reason:** Fastest to deploy, minimal dependencies

## 🆘 **Troubleshooting**

### **Universal Installer Issues**
- Run the included diagnostic script: `./diagnose-app.sh`
- Check the troubleshooting guide in the DMG

### **Standard Installer Issues**
- Install FFmpeg: `brew install ffmpeg`
- Run the diagnostic script from the project

### **Static Bundle Issues**
- May need FFmpeg for video/audio files
- Check if the issue is FFmpeg-related

## 📊 **File Size Comparison**

- **Universal Installer:** ~50-100MB (includes all tools)
- **Standard Installer:** ~20-50MB (app + bundled libraries)
- **Static Bundle:** ~10-30MB (app only)

## 🎉 **Success Stories**

### **Universal Installer**
- ✅ Works for non-technical users
- ✅ Handles all dependencies automatically
- ✅ Includes troubleshooting tools
- ✅ Creates desktop shortcuts

### **Standard Installer**
- ✅ Smaller file size
- ✅ Good for users with existing FFmpeg
- ✅ Professional appearance

### **Static Bundle**
- ✅ Fastest deployment
- ✅ Minimal external dependencies
- ✅ Good for basic use cases

## 🚨 **Important Notes**

1. **Universal Installer** is recommended for most users
2. **Static Bundle** may still need FFmpeg for video/audio analysis
3. All installers include the app with bundled FFmpeg libraries
4. The diagnostic scripts help troubleshoot any issues
5. All installers work on both Intel and Apple Silicon Macs 