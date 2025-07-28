# Metadata Generator
A tool to create metadata for digital artwork editions.

## Building and Running

### GUI (Default)
The project defaults to building the GUI version. Simply run:

```bash
cargo run
```

Or build and run the GUI explicitly:
```bash
cargo run --bin MetadataGenerator
```

### CLI Version
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