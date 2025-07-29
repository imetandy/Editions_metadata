#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== FFmpeg Dependencies Installer ===${NC}"

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo -e "${RED}Error: Homebrew is not installed.${NC}"
    echo -e "${YELLOW}Please install Homebrew first by running:${NC}"
    echo -e "${BLUE}/bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Homebrew is installed${NC}"

# Check if FFmpeg is already installed
if brew list ffmpeg &> /dev/null; then
    echo -e "${GREEN}✓ FFmpeg is already installed${NC}"
    echo -e "${BLUE}FFmpeg version: $(ffmpeg -version | head -n1)${NC}"
else
    echo -e "${YELLOW}Installing FFmpeg...${NC}"
    brew install ffmpeg
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ FFmpeg installed successfully${NC}"
    else
        echo -e "${RED}✗ Failed to install FFmpeg${NC}"
        exit 1
    fi
fi

# Verify installation
echo -e "${YELLOW}Verifying installation...${NC}"
if ffmpeg -version &> /dev/null; then
    echo -e "${GREEN}✓ FFmpeg is working correctly${NC}"
else
    echo -e "${RED}✗ FFmpeg verification failed${NC}"
    exit 1
fi

echo -e "${GREEN}=== Installation Complete! ===${NC}"
echo -e "${BLUE}You can now run MetadataGenerator without issues.${NC}"
echo -e "${YELLOW}If you still have problems, try restarting your terminal.${NC}" 