#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== FFmpeg Library Bundler ===${NC}"

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo -e "${RED}Error: Homebrew is not installed. Please install Homebrew first.${NC}"
    exit 1
fi

# Check if FFmpeg is installed via Homebrew
if ! brew list ffmpeg &> /dev/null; then
    echo -e "${YELLOW}FFmpeg not found via Homebrew. Installing...${NC}"
    brew install ffmpeg
fi

# Get FFmpeg installation path - try multiple locations
FFMPEG_PATH=""

# First try Homebrew
if brew list ffmpeg &> /dev/null; then
    FFMPEG_PATH=$(brew --prefix ffmpeg)
    echo -e "${BLUE}FFmpeg found via Homebrew at: ${FFMPEG_PATH}${NC}"
# Then try Applications folder
elif [ -d "/Applications/FFmpeg.app" ]; then
    FFMPEG_PATH="/Applications/FFmpeg.app/Contents/Resources"
    echo -e "${BLUE}FFmpeg found in Applications at: ${FFMPEG_PATH}${NC}"
elif [ -f "/Applications/ffmpeg" ]; then
    FFMPEG_PATH="/Applications"
    echo -e "${BLUE}FFmpeg found directly in Applications at: ${FFMPEG_PATH}${NC}"
else
    echo -e "${RED}Error: FFmpeg not found in any expected location${NC}"
    exit 1
fi

# Create lib directory in app bundle
APP_NAME="MetadataGenerator"
APP_BUNDLE="${APP_NAME}.app"
LIB_DIR="${APP_BUNDLE}/Contents/MacOS/lib"

echo -e "${YELLOW}Creating lib directory...${NC}"
mkdir -p "${LIB_DIR}"

# Copy FFmpeg libraries
echo -e "${YELLOW}Copying FFmpeg libraries...${NC}"
FFMPEG_LIBS=(
    "libavcodec.dylib"
    "libavdevice.dylib"
    "libavfilter.dylib"
    "libavformat.dylib"
    "libavutil.dylib"
    "libpostproc.dylib"
    "libswresample.dylib"
    "libswscale.dylib"
)

for lib in "${FFMPEG_LIBS[@]}"; do
    if [ -f "${FFMPEG_PATH}/lib/${lib}" ]; then
        echo -e "${GREEN}Copying ${lib}...${NC}"
        cp "${FFMPEG_PATH}/lib/${lib}" "${LIB_DIR}/"
    else
        echo -e "${YELLOW}Warning: ${lib} not found${NC}"
    fi
done

# Update library paths in the executable
echo -e "${YELLOW}Updating library paths in executable...${NC}"
EXECUTABLE="${APP_BUNDLE}/Contents/MacOS/${APP_NAME}"

if [ -f "${EXECUTABLE}" ]; then
    # Get the app bundle path for relative references
    APP_BUNDLE_PATH="@executable_path"
    
    # Update each library path
    for lib in "${FFMPEG_LIBS[@]}"; do
        if [ -f "${LIB_DIR}/${lib}" ]; then
            echo -e "${GREEN}Updating ${lib} path...${NC}"
            install_name_tool -change "${FFMPEG_PATH}/lib/${lib}" "${APP_BUNDLE_PATH}/lib/${lib}" "${EXECUTABLE}"
        fi
    done
    
    echo -e "${GREEN}✓ Library paths updated successfully${NC}"
else
    echo -e "${RED}Error: Executable not found at ${EXECUTABLE}${NC}"
    echo -e "${YELLOW}Make sure to run this script after building the app bundle${NC}"
    exit 1
fi

echo -e "${GREEN}=== FFmpeg bundling complete! ===${NC}"
echo -e "${BLUE}Your app now includes all necessary FFmpeg libraries${NC}" 