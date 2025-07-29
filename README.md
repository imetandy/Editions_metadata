# Metadata Generator

A simple tool to create metadata for digital artwork editions.

## Quick Start

### Download and Install

1. **Download** the installer from the [latest release](https://github.com/imetandy/Editions_metadata/releases/latest)
2. **Double-click** the DMG file to open it
3. **Drag** the app to your Applications folder
4. **Launch** the app from Applications

That's it! No complicated setup needed.

### Building from Source

If you want to build it yourself:

```bash
# Build and run the app
cargo run
```

## How to Use

### Using the App

1. **Open** the MetadataGenerator app
2. **Click** "Browse for folder" to select your artwork folder
3. **Fill in** the artwork details (title, artist, etc.)
4. **Click** "Generate metadata"
5. **Find** your metadata file in the selected folder

The app will create a JSON file with all your artwork information.

### Using Command Line

If you prefer the command line:

```bash
cargo run --bin cli -- -p /path/to/folder -m metadata.json
```

## System Requirements

- **macOS** 10.15 or later
- **No other software** needed - everything is included

## For Developers

### Prerequisites
- Install Rust from [rustup.rs](https://rustup.rs/)

### Building
```bash
# Build the app
cargo build

# Run tests
cargo test

# Build for release
cargo build --release
```

### Creating a Release
```bash
# Build the installer
cd metadata-generator
./scripts/create-dmg-installer.sh

# Create a release
./scripts/create-release.sh v1.0.0
```

## License

MIT License - see LICENSE file for details.