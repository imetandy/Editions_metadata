# Metadata Generator

A professional tool for creating metadata for digital artwork editions. Supports image, video, and audio file analysis with automatic hash generation and certificate of authenticity detection.

## Features

- **File Analysis**: Automatically extracts format information and file metadata
- **Hash Generation**: Creates secure hashes for all artwork files
- **Certificate Detection**: Automatically finds certificate of authenticity PDFs
- **Multiple Formats**: Supports images (PNG, JPG, etc.), videos (MP4, MOV, etc.), and audio files
- **Professional Output**: Generates standardized JSON metadata files
- **GUI Interface**: User-friendly graphical interface with Generate and Verify tabs
- **File Verification**: Verify file integrity and detect changes using BLAKE3 hashing
- **Metadata Fingerprinting**: Fingerprint metadata files themselves for tamper detection
- **No Dependencies**: Simple installer requires no external dependencies

## Installation

### Option 1: Download DMG (Recommended)

**Simple Installer (Recommended - No dependencies):**
1. Download the latest `MetadataGenerator-Simple-Installer.dmg`
2. Double-click to mount the DMG
3. Drag MetadataGenerator to your Applications folder
4. **Important**: If you get a "damaged" error, right-click and select "Open"
5. **No Xcode CLI tools or FFmpeg required!**

**Standard Installer:**
1. Download the latest `MetadataGenerator-Installer.dmg`
2. Double-click to mount the DMG
3. Drag MetadataGenerator to your Applications folder
4. **Important**: If you get a "damaged" error, right-click and select "Open"

**Universal Installer (Includes FFmpeg installation):**
1. Download `MetadataGenerator-Universal-Installer.dmg`
2. Double-click to mount the DMG
3. Double-click `install.sh` to run the automatic installer
4. The installer will set up everything including FFmpeg

**Static Bundle (Minimal dependencies):**
1. Download `MetadataGenerator-Static-Bundle.dmg`
2. Double-click to mount the DMG
3. Drag MetadataGenerator to your Applications folder
4. May still need FFmpeg for video/audio analysis

### Option 2: Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd metadata-generator

# Choose your installer type:

# Simple installer (no dependencies required - recommended)
./scripts/create-simple-installer.sh

# Standard installer (requires FFmpeg to be installed separately)
./scripts/create-dmg-installer.sh

# Universal installer (includes FFmpeg installation script)
./scripts/create-universal-installer.sh

# Static bundle (minimal dependencies, but may still need FFmpeg for video/audio)
./scripts/create-static-bundle.sh
```

## Usage

### GUI Version
1. Launch MetadataGenerator from Applications
2. Click "Browse for folder" to select your artwork folder
3. Fill in the artwork details
4. Click "Generate metadata"

### CLI Version
```bash
# Generate metadata
cargo run --features cli -- -p /path/to/artwork/folder

# Verify files
cargo run --features cli -- --verify --path /path/to/artwork/folder --metadata-file /path/to/metadata.json
```

## System Requirements

- **macOS**: 10.15 (Catalina) or later
- **Architecture**: Intel or Apple Silicon (M1/M2)
- **Dependencies**: None (for Simple Installer) or FFmpeg (for other installers)

## Troubleshooting

### Quick Fix (Recommended)
If the app won't launch or has issues:

```bash
# Download and run the quick fix script
./scripts/quick-fix.sh
```

### FFmpeg Library Issues

If you see an error about missing `libavutil.59.dylib`:

**Quick Fix:**
```bash
./scripts/install-ffmpeg-deps.sh
```

**Manual Fix:**
```bash
brew install ffmpeg
```

### App Won't Launch?

1. **Try right-clicking and selecting "Open"**
2. **Run the diagnostic tool:**
   ```bash
   ./scripts/diagnose-app.sh
   ```
3. **Try running from Terminal:**
   ```bash
   /Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator
   ```

### Common Issues

- **App shows as "damaged"**: Right-click → Open
- **Permission denied**: Check Security & Privacy settings
- **App won't start**: Try running from Terminal to see error messages
- **App crashes when analyzing files**: Install FFmpeg with `brew install ffmpeg`

### Detailed Troubleshooting

- **End User Guide**: [END_USER_TROUBLESHOOTING.md](END_USER_TROUBLESHOOTING.md)
- **Installation Guide**: [INSTALLATION_GUIDE.md](INSTALLATION_GUIDE.md)
- **Developer Troubleshooting**: [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- **Verification Guide**: [VERIFICATION_GUIDE.md](VERIFICATION_GUIDE.md)

## File Structure

Your artwork folder should contain:
```
artwork-folder/
├── image1.jpg
├── video1.mp4
├── audio1.wav
└── certificate/
    └── certificate_of_authenticity.pdf
```

## Output

The tool generates a JSON file with:
- File metadata (size, hash, format)
- Artwork information (title, creator, description)
- Edition details (number, total editions)
- Certificate of authenticity path

### Example JSON Output:
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

## Development

### Building
```bash
# GUI version
cargo build --release --features gui

# CLI version
cargo build --release --features cli
```

### Testing
```bash
./scripts/test-app.sh
```

## License

MIT License - see LICENSE file for details.

## Support

For issues and questions:
1. Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md)
2. Try running from Terminal to see detailed error messages
3. Include your macOS version and full error output 