#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== MetadataGenerator App Diagnostic Tool ===${NC}"

# Check if app exists
APP_PATH="/Applications/MetadataGenerator.app"
if [ ! -d "$APP_PATH" ]; then
    echo -e "${RED}Error: MetadataGenerator.app not found in /Applications${NC}"
    echo -e "${YELLOW}Please install the app first by dragging it to Applications${NC}"
    exit 1
fi

echo -e "${GREEN}✓ App found at: $APP_PATH${NC}"

# Check executable
EXECUTABLE="$APP_PATH/Contents/MacOS/MetadataGenerator"
if [ ! -f "$EXECUTABLE" ]; then
    echo -e "${RED}Error: Executable not found at $EXECUTABLE${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Executable found${NC}"

# Check if executable is actually executable
if [ ! -x "$EXECUTABLE" ]; then
    echo -e "${YELLOW}Warning: Executable doesn't have execute permissions${NC}"
    echo -e "${YELLOW}Fixing permissions...${NC}"
    chmod +x "$EXECUTABLE"
fi

# Check macOS version
MACOS_VERSION=$(sw_vers -productVersion)
echo -e "${BLUE}macOS Version: $MACOS_VERSION${NC}"

# Check if macOS version is supported (10.15+)
if [[ "$MACOS_VERSION" < "10.15" ]]; then
    echo -e "${RED}Error: macOS 10.15 (Catalina) or later is required${NC}"
    exit 1
fi

echo -e "${GREEN}✓ macOS version is supported${NC}"

# Check architecture
ARCH=$(uname -m)
echo -e "${BLUE}Architecture: $ARCH${NC}"

# Check if app is universal or matches architecture
APP_ARCH=$(file "$EXECUTABLE" | grep -o "x86_64\|arm64" | head -1)
if [ -z "$APP_ARCH" ]; then
    echo -e "${YELLOW}Warning: Could not determine app architecture${NC}"
else
    echo -e "${BLUE}App Architecture: $APP_ARCH${NC}"
    if [ "$ARCH" = "arm64" ] && [ "$APP_ARCH" = "x86_64" ]; then
        echo -e "${YELLOW}Warning: Running ARM64 Mac with x86_64 app (should work with Rosetta)${NC}"
    elif [ "$ARCH" = "x86_64" ] && [ "$APP_ARCH" = "arm64" ]; then
        echo -e "${RED}Error: Running x86_64 Mac with ARM64 app (not compatible)${NC}"
        exit 1
    fi
fi

# Check for FFmpeg libraries
echo -e "${BLUE}Checking FFmpeg dependencies...${NC}"

# Check if FFmpeg is installed via Homebrew
if command -v brew &> /dev/null; then
    if brew list ffmpeg &> /dev/null; then
        echo -e "${GREEN}✓ FFmpeg is installed via Homebrew${NC}"
        FFMPEG_PATH=$(brew --prefix ffmpeg)
        echo -e "${BLUE}FFmpeg path: $FFMPEG_PATH${NC}"
    else
        echo -e "${YELLOW}FFmpeg not installed via Homebrew${NC}"
    fi
else
    echo -e "${YELLOW}Homebrew not installed${NC}"
fi

# Check if FFmpeg is installed in Applications
if [ -d "/Applications/FFmpeg.app" ]; then
    echo -e "${GREEN}✓ FFmpeg found in Applications folder${NC}"
    FFMPEG_APP_PATH="/Applications/FFmpeg.app"
    FFMPEG_BIN_PATH="$FFMPEG_APP_PATH/Contents/Resources/bin"
    if [ -d "$FFMPEG_BIN_PATH" ]; then
        echo -e "${BLUE}FFmpeg binaries path: $FFMPEG_BIN_PATH${NC}"
        # Check if ffmpeg executable exists
        if [ -f "$FFMPEG_BIN_PATH/ffmpeg" ]; then
            echo -e "${GREEN}✓ FFmpeg executable found${NC}"
        else
            echo -e "${YELLOW}Warning: FFmpeg executable not found in expected location${NC}"
        fi
    fi
elif [ -f "/Applications/ffmpeg" ]; then
    echo -e "${GREEN}✓ FFmpeg executable found directly in Applications${NC}"
else
    echo -e "${YELLOW}FFmpeg not found in Applications folder${NC}"
fi

# Check if ffmpeg command is available in PATH
if command -v ffmpeg &> /dev/null; then
    echo -e "${GREEN}✓ FFmpeg is available in PATH${NC}"
    FFMPEG_VERSION=$(ffmpeg -version | head -n1)
    echo -e "${BLUE}FFmpeg version: $FFMPEG_VERSION${NC}"
else
    echo -e "${YELLOW}FFmpeg not found in PATH${NC}"
fi

# Check for bundled libraries
LIB_DIR="$APP_PATH/Contents/MacOS/lib"
if [ -d "$LIB_DIR" ]; then
    echo -e "${GREEN}✓ App has bundled libraries${NC}"
    echo -e "${BLUE}Bundled libraries:${NC}"
    ls -la "$LIB_DIR" | grep "\.dylib"
else
    echo -e "${YELLOW}No bundled libraries found${NC}"
fi

# Check library dependencies
echo -e "${BLUE}Checking library dependencies...${NC}"
otool -L "$EXECUTABLE" | grep -E "(libav|ffmpeg)" || echo -e "${YELLOW}No FFmpeg dependencies found in executable${NC}"

# Check for missing libraries
echo -e "${BLUE}Checking for missing libraries...${NC}"
MISSING_LIBS=$(otool -L "$EXECUTABLE" | grep "not found" || true)
if [ -n "$MISSING_LIBS" ]; then
    echo -e "${RED}Missing libraries:${NC}"
    echo "$MISSING_LIBS"
else
    echo -e "${GREEN}✓ No missing libraries detected${NC}"
fi

# Try to run the app and capture any output
echo -e "${BLUE}Attempting to run the app...${NC}"
echo -e "${YELLOW}This will try to launch the app and show any error messages:${NC}"

# Run the app in the background and capture output
"$EXECUTABLE" 2>&1 &
APP_PID=$!

# Wait a moment for the app to start or fail
sleep 3

# Check if the process is still running
if kill -0 $APP_PID 2>/dev/null; then
    echo -e "${GREEN}✓ App appears to be running (PID: $APP_PID)${NC}"
    echo -e "${YELLOW}If you don't see the GUI, check if it's behind other windows${NC}"
    
    # Kill the process
    kill $APP_PID 2>/dev/null || true
else
    echo -e "${RED}✗ App failed to start or crashed immediately${NC}"
    echo -e "${YELLOW}This usually indicates a missing dependency or permission issue${NC}"
fi

# Check quarantine attributes
echo -e "${BLUE}Checking quarantine attributes...${NC}"
if xattr -l "$APP_PATH" 2>/dev/null | grep -q "com.apple.quarantine"; then
    echo -e "${YELLOW}App has quarantine attribute (may cause issues)${NC}"
    echo -e "${YELLOW}To remove: xattr -cr $APP_PATH${NC}"
else
    echo -e "${GREEN}✓ No quarantine attributes${NC}"
fi

# Check permissions
echo -e "${BLUE}Checking permissions...${NC}"
ls -la "$APP_PATH/Contents/MacOS/"

# Summary and recommendations
echo -e "${BLUE}=== Summary ===${NC}"
echo -e "${YELLOW}If the app still doesn't work:${NC}"
echo -e "1. Try running from Terminal: $EXECUTABLE"
echo -e "2. Install FFmpeg: brew install ffmpeg"
echo -e "3. Check Console.app for crash logs"
echo -e "4. Try right-clicking the app and selecting 'Open'"
echo -e "5. Check System Preferences → Security & Privacy"

echo -e "${GREEN}=== Diagnostic complete ===${NC}" 