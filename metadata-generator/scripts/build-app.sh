#!/bin/bash

# Build the release version
echo "Building release version..."
cargo build --release --features gui

# Create app bundle structure
APP_NAME="MetadataGenerator"
APP_BUNDLE="${APP_NAME}.app"
CONTENTS_DIR="${APP_BUNDLE}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "Creating app bundle structure..."
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy the executable
echo "Copying executable..."
cp "target/release/${APP_NAME}" "${MACOS_DIR}/${APP_NAME}"

# Create Info.plist
echo "Creating Info.plist..."
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
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
</dict>
</plist>
EOF

# Copy app icon if it exists
if [ -f "assets/icons/AppIcon.icns" ]; then
    echo "Adding app icon..."
    cp "assets/icons/AppIcon.icns" "${RESOURCES_DIR}/AppIcon.icns"
elif [ -f "assets/icons/AppIcon.png" ]; then
    echo "Adding app icon (PNG format)..."
    cp "assets/icons/AppIcon.png" "${RESOURCES_DIR}/AppIcon.png"
fi

# Make the app bundle executable
chmod +x "${MACOS_DIR}/${APP_NAME}"

echo "App bundle created: ${APP_BUNDLE}"
echo "You can now distribute ${APP_BUNDLE} to other macOS users" 