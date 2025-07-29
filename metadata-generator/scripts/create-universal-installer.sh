#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== MetadataGenerator Universal Installer Creator ===${NC}"
echo -e "${YELLOW}This creates a DMG with all prerequisites included${NC}"

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

# Bundle FFmpeg libraries with static linking approach
echo -e "${YELLOW}Bundling FFmpeg libraries...${NC}"
if [ -f "scripts/bundle-ffmpeg.sh" ]; then
    chmod +x "scripts/bundle-ffmpeg.sh"
    ./scripts/bundle-ffmpeg.sh
    if [ $? -ne 0 ]; then
        echo -e "${RED}FFmpeg bundling failed!${NC}"
        exit 1
    fi
else
    echo -e "${RED}Error: bundle-ffmpeg.sh not found. Cannot create installer without FFmpeg bundling.${NC}"
    exit 1
fi

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

# Remove quarantine attribute
echo -e "${YELLOW}Removing quarantine attribute...${NC}"
xattr -cr "${APP_BUNDLE}" 2>/dev/null || true

echo -e "${GREEN}App bundle created: ${APP_BUNDLE}${NC}"

# Create installer package directory
INSTALLER_DIR="MetadataGenerator-Universal-Installer"
mkdir -p "${INSTALLER_DIR}"

# Copy the app bundle
echo -e "${YELLOW}Copying app bundle to installer directory...${NC}"
cp -R "${APP_BUNDLE}" "${INSTALLER_DIR}/"

# Create installation script
echo -e "${YELLOW}Creating installation script...${NC}"
cat > "${INSTALLER_DIR}/install.sh" << 'EOF'
#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== MetadataGenerator Universal Installer ===${NC}"

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo -e "${RED}Error: Please don't run this as root${NC}"
    exit 1
fi

# Check macOS version
MACOS_VERSION=$(sw_vers -productVersion)
if [[ "$MACOS_VERSION" < "10.15" ]]; then
    echo -e "${RED}Error: macOS 10.15 (Catalina) or later is required${NC}"
    exit 1
fi

echo -e "${GREEN}✓ macOS version is supported${NC}"

# Install the app
echo -e "${YELLOW}Installing MetadataGenerator...${NC}"
if [ -d "/Applications/MetadataGenerator.app" ]; then
    echo -e "${YELLOW}Removing existing installation...${NC}"
    rm -rf "/Applications/MetadataGenerator.app"
fi

cp -R "MetadataGenerator.app" "/Applications/"
chmod +x "/Applications/MetadataGenerator.app/Contents/MacOS/MetadataGenerator"
xattr -cr "/Applications/MetadataGenerator.app" 2>/dev/null || true

echo -e "${GREEN}✓ MetadataGenerator installed successfully${NC}"

# Install FFmpeg if not present
echo -e "${YELLOW}Checking FFmpeg installation...${NC}"
if command -v ffmpeg &> /dev/null; then
    echo -e "${GREEN}✓ FFmpeg is already installed${NC}"
else
    echo -e "${YELLOW}FFmpeg not found. Installing...${NC}"
    
    # Try to install via Homebrew
    if command -v brew &> /dev/null; then
        echo -e "${YELLOW}Installing FFmpeg via Homebrew...${NC}"
        brew install ffmpeg
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ FFmpeg installed successfully via Homebrew${NC}"
        else
            echo -e "${RED}✗ Failed to install FFmpeg via Homebrew${NC}"
        fi
    else
        echo -e "${YELLOW}Homebrew not found. Installing Homebrew first...${NC}"
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ Homebrew installed${NC}"
            echo -e "${YELLOW}Installing FFmpeg...${NC}"
            brew install ffmpeg
            if [ $? -eq 0 ]; then
                echo -e "${GREEN}✓ FFmpeg installed successfully${NC}"
            else
                echo -e "${RED}✗ Failed to install FFmpeg${NC}"
            fi
        else
            echo -e "${RED}✗ Failed to install Homebrew${NC}"
        fi
    fi
fi

# Create desktop shortcut
echo -e "${YELLOW}Creating desktop shortcut...${NC}"
if [ -d "$HOME/Desktop" ]; then
    ln -sf "/Applications/MetadataGenerator.app" "$HOME/Desktop/MetadataGenerator"
    echo -e "${GREEN}✓ Desktop shortcut created${NC}"
fi

echo -e "${GREEN}=== Installation Complete! ===${NC}"
echo -e "${BLUE}You can now launch MetadataGenerator from Applications or your Desktop${NC}"
echo -e "${YELLOW}If you encounter any issues, run the diagnostic script:${NC}"
echo -e "${BLUE}./diagnose-app.sh${NC}"

# Open Applications folder
open "/Applications"
EOF

chmod +x "${INSTALLER_DIR}/install.sh"

# Create diagnostic script
echo -e "${YELLOW}Creating diagnostic script...${NC}"
cp "scripts/diagnose-app.sh" "${INSTALLER_DIR}/"
chmod +x "${INSTALLER_DIR}/diagnose-app.sh"

# Create quick fix script
echo -e "${YELLOW}Creating quick fix script...${NC}"
cp "scripts/quick-fix.sh" "${INSTALLER_DIR}/"
chmod +x "${INSTALLER_DIR}/quick-fix.sh"

# Create README
echo -e "${YELLOW}Creating README...${NC}"
cat > "${INSTALLER_DIR}/README.txt" << 'EOF'
MetadataGenerator Universal Installer
====================================

This installer includes everything needed to run MetadataGenerator on macOS.

INSTALLATION:
1. Double-click "install.sh" to run the installer
2. Follow the prompts to install the app and dependencies
3. The app will be installed to /Applications
4. A desktop shortcut will be created

TROUBLESHOOTING:
- If the app won't launch, run: ./diagnose-app.sh
- For quick fixes, run: ./quick-fix.sh
- Check the troubleshooting guides for more help

SYSTEM REQUIREMENTS:
- macOS 10.15 (Catalina) or later
- Intel or Apple Silicon Mac

WHAT'S INCLUDED:
- MetadataGenerator.app (with bundled FFmpeg libraries)
- Automatic FFmpeg installation via Homebrew
- Diagnostic and troubleshooting tools
- Installation script

For more help, see the troubleshooting guides in the project repository.
EOF

# Create DMG installer
DMG_NAME="${APP_NAME}-Universal-Installer"
echo -e "${YELLOW}Creating universal DMG installer...${NC}"

# Clean up any existing DMG files
if [ -f "${DMG_NAME}.dmg" ]; then
    echo -e "${YELLOW}Removing existing DMG file...${NC}"
    rm -f "${DMG_NAME}.dmg"
fi

# Also clean up any temporary DMG files
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
  --volname "MetadataGenerator Universal Installer" \
  ${VOLUME_ICON} \
  --window-pos 200 120 \
  --window-size 800 500 \
  --icon-size 100 \
  --icon "${APP_NAME}.app" 200 190 \
  --icon "install.sh" 400 190 \
  --icon "README.txt" 600 190 \
  --hide-extension "${APP_NAME}.app" \
  --hide-extension "install.sh" \
  --hide-extension "README.txt" \
  --app-drop-link 200 350 \
  --no-internet-enable \
  --format UDZO \
  "${DMG_NAME}.dmg" \
  "${INSTALLER_DIR}"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Universal DMG installer created successfully: ${DMG_NAME}.dmg${NC}"
    echo -e "${BLUE}File size: $(du -h "${DMG_NAME}.dmg" | cut -f1)${NC}"
    
    # Remove quarantine from DMG
    echo -e "${YELLOW}Removing quarantine from DMG...${NC}"
    xattr -cr "${DMG_NAME}.dmg" 2>/dev/null || true
    
    echo -e "${GREEN}=== Universal installer ready! ===${NC}"
    echo -e "${BLUE}This DMG includes:${NC}"
    echo -e "  ✓ MetadataGenerator.app with bundled FFmpeg libraries"
    echo -e "  ✓ Automatic installation script"
    echo -e "  ✓ Diagnostic and troubleshooting tools"
    echo -e "  ✓ README with instructions"
    echo -e "${YELLOW}Users just need to run install.sh from the DMG!${NC}"
else
    echo -e "${RED}✗ Failed to create universal DMG installer${NC}"
    exit 1
fi

# Clean up temporary files
echo -e "${YELLOW}Cleaning up temporary files...${NC}"
rm -rf "${APP_BUNDLE}"
rm -rf "${INSTALLER_DIR}"

echo -e "${GREEN}=== Done! ===${NC}"
echo -e "${BLUE}Your universal installer is ready: ${DMG_NAME}.dmg${NC}" 