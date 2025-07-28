#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' metadata-generator/Cargo.toml | cut -d'"' -f2
}

# Function to update version in Cargo.toml
update_version() {
    local new_version=$1
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" metadata-generator/Cargo.toml
    else
        # Linux
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" metadata-generator/Cargo.toml
    fi
}

# Function to validate version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        print_error "Invalid version format. Use semantic versioning (e.g., 1.0.0)"
        exit 1
    fi
}

# Function to check if git is clean
check_git_status() {
    if [[ -n $(git status --porcelain) ]]; then
        print_error "Git working directory is not clean. Please commit or stash your changes."
        exit 1
    fi
}

# Function to check if tag already exists
check_tag_exists() {
    local version=$1
    if git tag -l | grep -q "^v$version$"; then
        print_error "Tag v$version already exists"
        exit 1
    fi
}

# Main release function
main() {
    local new_version=""
    local dry_run=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --version)
                new_version="$2"
                shift 2
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 --version <version> [--dry-run]"
                echo ""
                echo "Options:"
                echo "  --version <version>  New version to release (e.g., 1.0.0)"
                echo "  --dry-run           Show what would be done without making changes"
                echo "  --help, -h          Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
    
    # Check if version is provided
    if [[ -z "$new_version" ]]; then
        print_error "Version is required. Use --version <version>"
        echo "Use --help for usage information"
        exit 1
    fi
    
    # Validate version format
    validate_version "$new_version"
    
    # Get current version
    local current_version=$(get_current_version)
    print_status "Current version: $current_version"
    print_status "New version: $new_version"
    
    if [[ "$dry_run" == true ]]; then
        print_warning "DRY RUN MODE - No changes will be made"
        print_step "Would update version in metadata-generator/Cargo.toml"
        print_step "Would commit changes with message: 'chore: bump version to $new_version'"
        print_step "Would create tag: v$new_version"
        print_step "Would push changes and tag to origin"
        exit 0
    fi
    
    # Check git status
    check_git_status
    
    # Check if tag already exists
    check_tag_exists "$new_version"
    
    # Update version in Cargo.toml
    print_step "Updating version in Cargo.toml..."
    update_version "$new_version"
    
    # Commit changes
    print_step "Committing version change..."
    git add metadata-generator/Cargo.toml
    git commit -m "chore: bump version to $new_version"
    
    # Create tag
    print_step "Creating tag v$new_version..."
    git tag "v$new_version"
    
    # Push changes and tag
    print_step "Pushing changes and tag..."
    git push origin master
    git push origin "v$new_version"
    
    print_status "Release v$new_version has been created!"
    print_status "The CI/CD pipeline will automatically build and create a GitHub release."
    print_status "You can monitor the progress at: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^/]*\/[^/]*\).*/\1/')/actions"
}

# Run main function
main "$@" 