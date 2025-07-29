# Verification Feature Guide

## Overview

The MetadataGenerator now includes a powerful verification feature that allows you to:

1. **Fingerprint the metadata file itself** using BLAKE3 hashing
2. **Verify all artwork files** against their recorded hashes in the metadata
3. **Hash and verify certificates of authenticity** to ensure they haven't been modified
4. **Detect any changes or corruption** in your digital artwork files and certificates
5. **Ensure data integrity** for your artwork editions

## How It Works

### Metadata File Fingerprinting
- The metadata file itself is hashed using BLAKE3
- This creates a unique fingerprint that can detect if the metadata file has been modified
- The fingerprint is included in verification reports

### File Verification
- Each artwork file listed in the metadata is re-hashed using BLAKE3
- The new hash is compared against the hash recorded in the metadata
- Any mismatch indicates the file has been modified, corrupted, or is missing

### Certificate Verification
- If a certificate of authenticity is found, it is automatically hashed during metadata generation
- The certificate hash is stored in the metadata file
- During verification, the certificate is re-hashed and compared against the stored hash
- Any modification to the certificate will be detected

## GUI Usage

### Step 1: Open the Verify Tab
1. Launch MetadataGenerator
2. Click on the "Verify" tab at the top of the application

### Step 2: Select Files
1. **Browse for metadata file**: Click "Browse for metadata file" and select your `*_metadata.json` file
2. **Browse for base folder**: Click "Browse for base folder" and select the folder containing your artwork files
   - *Note: The base folder is automatically detected as the same directory as the metadata file*

### Step 3: Start Verification
1. Click "Verify files" to begin the verification process
2. Watch the progress as each file is verified
3. View the results when verification is complete

### Step 4: Review Results
The verification results show:
- **Metadata file hash**: The fingerprint of the metadata file itself
- **Total files**: Number of files checked
- **Valid files**: Number of files with matching hashes
- **Invalid files**: Number of files with mismatched or missing hashes
- **Certificate status**: ✅ Valid, ❌ Invalid, or ℹ️ No certificate found
- **Certificate hash**: The hash of the certificate file (if present)
- **Overall status**: ✅ All valid or ❌ Some invalid
- **Detailed results**: Scrollable list showing the status of each file

## CLI Usage

### Basic Verification
```bash
cargo run --bin cli --features cli -- --verify --path /path/to/artwork/folder --metadata-file /path/to/metadata.json
```

### Example
```bash
# Verify files in the current directory against Test_Artwork_metadata.json
cargo run --bin cli --features cli -- --verify --path . --metadata-file Test_Artwork_metadata.json
```

### CLI Output
The CLI provides detailed output including:
- Progress updates for each file being verified
- Summary statistics
- Detailed results for each file
- Clear success/failure indicators

## Verification Results

### Valid Files
- ✅ File hash matches the recorded hash in metadata
- File is unchanged and authentic

### Invalid Files
- ❌ File hash doesn't match the recorded hash
- Possible causes:
  - File has been modified
  - File has been corrupted
  - File is missing
  - File path is incorrect

### Missing Files
- ❌ File not found in the specified location
- Check if files have been moved or deleted

### Invalid Certificates
- ❌ Certificate hash doesn't match the recorded hash
- Possible causes:
  - Certificate has been modified
  - Certificate has been corrupted
  - Certificate file is missing
  - Certificate path is incorrect

## Use Cases

### 1. Data Integrity Verification
- Verify that artwork files haven't been corrupted during transfer
- Ensure files are exactly as they were when metadata was generated

### 2. Authenticity Verification
- Confirm that artwork files are authentic and haven't been modified
- Detect any unauthorized changes to digital artwork
- Verify that certificates of authenticity are genuine and unmodified

### 3. Archive Verification
- Periodically verify archived artwork collections
- Ensure long-term storage integrity

### 4. Distribution Verification
- Verify files before distribution to clients
- Ensure recipients receive authentic, unmodified files

## Technical Details

### Hash Algorithm
- **BLAKE3**: Fast, secure cryptographic hash function
- **Collision resistant**: Extremely unlikely for two different files to have the same hash
- **Deterministic**: Same file always produces the same hash

### Performance
- **Fast verification**: BLAKE3 is optimized for speed
- **Progress reporting**: Real-time progress updates for large files
- **Memory efficient**: Uses memory mapping for large files

### Security
- **Cryptographic strength**: BLAKE3 provides strong security guarantees
- **Tamper detection**: Any modification to files or certificates will be detected
- **Metadata protection**: Metadata file itself is fingerprinted
- **Certificate integrity**: Certificates of authenticity are protected from tampering

## Troubleshooting

### Common Issues

#### "File not found" errors
- Check that the base folder path is correct
- Ensure artwork files are in the expected location
- Verify file names match exactly (case-sensitive)

#### "Invalid hash" errors
- File has been modified since metadata generation
- File may be corrupted
- Re-generate metadata if files are intentionally modified

#### "Invalid certificate" errors
- Certificate has been modified since metadata generation
- Certificate may be corrupted
- Re-generate metadata if certificate is intentionally modified

#### "Metadata file not found" errors
- Check the path to the metadata file
- Ensure the file has a `.json` extension
- Verify the metadata file is valid JSON

### Best Practices

1. **Regular verification**: Verify files periodically, especially after transfers
2. **Backup verification**: Verify files after creating backups
3. **Distribution verification**: Always verify before distributing artwork
4. **Archive verification**: Verify archived collections regularly

## Integration with Existing Workflow

### After Generation
1. Generate metadata for your artwork
2. Immediately verify the generated metadata
3. Store both the artwork files and metadata securely

### Before Distribution
1. Verify all files against the metadata
2. Only distribute if verification passes
3. Include metadata file with distributed artwork

### Long-term Storage
1. Store metadata file alongside artwork files
2. Periodically verify the entire collection
3. Keep verification reports for audit purposes

## Example Workflow

```bash
# 1. Generate metadata
cargo run --bin cli --features cli -- --path ./artwork --metadata metadata_template.json

# 2. Verify immediately after generation
cargo run --bin cli --features cli -- --verify --path ./artwork --metadata-file ./artwork/Artwork_Title_metadata.json

# 3. If verification passes, proceed with distribution
# If verification fails, investigate and re-generate metadata if needed
```

The verification feature ensures the integrity and authenticity of your digital artwork and certificates of authenticity throughout their lifecycle, from creation to distribution to long-term storage. 