#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== MetadataGenerator DMG Installer Creator ===${NC}"

# Check if create-dmg is installed
if ! command -v create-dmg &> /dev/null; then
    echo -e "${RED}Error: create-dmg is not installed. Please run: brew install create-dmg${NC}"
    exit 1
fi

# Build the release version
echo -e "${YELLOW}Building release version...${NC}"
cargo build --release --features gui

if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

# Create app bundle structure
APP_NAME="MetadataGenerator"
APP_BUNDLE="${APP_NAME}.app"
CONTENTS_DIR="${APP_BUNDLE}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo -e "${YELLOW}Creating app bundle structure...${NC}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy the executable
echo -e "${YELLOW}Copying executable...${NC}"
cp "target/release/${APP_NAME}" "${MACOS_DIR}/${APP_NAME}"

# Create Info.plist
echo -e "${YELLOW}Creating Info.plist...${NC}"
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.andrewrobinson.${APP_NAME}</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>0.1.1</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.1</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>CFBundleDisplayName</key>
    <string>Metadata Generator</string>
    <key>CFBundleGetInfoString</key>
    <string>A tool to create metadata for digital artwork editions</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
</dict>
</plist>
EOF

# Copy app icon if it exists
if [ -f "assets/icons/AppIcon.icns" ]; then
    echo -e "${YELLOW}Adding app icon...${NC}"
    cp "assets/icons/AppIcon.icns" "${RESOURCES_DIR}/AppIcon.icns"
elif [ -f "assets/icons/AppIcon.png" ]; then
    echo -e "${YELLOW}Adding app icon (PNG format)...${NC}"
    cp "assets/icons/AppIcon.png" "${RESOURCES_DIR}/AppIcon.png"
fi

# Make the app bundle executable
chmod +x "${MACOS_DIR}/${APP_NAME}"

echo -e "${GREEN}App bundle created: ${APP_BUNDLE}${NC}"

# Create DMG installer
DMG_NAME="${APP_NAME}-Installer"
echo -e "${YELLOW}Creating DMG installer...${NC}"

# Clean up any existing DMG files
if [ -f "${DMG_NAME}.dmg" ]; then
    echo -e "${YELLOW}Removing existing DMG file...${NC}"
    rm -f "${DMG_NAME}.dmg"
fi

# Also clean up any temporary DMG files that might exist
rm -f "rw.*.${DMG_NAME}.dmg" 2>/dev/null || true

# Use custom volume icon if available
VOLUME_ICON=""
if [ -f "assets/icons/VolumeIcon.icns" ]; then
    VOLUME_ICON="--volicon assets/icons/VolumeIcon.icns"
    echo -e "${YELLOW}Using custom volume icon...${NC}"
elif [ -f "assets/icons/VolumeIcon.png" ]; then
    VOLUME_ICON="--volicon assets/icons/VolumeIcon.png"
    echo -e "${YELLOW}Using custom volume icon (PNG)...${NC}"
fi

create-dmg \
  --volname "${APP_NAME}" \
  ${VOLUME_ICON} \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "${APP_NAME}.app" 200 190 \
  --hide-extension "${APP_NAME}.app" \
  --app-drop-link 600 185 \
  --no-internet-enable \
  --format UDZO \
  "${DMG_NAME}.dmg" \
  "${APP_BUNDLE}"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ DMG installer created successfully: ${DMG_NAME}.dmg${NC}"
    echo -e "${BLUE}File size: $(du -h "${DMG_NAME}.dmg" | cut -f1)${NC}"
    echo -e "${YELLOW}You can now distribute ${DMG_NAME}.dmg to your company!${NC}"
else
    echo -e "${RED}✗ Failed to create DMG installer${NC}"
    exit 1
fi

# Clean up temporary app bundle
echo -e "${YELLOW}Cleaning up temporary files...${NC}"
rm -rf "${APP_BUNDLE}"

echo -e "${GREEN}=== Done! ===${NC}"
echo -e "${BLUE}Your professional installer is ready: ${DMG_NAME}.dmg${NC}" 