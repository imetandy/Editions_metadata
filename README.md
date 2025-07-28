# Metadata Generator
A tool to create metadata for digital artwork editions.

## GUI Usage

The project now includes a native GUI built with `eframe`. Run it with:

```bash
cargo run --bin gui
```

Use the **Browse for folder** button to select the directory containing your
files, fill in the artwork details and press **Generate metadata**. A JSON file
named `<title>_metadata.json` will be saved to the selected folder.