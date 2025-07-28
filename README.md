# Metadata Generator
A tool to create metadata for digital artwork editions.

## Quick Start

### Download Pre-built Binaries
The easiest way to get started is to download pre-built binaries from the [latest release](https://github.com/imetandy/Editions_metadata/releases/latest).

### Building from Source

#### GUI (Default)
The project defaults to building the GUI version. Simply run:

```bash
cargo run
```

Or build and run the GUI explicitly:
```bash
cargo run --bin MetadataGenerator
```

#### CLI Version
To build and run the command-line version:

```bash
cargo build --features cli
cargo run --bin cli -- -p /path/to/folder -m metadata.json
```

## GUI Usage

The project includes a native GUI built with `eframe`. Use the **Browse for folder** button to select the directory containing your files, fill in the artwork details and press **Generate metadata**. A JSON file named `<title>_metadata.json` will be saved to the selected folder.

## CLI Usage

The CLI version accepts the following arguments:
- `-p, --path`: Path to the folder containing files to process
- `-m, --metadata`: Optional path to a JSON metadata file

Example:
```bash
cargo run --bin cli -- -p /path/to/folder -m metadata.json
```

## Development

### Prerequisites
- Rust toolchain (install via [rustup](https://rustup.rs/))
- For cross-compilation: `rustup target add x86_64-pc-windows-msvc x86_64-apple-darwin aarch64-apple-darwin`

### Local Development
```bash
# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings
```

### Building for Distribution
Use the provided build scripts to create distribution packages:

**macOS/Linux:**
```bash
./scripts/build.sh
```

**Windows:**
```cmd
scripts\build.bat
```

This will create optimized builds for all supported platforms in the `dist/` directory.

## CI/CD Pipeline

This project uses GitHub Actions for automated testing and releases. See [CI_CD_README.md](CI_CD_README.md) for detailed information about:

- Automated testing on pull requests
- Cross-platform builds (Windows, macOS Intel, macOS ARM64)
- Automatic release creation
- Release artifact distribution

### Creating a Release
1. Update version in `metadata-generator/Cargo.toml`
2. Create and push a tag: `git tag v1.0.0 && git push origin v1.0.0`
3. The CI/CD pipeline will automatically build and create a GitHub release

## Supported Platforms

- **Windows**: x64 (CLI + GUI)
- **macOS**: Intel x64 and Apple Silicon ARM64 (CLI + GUI)
- **Linux**: x64 (CLI + GUI)

## License

MIT License - see LICENSE file for details.