#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Icon Conversion Helper ===${NC}"

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo -e "${YELLOW}ImageMagick not found. Installing...${NC}"
    if command -v brew &> /dev/null; then
        brew install imagemagick
    else
        echo -e "${RED}Please install ImageMagick manually:${NC}"
        echo -e "${YELLOW}https://imagemagick.org/script/download.php#macosx${NC}"
        exit 1
    fi
fi

# Check if source file exists
if [ ! -f "assets/icons/AppIcon.png" ]; then
    echo -e "${RED}Error: assets/icons/AppIcon.png not found${NC}"
    echo -e "${YELLOW}Please create a 1024x1024 PNG icon and save it as:${NC}"
    echo -e "${BLUE}assets/icons/AppIcon.png${NC}"
    exit 1
fi

echo -e "${YELLOW}Converting PNG to ICNS...${NC}"

# Create iconset directory
mkdir -p "assets/icons/AppIcon.iconset"

# Generate all required icon sizes
echo -e "${YELLOW}Generating icon sizes...${NC}"
magick "assets/icons/AppIcon.png" -resize 16x16 "assets/icons/AppIcon.iconset/icon_16x16.png"
magick "assets/icons/AppIcon.png" -resize 32x32 "assets/icons/AppIcon.iconset/icon_16x16@2x.png"
magick "assets/icons/AppIcon.png" -resize 32x32 "assets/icons/AppIcon.iconset/icon_32x32.png"
magick "assets/icons/AppIcon.png" -resize 64x64 "assets/icons/AppIcon.iconset/icon_32x32@2x.png"
magick "assets/icons/AppIcon.png" -resize 128x128 "assets/icons/AppIcon.iconset/icon_128x128.png"
magick "assets/icons/AppIcon.png" -resize 256x256 "assets/icons/AppIcon.iconset/icon_128x128@2x.png"
magick "assets/icons/AppIcon.png" -resize 256x256 "assets/icons/AppIcon.iconset/icon_256x256.png"
magick "assets/icons/AppIcon.png" -resize 512x512 "assets/icons/AppIcon.iconset/icon_256x256@2x.png"
magick "assets/icons/AppIcon.png" -resize 512x512 "assets/icons/AppIcon.iconset/icon_512x512.png"
magick "assets/icons/AppIcon.png" -resize 1024x1024 "assets/icons/AppIcon.iconset/icon_512x512@2x.png"

# Create ICNS file using iconutil
echo -e "${YELLOW}Creating ICNS file...${NC}"
iconutil -c icns "assets/icons/AppIcon.iconset" -o "assets/icons/AppIcon.icns"

# Clean up iconset directory
rm -rf "assets/icons/AppIcon.iconset"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ ICNS file created: assets/icons/AppIcon.icns${NC}"
else
    echo -e "${RED}✗ Failed to create ICNS file${NC}"
    exit 1
fi

# Create volume icon
echo -e "${YELLOW}Creating volume icon...${NC}"
convert "assets/icons/AppIcon.png" -resize 512x512 "assets/icons/VolumeIcon.png"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Volume icon created: assets/icons/VolumeIcon.png${NC}"
else
    echo -e "${RED}✗ Failed to create volume icon${NC}"
fi

echo -e "${GREEN}=== Icon conversion complete! ===${NC}"
echo -e "${BLUE}You can now run: ./scripts/create-dmg-installer.sh${NC}" 