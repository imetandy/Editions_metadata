# Simple Installer Guide

## 🎯 **Simple Installer - No Dependencies Required**

The **Simple Installer** is the recommended option for most users. It requires **no external dependencies** and works immediately after installation.

### **What's Different?**

- ✅ **No Xcode CLI tools required**
- ✅ **No FFmpeg required**
- ✅ **No Homebrew required**
- ✅ **No external libraries needed**
- ✅ **Works on any macOS 10.15+ system**

### **What's Simplified?**

- 📁 **File Analysis**: Basic format detection (no resolution/duration for video/audio)
- 🔍 **Hash Generation**: Full BLAKE3 hashing for all files
- 📄 **Certificate Detection**: Still finds certificate of authenticity PDFs
- 📊 **JSON Output**: Clean metadata without resolution/duration fields

## 🚀 **Installation**

### **Step 1: Download**
Download `MetadataGenerator-Simple-Installer.dmg`

### **Step 2: Install**
1. Double-click the DMG to mount it
2. Drag `MetadataGenerator` to your Applications folder
3. Eject the DMG

### **Step 3: Launch**
1. Go to Applications and double-click `MetadataGenerator`
2. If you get a "damaged" error, right-click and select "Open"

## 📋 **What You Get**

The Simple Installer provides:
- **File hashing** for all artwork files
- **Format detection** (JPG, PNG, MP4, MOV, etc.)
- **File size** and metadata
- **Certificate of authenticity** detection
- **Professional JSON output**

## 🔧 **Building Your Own Simple Installer**

```bash
cd metadata-generator
./scripts/create-simple-installer.sh
```

This creates `MetadataGenerator-Simple-Installer.dmg` with no dependencies.

## 📊 **JSON Output Example**

```json
{
  "artwork_id": "ART001",
  "artwork_title": "Digital Sunset",
  "artwork_creator": "Artist Name",
  "year_of_creation": 2024,
  "artwork_files": [
    {
      "path": "./image1.jpg",
      "file_name": "image1.jpg",
      "file_hash": "abc123...",
      "file_size": 2048576,
      "format": "JPG"
    }
  ]
}
```

## 🆚 **Comparison with Other Installers**

| Feature | Simple | Standard | Universal | Static |
|---------|--------|----------|-----------|--------|
| **Dependencies** | None | FFmpeg | Auto-install | FFmpeg |
| **Xcode CLI** | ❌ | ❌ | ❌ | ❌ |
| **Resolution/Duration** | ❌ | ✅ | ✅ | ✅ |
| **File Size** | Smallest | Medium | Largest | Small |
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

## 🎯 **When to Use Simple Installer**

**Use Simple Installer when:**
- You want zero dependencies
- You don't need video/audio resolution/duration
- You're distributing to non-technical users
- You want the smallest file size
- You want the fastest installation

**Use other installers when:**
- You need video/audio resolution/duration analysis
- You're okay with installing FFmpeg
- You need full media file analysis

## 🚨 **Troubleshooting**

### **App Won't Launch**
1. Right-click the app and select "Open"
2. Check System Preferences → Security & Privacy
3. Run from Terminal to see error messages:
   ```bash
   /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
   ```

### **Still Having Issues?**
The Simple Installer should work on any macOS 10.15+ system without any additional setup. If you're still having issues, it's likely a macOS security setting or permission issue.

## ✅ **Success Stories**

- ✅ Works on fresh macOS installations
- ✅ No developer tools required
- ✅ No package managers needed
- ✅ Instant deployment
- ✅ Zero configuration

The Simple Installer is the perfect solution for users who want a professional metadata generation tool without any technical setup! 