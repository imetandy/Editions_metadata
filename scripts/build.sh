#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to build for a specific target
build_target() {
    local target=$1
    local feature=$2
    local output_name=$3
    
    print_status "Building for target: $target with feature: $feature"
    
    if ! cargo build --release --target "$target" --features "$feature"; then
        print_error "Failed to build for $target with $feature"
        return 1
    fi
    
    # Copy the built binary to the output directory
    local target_dir="target/$target/release"
    local output_dir="dist"
    
    mkdir -p "$output_dir"
    
    if [[ "$target" == *"windows"* ]]; then
        if [[ "$feature" == "cli" ]]; then
            cp "$target_dir/cli.exe" "$output_dir/cli-$target.exe"
        else
            cp "$target_dir/MetadataGenerator.exe" "$output_dir/MetadataGenerator-$target.exe"
        fi
    else
        if [[ "$feature" == "cli" ]]; then
            cp "$target_dir/cli" "$output_dir/cli-$target"
        else
            cp "$target_dir/MetadataGenerator" "$output_dir/MetadataGenerator-$target"
        fi
    fi
    
    print_status "Successfully built $output_name for $target"
}

# Main script
main() {
    print_status "Starting build process..."
    
    # Check if we're in the right directory
    if [[ ! -f "metadata-generator/Cargo.toml" ]]; then
        print_error "Please run this script from the project root directory"
        exit 1
    fi
    
    # Change to the metadata-generator directory
    cd metadata-generator
    
    # Clean previous builds
    print_status "Cleaning previous builds..."
    cargo clean
    
    # Create output directory
    mkdir -p ../dist
    
    # Define targets to build
    local targets=(
        "x86_64-unknown-linux-gnu"
        "x86_64-pc-windows-msvc"
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )
    
    # Build CLI version for all targets
    print_status "Building CLI version..."
    for target in "${targets[@]}"; do
        if build_target "$target" "cli" "cli"; then
            print_status "CLI build successful for $target"
        else
            print_warning "CLI build failed for $target"
        fi
    done
    
    # Build GUI version for all targets
    print_status "Building GUI version..."
    for target in "${targets[@]}"; do
        if build_target "$target" "gui" "MetadataGenerator"; then
            print_status "GUI build successful for $target"
        else
            print_warning "GUI build failed for $target"
        fi
    done
    
    # Create zip files for distribution
    print_status "Creating distribution packages..."
    cd ../dist
    
    # Create zip files for each platform
    for target in "${targets[@]}"; do
        local platform_name=""
        case "$target" in
            "x86_64-unknown-linux-gnu")
                platform_name="linux-x64"
                ;;
            "x86_64-pc-windows-msvc")
                platform_name="windows-x64"
                ;;
            "x86_64-apple-darwin")
                platform_name="macos-x64"
                ;;
            "aarch64-apple-darwin")
                platform_name="macos-arm64"
                ;;
        esac
        
        if [[ -f "cli-$target" ]] || [[ -f "cli-$target.exe" ]]; then
            if [[ "$target" == *"windows"* ]]; then
                zip -r "MetadataGenerator-$platform_name.zip" "cli-$target.exe" "MetadataGenerator-$target.exe" 2>/dev/null || true
            else
                zip -r "MetadataGenerator-$platform_name.zip" "cli-$target" "MetadataGenerator-$target" 2>/dev/null || true
            fi
            print_status "Created package: MetadataGenerator-$platform_name.zip"
        fi
    done
    
    print_status "Build process completed!"
    print_status "Distribution packages are available in the 'dist' directory"
}

# Run main function
main "$@" 