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

# Get FFmpeg installation path
FFMPEG_PATH=$(brew --prefix ffmpeg)
echo -e "${BLUE}FFmpeg path: ${FFMPEG_PATH}${NC}"

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