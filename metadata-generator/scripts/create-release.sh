#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== GitHub Release Creator ===${NC}"

# Check if version is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Please provide a version number${NC}"
    echo -e "${YELLOW}Usage: ./scripts/create-release.sh v0.1.1${NC}"
    exit 1
fi

VERSION=$1

# Validate version format
if [[ ! $VERSION =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Version must be in format vX.Y.Z (e.g., v0.1.1)${NC}"
    exit 1
fi

echo -e "${YELLOW}Creating release for version: ${VERSION}${NC}"

# Build the installer
echo -e "${YELLOW}Building DMG installer...${NC}"
./scripts/create-dmg-installer.sh

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to build installer${NC}"
    exit 1
fi

# Create zip backup
echo -e "${YELLOW}Creating zip backup...${NC}"
./scripts/build-app.sh
zip -r MetadataGenerator-macOS.zip MetadataGenerator.app

# Create git tag
echo -e "${YELLOW}Creating git tag...${NC}"
git tag $VERSION

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to create git tag${NC}"
    exit 1
fi

# Push tag
echo -e "${YELLOW}Pushing tag to GitHub...${NC}"
git push origin $VERSION

if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to push tag${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Release created successfully!${NC}"
echo -e "${BLUE}Version: ${VERSION}${NC}"
echo -e "${BLUE}Files ready:${NC}"
echo -e "  - MetadataGenerator-Installer.dmg"
echo -e "  - MetadataGenerator-macOS.zip"
echo -e ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "1. Go to: https://github.com/imetandy/Editions_metadata/releases"
echo -e "2. Edit the release description"
echo -e "3. Upload the DMG and ZIP files"
echo -e "4. Publish the release"
echo -e ""
echo -e "${GREEN}Or use GitHub Actions for automated releases!${NC}" 