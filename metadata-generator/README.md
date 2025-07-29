# Metadata Generator

A professional tool for creating metadata for digital artwork editions. Supports image, video, and audio file analysis with automatic hash generation and certificate of authenticity detection.

## Features

- **File Analysis**: Automatically extracts resolution, duration, and format information
- **Hash Generation**: Creates secure hashes for all artwork files
- **Certificate Detection**: Automatically finds certificate of authenticity PDFs
- **Multiple Formats**: Supports images (PNG, JPG, etc.), videos (MP4, MOV, etc.), and audio files
- **Professional Output**: Generates standardized JSON metadata files
- **GUI & CLI**: Both graphical and command-line interfaces available

## Installation

### Option 1: Download DMG (Recommended)

1. Download the latest `MetadataGenerator-Installer.dmg`
2. Double-click to mount the DMG
3. Drag MetadataGenerator to your Applications folder
4. **Important**: If you get a "damaged" error, right-click and select "Open"

### Option 2: Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd metadata-generator

# Build and create installer
./scripts/create-dmg-installer.sh
```

## Usage

### GUI Version
1. Launch MetadataGenerator from Applications
2. Click "Browse for folder" to select your artwork folder
3. Fill in the artwork details
4. Click "Generate metadata"

### CLI Version
```bash
cargo run --features cli -- -p /path/to/artwork/folder
```

## System Requirements

- **macOS**: 10.15 (Catalina) or later
- **Architecture**: Intel or Apple Silicon (M1/M2)
- **Dependencies**: FFmpeg (automatically installed if needed)

## Troubleshooting

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

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for detailed solutions.

### Common Issues

- **App shows as "damaged"**: Right-click → Open
- **Permission denied**: Check Security & Privacy settings
- **App won't start**: Try running from Terminal to see error messages

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
- File metadata (size, hash, resolution, duration)
- Artwork information (title, creator, description)
- Edition details (number, total editions)
- Certificate of authenticity path

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