#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== MetadataGenerator App Bundle Builder ===${NC}"

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
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
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

# Set proper permissions
echo -e "${YELLOW}Setting permissions...${NC}"
chmod +x "${MACOS_DIR}/${APP_NAME}"
chmod 755 "${APP_BUNDLE}"
chmod 755 "${CONTENTS_DIR}"
chmod 755 "${MACOS_DIR}"
chmod 755 "${RESOURCES_DIR}"
chmod 644 "${CONTENTS_DIR}/Info.plist"

# Remove quarantine attribute (common cause of "damaged" apps)
echo -e "${YELLOW}Removing quarantine attribute...${NC}"
xattr -cr "${APP_BUNDLE}" 2>/dev/null || true

# Test the app bundle
echo -e "${YELLOW}Testing app bundle...${NC}"
if [ -f "${MACOS_DIR}/${APP_NAME}" ]; then
    echo -e "${GREEN}✓ Executable exists and is executable${NC}"
else
    echo -e "${RED}✗ Executable not found!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ App bundle created: ${APP_BUNDLE}${NC}"
echo -e "${BLUE}You can now distribute ${APP_BUNDLE} to other macOS users${NC}"
echo -e "${YELLOW}To test: double-click ${APP_BUNDLE}${NC}" 